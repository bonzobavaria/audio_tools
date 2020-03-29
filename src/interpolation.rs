pub fn interpolate_linear(input1: f32, input2: f32, fraction: f32) -> f32 {
    input1 * (1.0 - fraction)
    + input2 * fraction
}
