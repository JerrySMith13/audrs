use sndfile::{OpenOptions, SndFile, SndFileIO};
use std::thread;
use std::sync::mpsc;
mod render;
mod eq;
const WINSIZE: usize = 2048;

fn mix_to_mono(pcm: Vec<f32>) -> Vec<f32>{
    let mut filled = Vec::with_capacity(pcm.len()/2);
    for i in 0..pcm.len()/2{
        filled.push(pcm[i]);
    }
    return filled;
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
    let sample_rate = file.get_samplerate();
    let mut file_size = file.len().expect("failed to get audio file size");
    let channels = file.get_channels();
    let mut file: Vec<f32> = file.read_all_to_vec().unwrap();

    match channels{
        1 => {},
        2 => {
            file_size = file_size / 2;
            file = mix_to_mono(file);
        }
        _ => {
            panic!("error: only up to 2 channels supported");
        }
    }

    let (send_raw, recv_raw) = mpsc::channel();
    let (send_eq, recv_eq) = mpsc::channel();

    let eq_analyzer = eq::EqAnalyze::new(WINSIZE, sample_rate);
    let mut factory = eq::EqFactory::new(recv_raw, send_eq, eq_analyzer);
    let eq_handle = thread::spawn(move || {
        factory.run_loop();
    });

    let mut render_loop = render::RenderLoop::new(WINSIZE, sample_rate, recv_eq);
    let render_handle = thread::spawn(move ||{
        render_loop.start_renderloop();
    });

    for i in 0..file.len() / WINSIZE{
        let slice = &file[(i * WINSIZE)..((i + 1) *WINSIZE)];
        send_raw.send(slice.to_vec());

    }

    eq_handle.join();
    render_handle.join();





    /*
        to do analysis:
        a. figure windows per second (sample rate / window size)
        b. use that to determine when each window should be played


    */


}
