
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget,
};

struct EqRenderable {
    block_buf: Vec<(f32, f32)>,
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
                    buf.cell_mut(Position::new(k as u16, i as u16)).unwrap().fg =
                        ratatui::style::Color::Green;
                }
            }
        }
    }
}
