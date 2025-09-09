use crate::domain::{entities::node::Node, error::FleetError};

pub trait K8sPort: Clone + Send + Sync {
    fn install_single_node(&self, vm: &Node)
    -> impl Future<Output = Result<(), FleetError>> + Send;
    fn get_kubeconfig(&self, vm: &Node) -> impl Future<Output = Result<String, FleetError>> + Send;
    fn is_ready(&self, vm: &Node) -> impl Future<Output = Result<bool, FleetError>> + Send;
    fn uninstall(&self, vm: &Node) -> impl Future<Output = Result<(), FleetError>> + Send;
}
