use rustfft::{Fft, FftPlanner, num_complex::Complex};
use sndfile::{OpenOptions, SndFile, SndFileIO};
use std::sync::Arc;
const WINSIZE: usize = 2048;
fn hann_window(samples: &[f32]) -> Vec<f32> {
    let n = samples.len();
    samples
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let window =
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos());
            x * window
        })
        .collect()
}
/// Computes the magnitude spectrum (EQ) from a windowed float sample buffer.
/// Returns a Vec<f32> of length (n/2 + 1) — the rfft output bins.
pub fn rfft_magnitude(samples: &[f32], plan: &Arc<dyn Fft<f32>>) -> Vec<f32> {
    // rustfft works with complex buffers — pack real samples as (sample, 0.0i)
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex { re: s, im: 0.0 })
        .collect();

    plan.process(&mut buffer);

    // rfft only needs the first n/2 + 1 bins (the rest are conjugate mirrors)
    let num_bins = samples.len() / 2 + 1;

    buffer[..num_bins]
        .iter()
        .map(|c| c.norm()) // magnitude = sqrt(re² + im²)
        .collect()
}

/// Maps bin index to frequency in Hz given a sample rate.
pub fn bin_to_hz(bin: usize, n: usize, sample_rate: f32) -> f32 {
    bin as f32 * sample_rate / n as f32
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} <sound file path>", args[0]);
    }
    let opened = OpenOptions::ReadOnly(sndfile::ReadOptions::Auto);
    let mut file: SndFile = match opened.from_path(args[1].clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("eq: Error opening file");
            return;
        }
    };
    let channels = file.get_channels();
    let mut vec = vec![0 as f32; WINSIZE * channels];
    let mut single_channel = vec![0 as f32; WINSIZE];

    let mut i: u64 = 0;
    let len = file.len().expect("Uh oh");
    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let mut planner = FftPlanner::<f32>::new();
    let plan = planner.plan_fft_forward(WINSIZE);

    /*
        to do analysis:
        a. figure windows per second (sample rate / window size)
        b. use that to determine when each window should be played


    */

    while i <= len {
        file.read_to_slice(&mut vec).unwrap();
        for j in 0..(WINSIZE) {
            single_channel[j] = vec[j * 2];
        }
        let windowed = hann_window(&single_channel);
        let magnitudes = rfft_magnitude(&windowed, &plan);

        let eq_for_window = magnitudes
            .iter()
            .enumerate()
            .map(|(bin, mag)| {
                let new_mag = 20 as f32 * mag.log10();
                let new_hz = bin_to_hz(bin, WINSIZE, file.get_samplerate() as f32);
                (new_hz, new_mag)
            })
            .collect::<Vec<_>>();

        for sample in eq_for_window() {}

        i += (WINSIZE * 2) as u64;
    }
}
