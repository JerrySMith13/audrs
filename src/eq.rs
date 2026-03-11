use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::sync::Arc;


const WINSIZE: usize = 2048;



pub struct EqAnalyze{
    plan: Arc<dyn Fft<f32>>,
    sample_rate: usize
}

impl EqAnalyze{
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
    fn rfft_magnitude(samples: &[f32], plan: &Arc<dyn Fft<f32>>) -> Vec<f32> {
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
    fn bin_to_hz(bin: usize, n: usize, sample_rate: f32) -> f32 {
        bin as f32 * sample_rate / n as f32
    }

    pub fn eq(&self, input: &[f32]) -> Vec<(f32, f32)>{
        let windowed = Self::hann_window(&input);
        let magnitudes = Self::rfft_magnitude(&windowed, &self.plan);

        let eq_for_window: Vec<(f32, f32)> = magnitudes
        .iter()
        .enumerate()
        .map(|(bin, mag)| {
            let new_mag = 20 as f32 * mag.log10();
            let new_hz = Self::bin_to_hz(bin, input.len(), self.sample_rate as f32);
            (new_hz, new_mag)
        })
        .collect::<Vec<_>>();
        return eq_for_window


    }

    pub fn new(winsize: usize, rate: usize) ->Self{
        let mut plan = FftPlanner::new();
        let plan = plan.plan_fft_forward(winsize);
        return Self{
            plan,
            sample_rate: rate,

        }
    }
}

use std::sync::mpsc::{Sender, Receiver};

enum EqErr{

}

pub struct EqFactory{
    aud: Receiver<Vec<f32>>,
    eq: Sender<Vec<(f32, f32)>>,
    analyzer: EqAnalyze
}

impl EqFactory{
    fn consume_frame(&mut self) -> Result<(), ()> {
        let frame = match self.aud.recv(){
            Ok(v) => v,
            Err(_) => {panic!("EqFactory: error recieving audio frame, panicking...")}
        };

        let eq_for_frame = self.analyzer.eq(&frame);
        self.eq.send(eq_for_frame);

        Ok(())
    }

    pub fn run_loop(&mut self) -> Result<(), ()>{
        loop{
            self.consume_frame().unwrap();
        }
    }
    pub fn new(raw_channel: Receiver<Vec<f32>>, eqed: Sender<Vec<(f32, f32)>>, analyzer: EqAnalyze) -> Self{
        return Self{
            aud: raw_channel,
            eq: eqed,
            analyzer
        }
    }
}




