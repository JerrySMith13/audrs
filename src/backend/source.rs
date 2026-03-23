use std::sync::Arc;

pub enum SrcOrigin {
    File(Arc<str>),
}

pub struct Source {
    pub origin: SrcOrigin,
    pub sample_rate: u32,
    pub channels: u16,
}
