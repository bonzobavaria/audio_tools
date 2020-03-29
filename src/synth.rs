use crate::delay::{SimpleDelay};
use crate::reader::{EnvReader};
use crate::osc::{OscReader};
use crate::wavetable;
use crate::midi;

const TABLE_SIZE: usize = 1024;
// TODO: Don't use sample rate during initialization
const SAMPLE_RATE: u32 = 44100;

pub enum OscType {
    Sawtooth,
    Sine,
    Square,
    Triangle,
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
    envelope_reader: Vec<EnvReader>,
    midi_table: Vec<f32>,
    table_reader: Vec<OscReader>,
    volume: f32,
    // Right now a voice info is just an f32 for the frequency of the voice.
    // This could be expanded to contain more info.
    voice_info: Vec<f32>,
    voice_output: Vec<f32>,
    wavetable: Vec<Vec<f32>>,
    wavetable_index: usize,
}

impl BasicSynth {
    pub fn new() -> BasicSynth {
        // Initialise the state that we want to live on the audio thread.
        let reader1 = OscReader::new();
        // TODO: Try to remove the need for setting params here, and not use sr.
        let mut env_reader1 = EnvReader::new(TABLE_SIZE, SAMPLE_RATE);
        env_reader1.set_attack(0.01);
        env_reader1.set_release(0.5);
        // TODO: Create a sine, tri, square, saw wavetable export from wavetable.
        let sine_table = wavetable::make_sine_table(TABLE_SIZE);
        let square_table = wavetable::make_square_table(TABLE_SIZE);
        let saw_table = wavetable::make_sawtooth_table(TABLE_SIZE);
        let exp_envelope_table = wavetable::make_exp_envelope(TABLE_SIZE);
        BasicSynth {
            delay: SimpleDelay::new((SAMPLE_RATE * 2) as usize),
            delay_length_samples: (SAMPLE_RATE as f32 * 0.25),
            delay_feedback_amount: 0.7,
            envelope_reader: vec![env_reader1; 128],
            midi_table: midi::make_midi_freq_table(),
            table_reader: vec![reader1; 128],
            volume: 0.5,
            voice_info: vec![0.0; 128],
            voice_output: vec![0.0; 128],
            // TODO: Don't store the envelope table with the oscs.
            wavetable: vec![sine_table, square_table, saw_table, exp_envelope_table],
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
                // TODO: use velocity for notes by setting note amplitude, or doing
                // something with env time.
                self.voice_info[note as usize] = self.midi_table[note as usize];
                self.envelope_reader[note as usize].start();
            }
            Message::SetOscillator(osctype) => {
                match osctype {
                    // TODO: Implement triangle wave
                    OscType::Sine => { self.wavetable_index = 0; }
                    OscType::Square => { self.wavetable_index = 1; }
                    OscType::Sawtooth => { self.wavetable_index = 2; }
                    _ => {} // Triangle not implemented
                }
            }
            Message::SetEnvAttack(value) => {
                for envelope in self.envelope_reader.iter_mut() {
                    envelope.set_attack(value);
                }
            }
            Message::SetEnvRelease(value) => {
                for envelope in self.envelope_reader.iter_mut() {
                    envelope.set_release(value);
                }
            }
        }
    }
    pub fn tick(&mut self, sample_rate: u32) -> f32 {
        for i in 0..self.table_reader.len() {
            // TODO: Change "active" to "is_active"
            if self.envelope_reader[i].active {
                self.table_reader[i].increment(self.voice_info[i], sample_rate);
                self.envelope_reader[i].increment();
            }
        }
        for i in 0..self.table_reader.len() {
            if self.envelope_reader[i].active {
                self.voice_output[i] =
                    self.table_reader[i]
                        .read(&self.wavetable[self.wavetable_index]) *
                    self.envelope_reader[i].read(&self.wavetable[3]);
            }
        }

        let voice_output: f32 = self.voice_output.iter().sum();

        let delay_output = self.delay.tick(
            voice_output, 
            self.delay_length_samples,
            self.delay_feedback_amount,
        );

        let delay_amount = 0.7;

        voice_output + delay_output * delay_amount * self.volume
    }
}
