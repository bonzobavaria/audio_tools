// An interpolation policy removes the burden of doing interpolation from readers.
pub fn interpolate_linear(table: &Vec<f32>, phase: f32) -> f32 {
    // Expanded phase, from normal value to table length
    let ex_phase = phase * table.len() as f32;
    let index = ex_phase as usize;
    let fraction = ex_phase - index as f32;
    let mut next_index = index + 1;
    if next_index >= table.len() {
        next_index = 0;
    }
    table[index] * (1.0 - fraction) + table[next_index] * fraction
}

pub fn envelope_linear(table: &Vec<f32>, phase: f32) -> f32 {
    // Envolopes are not circular tables, so in this interpolation policy, we
    // don't cycle around to the beginning of the table once we reach the end.
    let ex_phase = phase * table.len() as f32;
    let index = ex_phase as usize;
    let fraction = ex_phase - index as f32;
    let next_index = index + 1;
    if next_index >= table.len() {
        table[index]
    } else {
        table[index] * (1.0 - fraction) + table[next_index] * fraction
    }
}

pub fn simple_linear(sample1: f32, sample2: f32, fraction: f32) -> f32 {
    sample1 * (1.0 - fraction) + sample2 * fraction
}
