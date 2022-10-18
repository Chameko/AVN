use petgraph::stable_graph::StableGraph;

/// Represents the path between sequences.
pub enum Path {
    LoadPoint(String),
    Continue,
}

/// A possible event in a sequence
pub trait Block {}

/// A sequence of events that take place one after the other 
pub struct Sequence {
    events: Vec<Box<dyn Block>>,
}

/// The overall story of the visual novel
pub struct Story {
    tree: StableGraph<Sequence, Path>
}