pub type Buffer = Vec<f32>; // interleaved PCM samples

pub trait AudioNode {
    fn process(&mut self, buf: &mut Buffer);
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);
