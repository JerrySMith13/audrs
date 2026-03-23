pub mod filter;
pub mod mix;
pub mod node;
pub mod source;

pub use filter::{Filter, FilterKind, Send};
pub use mix::Mix;
pub use node::{AudioNode, Buffer, NodeId};
pub use source::{Source, SrcOrigin};

enum Node {
    Source(Source),
    Filter(Filter),
    Mix(Mix),
}

pub struct Graph {
    nodes: Vec<Node>,
    routes: Vec<(NodeId, NodeId)>,
}

impl Graph {
    pub fn new() -> Self {
        let master = Node::Mix(Mix {
            filters: vec![],
            output: None,
        });
        Self {
            nodes: vec![master],
            routes: vec![],
        }
    }

    pub fn master(&self) -> NodeId {
        NodeId(0)
    }

    pub fn add_source(&mut self, src: Source) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node::Source(src));
        id
    }

    pub fn add_filter(&mut self, f: Filter) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node::Filter(f));
        id
    }

    pub fn add_mix(&mut self, m: Mix) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node::Mix(m));
        id
    }

    pub fn route(&mut self, from: NodeId, to_mix: NodeId) {
        self.routes.push((from, to_mix));
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
