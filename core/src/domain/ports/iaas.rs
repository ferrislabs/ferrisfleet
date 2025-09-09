use crate::domain::{
    entities::node::{Node, NodeRole},
    error::FleetError,
};

pub trait IaasPort {
    fn create_vm(
        &self,
        name: &str,
        role: NodeRole,
    ) -> impl Future<Output = Result<Node, FleetError>> + Send;
    fn delete_vm(&self, id: &str) -> impl Future<Output = Result<(), FleetError>> + Send;
    fn list_vms(&self) -> impl Future<Output = Result<Vec<Node>, FleetError>> + Send;
    fn get_vm_ip(&self, name: &str) -> impl Future<Output = Result<String, FleetError>> + Send;
}
