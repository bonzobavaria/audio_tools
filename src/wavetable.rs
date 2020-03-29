use std::f32::consts::E;

use crate::constants::{TWO_PI};

type Wavetable = Vec<f32>;

#[derive(Clone)]
pub struct Partial {
    freq: f32,
    amp: f32,
    phase: f32,
}

pub fn make_exp_envelope(table_size: usize) -> Wavetable {
    let mut wavetable: Vec<f32> = Vec::new();
    let ts: f32 = 1.0 / table_size as f32;
    for i in 0..table_size {
        let sample: f32 = f32::powf(i as f32 * ts, E);
        wavetable.push(sample);
    }
    wavetable.clone()
}

pub fn make_sine_table(table_size: usize) -> Wavetable {
    make_fourier_table_norm(
        table_size,
        vec![Partial {
            freq: 1.0,
            amp: 1.0,
            phase: 0.0,
        }],
    )
}

pub fn make_sawtooth_table(table_size: usize) -> Wavetable {
    make_fourier_table_norm(table_size, make_sawtooth_partials(24))
}

pub fn make_square_table(table_size: usize) -> Wavetable {
    make_fourier_table_norm(table_size, make_square_partials(24))
}

fn make_sawtooth_partials(num_partials: usize) -> Vec<Partial> {
    let mut partials = Vec::new();
    for index in 1..num_partials {
        let partial = Partial {
            freq: index as f32,
            amp: 1.0 / index as f32,
            phase: 0.0,
        };
        partials.push(partial);
    }
    partials.clone()
}

fn make_square_partials(num_partials: usize) -> Vec<Partial> {
    let mut partials = Vec::new();
    for index in 1..num_partials {
        // Even partials are not present in a square wave
        if index % 2 == 0 {
            continue;
        }
        let partial = Partial {
            freq: index as f32,
            amp: 1.0 / index as f32,
            phase: 0.0,
        };
        partials.push(partial);
    }
    partials.clone()
}

// This function combines creating a wavetable from a list of Partials, and
// normalizing the output vector so it's maximum absolute value is 1.0. These
// operations are combined here for computational efficiency.
pub fn make_fourier_table_norm(table_size: usize, partials: Vec<Partial>) -> Wavetable {
    let mut wavetable: Vec<f32> = Vec::new();
    // Track maximum amplitude while creating wavetable, since we need to know
    // that to be able to normalize the wavetable.
    let mut maximum_amplitude = 0.0;
    let ts: f32 = 1.0 / table_size as f32;
    for i in 0..table_size {
        let mut sample: f32 = 0.0;
        for partial in partials.iter() {
            let angle = TWO_PI * partial.freq * i as f32 * ts + partial.phase;
            sample += angle.sin() * partial.amp;
        }
        if sample.abs() > maximum_amplitude {
            maximum_amplitude = sample.abs();
        }
        wavetable.push(sample);
    }
    // Apply normalization to the generated wavetable by multiplying every
    // sample by the normalization scale factor
    let scale_factor: f32 = 1.0 / maximum_amplitude;
    for sample in wavetable.iter_mut() {
        *sample *= scale_factor;
    }
    // Return the generated wavetable
    wavetable.clone()
}
