use crate::constants::TWO_PI;

pub struct SVF {
    integrator_z: Vec<f32>,
    alpha0: f32,
    alpha: f32,
    rho: f32,
    memo: Memo,
}

struct Memo {
    fc: f32,
    q: f32,
    sample_rate: u32,
}

impl SVF {
    pub fn new(sr: u32) -> SVF {
        let mut svf = SVF {
            integrator_z: vec![0.0; 2],
            alpha0: 0.0,
            alpha: 0.0,
            rho: 0.0,
            memo: Memo {
                fc: 500.0,
                q: 20.0,
                sample_rate: sr,
            },
        };
        svf.calculate_coeffs();
        svf
    }
    pub fn process_sample(&mut self, input: f32, fc: f32, q: f32, sample_rate: u32) -> f32 {
        if fc != self.memo.fc || q != self.memo.q || sample_rate != self.memo.sample_rate {
            self.memo.fc = fc;
            self.memo.q = q;
            self.memo.sample_rate = sample_rate;
            self.calculate_coeffs();
        }
        // Calculate filter outputs
        let hpf = self.alpha0 * (input - self.rho * self.integrator_z[0] - self.integrator_z[1]);
        let bpf = self.alpha * hpf + self.integrator_z[0];
        let lpf = self.alpha * bpf + self.integrator_z[1];
        //let bsf = hpf + lpf;

        // Update state registers
        self.integrator_z[0] = self.alpha * hpf + bpf;
        self.integrator_z[1] = self.alpha * bpf + lpf;

        bpf
    }
    fn calculate_coeffs(&mut self) {
        let wd = TWO_PI * self.memo.fc;
        let ts = 1.0 / self.memo.sample_rate as f32;
        let angle = (2.0 / ts) * (wd * ts / 2.0).tan();
        let g = angle * ts / 2.0;
        let r = 1.0 / (2.0 * self.memo.q);

        self.alpha0 = 1.0 / (1.0 + 2.0 * r * g + g * g);
        self.alpha = g;
        self.rho = 2.0 * r + g;
    }
}
