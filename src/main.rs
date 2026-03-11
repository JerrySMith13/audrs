use sndfile::{OpenOptions, SndFile, SndFileIO};
use std::thread;
use std::sync::mpsc;
mod render;
mod eq;
const WINSIZE: usize = 2048;
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
    let sample_rate = file.get_samplerate();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let eq_analyzer = eq::EqAnalyze::new(WINSIZE, sample_rate);
    let (send_raw, recv_raw) = mpsc::channel();
    let (send_eq, recv_eq) = mpsc::channel();

    let factory = eq::EqFactory {

    }
    thread::spawn(|| {

    })

    /*
        to do analysis:
        a. figure windows per second (sample rate / window size)
        b. use that to determine when each window should be played


    */


    let ()
}
