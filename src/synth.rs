use std::f32::consts::E;

//use crate::biquad;
use crate::svf;
use crate::delay;
use crate::envelope;
use crate::midi;
use crate::osc;
use crate::wavetable;

pub enum OscType {
    Sine,
    Triangle,
    Square,
    Sawtooth,
}

#[derive(Clone)]
struct NoteInfo {
    frequency: f32,
    velocity: f32,
}

impl NoteInfo {
    pub fn new(freq: f32, vel: f32) -> NoteInfo {
        NoteInfo {
            frequency: freq,
            velocity: vel,
        }
    }
}

pub enum Message {
    SetVolume(f32),
    SetOscillator(OscType),
    NoteOn(u8, u8),
    SetEnvAttack(f32),
    SetEnvRelease(f32),
    SetDelayWetdry(f32),
    SetDelayFeedback(f32),
    SetDelaySeconds(f32),
    SetFilterFreq(f32),
    SetFilterQ(f32),
}

struct UserControl {
    delay_feedback_amount: f32,
    delay_seconds: f32,
    delay_wetdry: f32,
    envelope_attack: f32,
    envelope_release: f32,
    filter_freq: f32,
    filter_q: f32,
    volume: f32,
    wavetable_index: usize,
}

impl UserControl {
    pub fn new() -> UserControl {
        UserControl {
            delay_feedback_amount: 0.7,
            delay_seconds: 0.25,
            delay_wetdry: 0.5,
            envelope_attack: 0.01,
            envelope_release: 0.5,
            filter_freq: 1000.0,
            filter_q: 1.0,
            volume: 0.5,
            wavetable_index: OscType::Sine as usize,
        }
    }
}

pub struct BasicSynth {
    control: UserControl,
    delay: delay::SimpleDelay,
    envelope_reader: Vec<envelope::EnvReader>,
    envelope_table: Vec<f32>,
    filter: svf::SVF,
    midi_table: Vec<f32>,
    table_reader: Vec<osc::OscReader>,
    voice_info: Vec<NoteInfo>,
    voice_output: f32,
    wavetable: Vec<Vec<f32>>,
}

impl BasicSynth {
    pub fn new() -> BasicSynth {
        BasicSynth {
            control: UserControl::new(),
            delay: delay::SimpleDelay::new((44100 * 2) as usize),
            envelope_reader: vec![envelope::EnvReader::new(); 128],
            envelope_table: wavetable::make_exp_envelope(1024, E),
            filter: svf::SVF::new(44100),
            midi_table: midi::make_midi_freq_table(),
            table_reader: vec![osc::OscReader::new(); 128],
            voice_info: vec![NoteInfo::new(0.0, 0.0); 128],
            voice_output: 0.0,
            wavetable: wavetable::make_sin_saw_table(1024, 24),
        }
    }
    pub fn send(&mut self, message: Message) {
        match message {
            // NOTE: Incoming slider range values will always be in range 0.0
            // 1.0.
            Message::NoteOn(note, velocity) => {
                let norm_velocity: f32 = velocity as f32 / 127.0;
                let n = NoteInfo::new(self.midi_table[note as usize], norm_velocity);
                self.voice_info[note as usize] = n;
                self.envelope_reader[note as usize].start();
            }
            Message::SetOscillator(osctype) => {
                self.control.wavetable_index = osctype as usize;
            }
            Message::SetVolume(value) => {
                self.control.volume = f32::powf(value, 2.0);
            }
            Message::SetEnvAttack(value) => {
                self.control.envelope_attack = value;
            }
            Message::SetEnvRelease(value) => {
                self.control.envelope_release = value;
            }
            Message::SetDelayWetdry(value) => {
                self.control.delay_wetdry = value;
            }
            Message::SetDelayFeedback(value) => {
                self.control.delay_feedback_amount = value;
            }
            Message::SetDelaySeconds(value) => {
                self.control.delay_seconds = value;
            }
            Message::SetFilterFreq(value) => {
                self.control.filter_freq = f32::powf(value, 2.0) * 22050.0;
            }
            Message::SetFilterQ(value) => {
                self.control.filter_q = value * 20.0;
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
                self.table_reader[i].increment(freq, sample_rate);
                self.envelope_reader[i].increment(
                    self.control.envelope_attack,
                    self.control.envelope_release,
                    sample_rate,
                );
            }
        }
        // Read from signal generators and add to the voice output.
        for i in 0..self.table_reader.len() {
            if self.envelope_reader[i].is_active {
                self.voice_output += self.table_reader[i].read(
                    &self.wavetable[self.control.wavetable_index],
                    osc::linear_interpolate,
                ) * self.envelope_reader[i]
                    .read(&self.envelope_table, envelope::linear_interpolate)
                    * self.voice_info[i].velocity;
            }
        }

        // Pass output through the filter
        let filter_output = self.filter.process_sample(
            self.voice_output,
            self.control.filter_freq,
            self.control.filter_q,
            sample_rate,
        );

        // TODO: Stop storing voice_output, just build it up in the tick fn
        self.voice_output = filter_output;

        let delay_output = self.delay.tick(
            self.voice_output,
            self.control.delay_seconds,
            self.control.delay_feedback_amount,
            sample_rate,
        );

        // TODO: Create a mixer module to handle wet/dry
        (self.voice_output * (1.0 - self.control.delay_wetdry)
            + (delay_output * self.control.delay_wetdry))
            * self.control.volume
    }
}
