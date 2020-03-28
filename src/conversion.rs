pub fn seconds_to_samples(seconds: f32, sample_rate: u32) -> f32 {
    sample_rate as f32 * seconds
}
