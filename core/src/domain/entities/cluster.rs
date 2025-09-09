use serde::{Deserialize, Serialize};

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

pub struct K3sCluster {
    pub id: String,
    pub name: String,
    pub control_plane_nodes: Vec<Node>,
    pub worker_nodes: Vec<Node>,
    pub status: K3sClusterStatus,
    pub kubeconfig: Option<String>,
    pub api_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum K3sClusterStatus {
    Creating,
    Ready,
    Failed,
    Updating,
    Deleting,
}

impl K3sCluster {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            control_plane_nodes: Vec::new(),
            worker_nodes: Vec::new(),
            status: K3sClusterStatus::Creating,
            kubeconfig: None,
            api_endpoint: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.status, K3sClusterStatus::Ready)
    }
}
