#[derive(Debug, Clone)]
pub enum NodeRole {
    ControlPlane,
    Worker,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub role: NodeRole,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Provisioning,
    Ready,
    Failed,
}

impl Node {
    pub fn new(id: String, name: String, role: NodeRole) -> Self {
        Self {
            id,
            name,
            role,
            status: NodeStatus::Provisioning,
        }
    }

    pub fn mark_ready(&mut self) {
        self.status = NodeStatus::Ready;
    }

    pub fn mark_failed(&mut self) {
        self.status = NodeStatus::Failed;
    }
}
