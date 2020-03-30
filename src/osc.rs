// Reader for an oscillator wavetable. This data type does not manage any vector
// it reads from. It only tracks the table position to read from, and takes a
// reference to a vector to read from as an argument to the read function.

// Note that phase is public. This leaves callers or higher-level modules free
// to perform modulation or sync in any way they see fit.
#[derive(Clone)]
pub struct OscReader {
    pub phase: f32,
    memo: Memo,
}

#[derive(Clone)]
struct Memo {
    frequency: f32,
    sample_rate: u32,
    phase_inc: f32,
}

impl OscReader {
    pub fn new() -> OscReader {
        OscReader {
            phase: 0.0,
            memo: Memo {
                frequency: 0.0,
                sample_rate: 44100,
                phase_inc: 0.0,
            },
        }
    }
    pub fn read<F>(&self, table: &Vec<f32>, interpolate: F) -> f32
    where
        F: Fn(&Vec<f32>, f32) -> f32,
    {
        interpolate(&table, self.phase)
    }
    // TODO: memoize frequency, sample_rate, and calculate phase inc from that.
    pub fn increment(&mut self, freq: f32, sr: u32) {
        if freq != self.memo.frequency || sr != self.memo.sample_rate {
            self.memo.frequency = freq;
            self.memo.sample_rate = sr;
            self.memo.phase_inc = 1.0 / self.memo.sample_rate as f32 * self.memo.frequency
        }
        // While we could store the wavetable index and update it directly
        // instead of using a normalized phase value, that would require us to
        // make assumptions about the size of the wavetable, which we don't
        // manage.
        self.phase += self.memo.phase_inc;
        // NOTE: The use of modulo on the audio thread could be dangerous. Find
        // out and try to minimize this.
        self.phase %= 1.0
    }
}
