#[derive(Clone)]
enum Stage {
    Attack,
    Release,
}

#[derive(Clone)]
pub struct EnvReader {
    pub is_active: bool,
    current_stage: Stage,
    memo: Memo,
    phase: f32,
}

#[derive(Clone)]
struct Memo {
    sample_rate: u32,
    attack_seconds: f32,
    release_seconds: f32,
    attack_phase_inc: f32,
    release_phase_inc: f32,
}

impl EnvReader {
    pub fn new() -> EnvReader {
        EnvReader {
            is_active: false,
            current_stage: Stage::Attack,
            phase: 0.0,
            memo: Memo { 
                attack_seconds: 0.01,
                release_seconds: 0.5,
                attack_phase_inc: 1.0 / (44100.0 * 0.01),
                release_phase_inc: 1.0 / (44100.0 * 0.5),
                sample_rate: 44100, 
            },
        }
    }
    pub fn read(&self, table: &Vec<f32>) -> f32 {
        table[(self.phase * table.len() as f32) as usize]
    }
    pub fn start(&mut self) {
        self.is_active = true;
        self.current_stage = Stage::Attack;
    }
    pub fn increment(&mut self, attack: f32, release: f32, sample_rate: u32) {
        self.update_memo(attack, release, sample_rate);
        match &self.current_stage {
            Stage::Attack => {
                self.phase += self.memo.attack_phase_inc;
                if self.phase >= 1.0 { 
                    // Undo the increment if it's time to switch phase.
                    self.phase -= self.memo.attack_phase_inc;
                    self.current_stage = Stage::Release;
                }
            }
            Stage::Release => {
                self.phase -= self.memo.release_phase_inc;
                if self.phase <= 0.0 {
                    self.phase = 0.0;
                    self.is_active = false;
                }
            }
        }
    }
    fn update_memo(&mut self, attack: f32, release: f32, sample_rate: u32) {
        if sample_rate != self.memo.sample_rate {
            self.memo.sample_rate = sample_rate;
            self.memo.attack_phase_inc = 
                1.0 / (self.memo.sample_rate as f32 * self.memo.attack_seconds);
            self.memo.release_phase_inc = 
                1.0 / (self.memo.sample_rate as f32 * self.memo.release_seconds);
        }
        if attack != self.memo.attack_seconds || release != self.memo.release_seconds {
            self.memo.attack_seconds = attack;
            self.memo.release_seconds = release;
            self.memo.attack_phase_inc = 
                1.0 / (self.memo.sample_rate as f32 * self.memo.attack_seconds);
            self.memo.release_phase_inc = 
                1.0 / (self.memo.sample_rate as f32 * self.memo.release_seconds);
        }
    }
}
