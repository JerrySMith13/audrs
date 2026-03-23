use super::node::NodeId;

pub struct Mix {
    pub filters: Vec<NodeId>,
    pub output: Option<NodeId>,
}
