use super::node::NodeId;

pub struct Send {
    pub target: NodeId,
    pub level: f32,
}

pub enum FilterKind {
    Gain(f32),
}

pub struct Filter {
    pub kind: FilterKind,
    pub sends: Vec<Send>,
}
