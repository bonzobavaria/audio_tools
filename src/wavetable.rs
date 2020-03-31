use crate::constants::TWO_PI;

type Wavetable = Vec<f32>;

#[derive(Clone)]
pub struct Partial {
    freq: f32,
    amp: f32,
    phase: f32,
}

// Table generators

pub fn make_sin_saw_table(table_size: usize, num_partials: usize) -> Vec<Wavetable> {
    make_wavetable(
        table_size,
        vec![
            vec![Partial {
                freq: 1.0,
                amp: 1.0,
                phase: 0.0,
            }],
            make_triangle_partials(num_partials),
            make_square_partials(num_partials),
            make_sawtooth_partials(num_partials),
        ],
    )
}

pub fn make_wavetable(table_size: usize, partial_sets: Vec<Vec<Partial>>) -> Vec<Wavetable> {
    partial_sets
        .into_iter()
        .map(|pset| make_fourier_table_norm(table_size, pset))
        .collect()
}

pub fn make_exp_envelope(table_size: usize, curve: f32) -> Wavetable {
    let mut wavetable: Vec<f32> = Vec::new();
    let ts: f32 = 1.0 / table_size as f32;
    for i in 0..table_size {
        let sample: f32 = f32::powf(i as f32 * ts, curve);
        wavetable.push(sample);
    }
    wavetable
}

// Partial generators

fn make_triangle_partials(num_partials: usize) -> Vec<Partial> {
    // A triangle wave contains only odd harmonics, with alternating signs.
    let mut partials = Vec::new();
    for index in 1..num_partials {
        if index % 2 == 0 {
            continue;
        }
        let partial = Partial {
            freq: index as f32,
            amp: 1.0 / usize::pow(index, 2) as f32 * i32::pow(-1, index as u32 + 1) as f32,
            phase: 0.0,
        };
        partials.push(partial);
    }
    partials
}

fn make_square_partials(num_partials: usize) -> Vec<Partial> {
    let mut partials = Vec::new();
    for index in 1..num_partials {
        // A square wave contains only odd harmonics, with non-alternating signs.
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
    partials
}

fn make_sawtooth_partials(num_partials: usize) -> Vec<Partial> {
    // A sawtooth wave has energy at all harmonics, with alternating signs.
    let mut partials = Vec::new();
    for index in 1..num_partials {
        let partial = Partial {
            freq: index as f32,
            amp: 1.0 / index as f32 * i32::pow(-1, index as u32 + 1) as f32,
            phase: 0.0,
        };
        partials.push(partial);
    }
    partials
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
    wavetable
}
