use super::node::Node;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub id: String,
    pub name: String,
    pub nodes: Vec<Node>,
}

impl Cluster {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
