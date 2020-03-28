// Reader for an oscillator wavetable. This data type does not manage any vector
// it reads from. It only tracks the table position to read from, and takes a
// reference to a vector to read from as an argument to the read function.

#[derive(Clone)]
pub struct OscReader {
    pub active: bool,
    pub wt_increment: f32,
    pub wt_position: f32,
    pub frequency: f32,
    pub sample_rate: u32,
    table_size: usize,
}

impl OscReader {
    pub fn new(
        table_size: usize, 
        sample_rate: u32,
    ) -> OscReader {
        OscReader {
            active: false,
            frequency: 0.0,
            sample_rate,
            table_size,
            wt_increment: 0.0,
            wt_position: 0.0,
        }
    }
    pub fn read(&self, table: &Vec<f32>) -> f32 {
        table[self.wt_position as usize]
    }
    pub fn increment(&mut self) {
        self.wt_position += self.wt_increment;
        self.wt_position %= self.table_size as f32;
    }
    // TODO: Place reponsibiliy for calculating this on the caller directly or
    // indirectly by using a wrapper with memoization capabilities. The reader
    // itself doesn't need to care about frequency.
    pub fn update_frequency(&mut self, next_frequency: f32) {
        self.frequency = next_frequency;
        self.wt_increment = 
            freq_to_wt_inc(self.frequency, self.table_size, self.sample_rate);
    }
}

fn freq_to_phase_inc(freq: f32, sample_rate: u32) -> f32 {
    1.0 / sample_rate as f32 * freq
}

// The OscReader doesn't track phase as a normalized value between 0 and 1. For
// computational efficiency is tracks the actual wavetable index. This means that
// the data type has to keep track on table size and stuff.
fn freq_to_wt_inc(freq: f32, table_size: usize, sample_rate: u32) -> f32 {
    table_size as f32 / sample_rate as f32 * freq
}

