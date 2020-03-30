use std::f32::consts::E;

use crate::delay::{SimpleDelay};
use crate::envelope::{EnvReader};
use crate::osc::{OscReader};
use crate::wavetable;
use crate::midi;
use crate::interpolation;

pub enum OscType {
    Sawtooth,
    Sine,
    Square,
    Triangle,
}

#[derive(Clone)]
struct NoteInfo {
    frequency: f32,
    velocity: f32,
}

impl NoteInfo {
    pub fn new(freq: f32, vel: f32) -> NoteInfo {
        NoteInfo { frequency: freq, velocity: vel }
    }
}

pub enum Message {
    SetVolume(f32),
    SetOscillator(OscType),
    NoteOn(u8, u8),
    SetEnvAttack(f32),
    SetEnvRelease(f32),
}

pub struct BasicSynth {
    delay: SimpleDelay,
    delay_feedback_amount: f32,
    delay_length_samples: f32,
    delay_wetdry: f32,
    envelope_reader: Vec<EnvReader>,
    envelope_attack: f32,
    envelope_release: f32,
    envelope_table: Vec<f32>,
    midi_table: Vec<f32>,
    table_reader: Vec<OscReader>,
    volume: f32,
    voice_info: Vec<NoteInfo>,
    voice_output: f32,
    wavetable: Vec<Vec<f32>>,
    wavetable_index: usize,
}

impl BasicSynth {
    pub fn new() -> BasicSynth {
        BasicSynth {
            delay: SimpleDelay::new((44100 * 2) as usize),
            // TODO: Let SimpleDelay convert seconds to samples
            delay_length_samples: (44100 as f32 * 0.25),
            delay_feedback_amount: 0.7,
            delay_wetdry: 0.5,
            envelope_attack: 0.01,
            envelope_reader: vec![EnvReader::new(); 128],
            envelope_release: 0.5,
            envelope_table: wavetable::make_exp_envelope(1024, E),
            midi_table: midi::make_midi_freq_table(),
            table_reader: vec![OscReader::new(); 128],
            volume: 0.5,
            voice_info: vec![NoteInfo::new(0.0, 0.0); 128],
            voice_output: 0.0,
            wavetable: wavetable::make_sin_saw_table(1024, 24),
            wavetable_index: 0,
        }
    }
    pub fn send(&mut self, message: Message) {
        match message {
            // TODO: Only 0.0 - 1.0 are acceptable inputs. Make it impossible to
            // respesent unwanted inputs.
            Message::SetVolume(value) => {
                self.volume = f32::powf(value, 2.0);
            }
            Message::NoteOn(note, velocity) => {
                let norm_velocity: f32 = velocity as f32 / 127.0;
                let n = NoteInfo::new(self.midi_table[note as usize], norm_velocity);
                self.voice_info[note as usize] = n;
                self.envelope_reader[note as usize].start();
            }
            Message::SetOscillator(osctype) => {
                match osctype {
                    // TODO: Implement triangle wave
                    OscType::Sine => { self.wavetable_index = 0; }
                    OscType::Triangle => {self.wavetable_index = 1; }
                    OscType::Square => { self.wavetable_index = 2; }
                    OscType::Sawtooth => { self.wavetable_index = 3; }
                }
            }
            Message::SetEnvAttack(value) => {
                self.envelope_attack = value;
            }
            Message::SetEnvRelease(value) => {
                self.envelope_release = value;
            }
        }
    }
    pub fn tick(&mut self, sample_rate: u32) -> f32 {
        self.voice_output = 0.0;
        // Increment readers if they have an active envelope. There is still
        // time to tweak the phase after this for FM or sync effects.
        for i in 0..self.table_reader.len() {
            if self.envelope_reader[i].is_active {
                let freq = self.voice_info[i].frequency;
                // TODO: Pass referenc to number instead of copy
                self.table_reader[i].increment(freq, sample_rate);
                self.envelope_reader[i].increment(
                    self.envelope_attack,
                    self.envelope_release,
                    sample_rate,
                );
            }
        }
        // Read from signal generators and add to the voice output.
        for i in 0..self.table_reader.len() {
            if self.envelope_reader[i].is_active {
                self.voice_output +=
                    self.table_reader[i]
                        .read(
                            &self.wavetable[self.wavetable_index],
                            interpolation::interpolate_linear,
                            ) 
                        * self.envelope_reader[i].read(
                            &self.envelope_table,
                            interpolation::envelope_linear,
                        )
                        * self.voice_info[i].velocity;
            }
        }

        let delay_output = self.delay.tick(
            self.voice_output, 
            self.delay_length_samples,
            self.delay_feedback_amount,
        );

        (self.voice_output * (1.0 - self.delay_wetdry)
        + (delay_output * self.delay_wetdry))
        * self.volume
    }
}
