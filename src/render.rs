use std::{sync::mpsc::Receiver, time::{Duration, Instant}};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget,
};

struct EqRenderable {
    block_buf: Vec<(f32, f32)>,
}

impl EqRenderable{
    pub fn new(blocks: Vec<(f32, f32)>) -> Self{
        Self {
            block_buf: blocks,
        }
    }
}

impl Widget for EqRenderable {
    fn render(self, rect: Rect, buf: &mut Buffer) {
        let distribution = self.block_buf.len() / rect.width as usize;

        let mut final_vals: Vec<(f32, f32)> = Vec::with_capacity(rect.width as usize);

        let i = 0;
        let mut curr_sum: f32 = 0.0;
        while i < self.block_buf.len() {
            if i != 0 && i % distribution == 0 {
                final_vals.push((
                    self.block_buf[i - (distribution / 2)].0,
                    curr_sum / distribution as f32,
                ));
                curr_sum = 0.0;
            }

            curr_sum += self.block_buf[i].1;
        }

        for i in 0..rect.height as i32 {
            for (k, x) in final_vals.iter().enumerate() {
                if x.1 > (i - 20) as f32 {
                    let cell = buf.cell_mut(Position::new(k as u16, i as u16)).unwrap();
                    cell.fg = ratatui::style::Color::Green;
                    cell.bg = ratatui::style::Color::Green;

                }
            }
        }
    }

}


pub struct RenderLoop{
    block_size: usize,
    sample_rate: usize,
    eq_recv: Receiver<Vec<(f32, f32)>>,
}

use crossterm::event;
impl RenderLoop{
    pub fn start_renderloop(&mut self) ->Result<(), ()>{
        let max_run = Duration::from_micros((self.block_size as u64* 1_000_000)/ self.sample_rate as u64);
        let mut deadline = Instant::now() + max_run;
        ratatui::run(
            |terminal| {
                loop{
                    let received = match self.eq_recv.recv(){
                        Ok(o) => o,
                        Err(_) => panic!("render: eq receiving went wrong!"),
                    };
                    terminal.draw(|frame| frame.render_widget(EqRenderable::new(received), frame.area())).expect("Error drawing to terminal");
                    if event::read().expect("Error reading crossterm event").is_key_press() {
                        break Ok(());
                    }
                    let coarse_sleep_until = deadline - Duration::from_millis(1);
                    if coarse_sleep_until > Instant::now() {
                        std::thread::sleep(coarse_sleep_until - Instant::now());
                    }
                    while Instant::now() < deadline {
                        std::hint::spin_loop();
                    }
                    deadline += max_run;
                }
            }

        )
    }

    pub fn new(block_size: usize, sample_rate: usize, eq_recv: Receiver<Vec<(f32, f32)>>,) -> Self{
        Self {
            block_size,
            sample_rate,
            eq_recv
        }
    }
}
