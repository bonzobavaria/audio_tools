// A circular buffer is a wrapper around vec that only supports writing into the
// buffer at the next index (starting over at index 0 if the write index would
// be out of bounds. This data structure is very useful in audio DSP for
// creating delay and filter effects.
pub struct CircularBuffer {
    buffer: Vec<f32>,
    write_index: usize,
}

impl CircularBuffer {
    pub fn new(buffer_size: usize) -> CircularBuffer {
        CircularBuffer {
            buffer: vec![0.0; buffer_size],
            write_index: 0,
        }
    }
    pub fn write(&mut self, value: f32) {
        self.buffer[self.write_index] = value;
        self.write_index += 1;
        if self.write_index == self.buffer.len() {
            self.write_index = 0;
        }
    }
    // Callers read from the circular buffer at a specified distance from the
    // write index, i.e., samples that were inserted N write operations ago,
    // where N is length_samples. Conversion from units such as seconds to
    // samples, or interpolation between multiple read values are higher-level
    // concerns, handled by callers.
    pub fn read(&self, length_samples: usize) -> f32 {
        if length_samples > self.buffer.len() {
            panic!("Requested delay length is greater than buffer size!");
        }
        // usize::min_value() == 0, so we can't subtract two of them and think
        // about whether the result is negative. We convert our usizes to i32
        // here to handle this.
        let mut read_index = self.write_index as i32 - length_samples as i32;
        if read_index < 0 {
            read_index += self.buffer.len() as i32;
        }

        self.buffer[read_index as usize]
    }
}

// Interpolation

pub fn discard(buf: CircularBuffer, length_samples: f32) -> f32 {
    buf.read(length_samples as usize)
}

pub fn linear_interpolate(buf: &CircularBuffer, length_samples: &f32) -> f32 {
    let index = *length_samples as usize;
    let sample1 = buf.read(index);
    let sample2 = buf.read(index + 1);
    let fraction = length_samples - index as f32;
    sample1 * (1.0 - fraction) + sample2 * fraction
}

// Note that unlike read-only signal generators, effects with an internal
// CircularBuffer do manage their internal buffer, instead of reading from a
// referenced wavetable, and they also combine incrementing and reading
// operations. The internal buffer is managed because access to the write index
// is necessary for reading, and it's not useful to have multiple actors writing
// into a CircularBuffer. Reading and incrementing (i.e. writing) actions are
// combined becuase the input value to the write operation depends on the
// previous write operation whenever feedback designs are used.
pub struct SimpleDelay {
    buffer: CircularBuffer,
    memo: Memo,
}

struct Memo {
    delay_samples: f32,
    delay_seconds: f32,
    sample_rate: u32,
}

impl Memo {
    pub fn new() -> Memo {
        Memo {
            delay_samples: 22050.0, 
            delay_seconds: 0.5,
            sample_rate: 44100,
        }
    }
}

// Parameters are provided as inputs to the delay. In general this provides
// room to modulate or calculate parameters per tick as needed, without needing
// accessors. Callers can provide parameter management structs if needed.
impl SimpleDelay {
    pub fn new(buffer_size: usize) -> SimpleDelay {
        SimpleDelay {
            buffer: CircularBuffer::new(buffer_size),
            memo: Memo::new(),
        }
    }
    pub fn tick(
        &mut self, 
        input_sample: f32, 
        delay_seconds: f32, 
        feedback_amount: f32,
        sample_rate: u32,
        ) -> f32 {
        self.update_memo(delay_seconds, sample_rate);
        // TODO: defer to external interpolation policy somehow.
        let output = linear_interpolate(&self.buffer, &self.memo.delay_samples);
        self.buffer.write(input_sample + (output * feedback_amount));
        output
    }
    fn update_memo(&mut self, delay_seconds: f32, sample_rate: u32) {
        if sample_rate != self.memo.sample_rate 
            || delay_seconds != self.memo.delay_seconds {
                self.memo.sample_rate = sample_rate;
                self.memo.delay_seconds = delay_seconds;
                self.memo.delay_samples = 
                    self.memo.sample_rate as f32 * self.memo.delay_seconds;
        }
    }
}

//pub struct DelayTap(pub f32, pub f32);

//pub struct UberDelay {
//input_buffer: CircularBuffer,
//capture_buffer: CircularBuffer,
//}

//impl UberDelay {
//pub fn new(buffer_size: usize) -> UberDelay {
//UberDelay {
//input_buffer: CircularBuffer::new(buffer_size),
//capture_buffer: CircularBuffer::new(buffer_size),
//}
//}
//// TODO: we need to recieve lots of params here and generate the multitap output
//// TODO: Create functions to make delay taps.
//pub fn tick(input_sample: f32, delay_taps: Vec<DelayTap>, feedback_amount: f32, pattern_length:f32) -> f32 {
//1.0
//}
//}
