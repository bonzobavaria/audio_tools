#[derive(Clone)]
enum Stage {
    Attack,
    Release,
}

#[derive(Clone)]
pub struct EnvReader {
    pub active: bool,
    attack_seconds: f32,
    release_seconds: f32,
    pub attack_wt_increment: f32,
    pub release_wt_increment: f32,
    pub sample_rate: u32,
    current_stage: Stage,
    table_size: usize,
    wt_position: f32,
}

// seconds to samples or wavetable increment amount is just the inverse of
// frequency.
fn seconds_to_wt_inc(seconds: f32, table_size: usize, sample_rate: u32) -> f32 {
    table_size as f32 / sample_rate as f32 * (1.0 / seconds)
}

impl EnvReader {
    pub fn new(
        table_size: usize, 
        sample_rate: u32,
    ) -> EnvReader {
        EnvReader {
            active: false,
            attack_seconds: 0.0,
            release_seconds: 0.0,
            sample_rate,
            table_size,
            current_stage: Stage::Attack,
            attack_wt_increment: 0.0,
            release_wt_increment: 0.0,
            wt_position: 0.0,
        }
    }
    pub fn read(&self, table: &Vec<f32>) -> f32 {
        table[self.wt_position as usize]
    }
    pub fn start(&mut self) {
        self.active = true;
        self.current_stage = Stage::Attack;
    }
    pub fn increment(&mut self) {
        match &self.current_stage {
            Stage::Attack => {
                self.wt_position += self.attack_wt_increment;
                if self.wt_position >= self.table_size as f32 {
                    self.wt_position -= self.attack_wt_increment;
                    self.current_stage = Stage::Release;
                }
            }
            Stage::Release => {
                self.wt_position -= self.release_wt_increment;
                if self.wt_position <= 0.0 {
                    self.wt_position = 0.0;
                    self.active = false;
                }
                
            }
        }
    }
    pub fn set_attack(&mut self, next_attack: f32) {
        self.attack_seconds = next_attack;
        self.attack_wt_increment = seconds_to_wt_inc(
            self.attack_seconds, 
            self.table_size, 
            self.sample_rate,
        )
    }
    pub fn set_release(&mut self, next_release: f32) {
        self.release_seconds = next_release;
        self.release_wt_increment = seconds_to_wt_inc(
            self.release_seconds, 
            self.table_size, 
            self.sample_rate,
        )
    }
    pub fn update_sample_rate(&mut self, next_sample_rate: u32) {
        println!("Sample rate updated!");
        self.sample_rate = next_sample_rate;
        //self.wt_increment = seconds_to_wt_inc(
            //self.length_seconds, 
            //self.table_size, 
            //self.sample_rate
        //);
    }
}
