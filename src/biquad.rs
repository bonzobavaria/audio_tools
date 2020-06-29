use std::f32::consts::PI;

pub struct Biquad {
    coeffs: Vec<f32>,
    buffer: Vec<f32>,
    memo: Memo,
}

struct Memo {
    sample_rate: u32,
    freq: f32,
    q: f32,
}

enum Coeffs {
    A0,
    A1,
    A2,
    B1,
    B2,
}

enum Buffer {
    XZ1,
    XZ2,
    YZ1,
    YZ2,
}

impl Biquad {
    // Implements the direct form biquad filter from Pirkle 2019, p.252 & p.270
    pub fn new(sr: u32) -> Biquad {
        Biquad {
            buffer: vec![0.0; 4],
            coeffs: vec![0.0; 5],
            memo: Memo {
                sample_rate: sr,
                freq: 500.0,
                q: 100.0,
            },
        }
    }
    pub fn tick(&mut self, input: f32, freq: f32, q: f32, sample_rate: u32) -> f32 {
        if freq != self.memo.freq || q != self.memo.q || sample_rate != self.memo.sample_rate {
            self.calculate_coeffs(freq, q, sample_rate);
            self.memo.freq = freq;
            self.memo.q = q;
            self.memo.sample_rate = sample_rate;
        };
        let output: f32 = self.coeffs[Coeffs::A0 as usize] * input
            + self.coeffs[Coeffs::A1 as usize] * self.buffer[Buffer::XZ1 as usize]
            + self.coeffs[Coeffs::A2 as usize] * self.buffer[Buffer::XZ2 as usize]
            - self.coeffs[Coeffs::B1 as usize] * self.buffer[Buffer::YZ1 as usize]
            - self.coeffs[Coeffs::A2 as usize] * self.buffer[Buffer::YZ2 as usize];
        output
    }
    pub fn calculate_coeffs(&mut self, freq: f32, q: f32, sample_rate: u32) {
        let theta_c = 2.0 * PI * freq / sample_rate as f32;
        let d = 1.0 / q;
        let beta_numerator = 1.0 - ((d / 2.0) * theta_c.sin());
        let beta_denominator = 1.0 + ((d / 2.0) * theta_c.sin());
        let beta = 0.5 * (beta_numerator / beta_denominator);
        let gamma = (0.5 + beta) * theta_c.cos();
        let alpha = (0.5 + beta - gamma) / 2.0;

        self.coeffs[Coeffs::A0 as usize] = alpha;
        self.coeffs[Coeffs::A1 as usize] = 2.0 * alpha;
        self.coeffs[Coeffs::A2 as usize] = alpha;
        self.coeffs[Coeffs::B1 as usize] = -2.0 * gamma;
        self.coeffs[Coeffs::B2 as usize] = 2.0 * beta;
    }
}
