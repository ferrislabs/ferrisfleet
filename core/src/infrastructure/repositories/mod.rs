use crate::{
    domain::{
        entities::node::{Node, NodeRole},
        error::FleetError,
        ports::iaas::IaasPort,
    },
    infrastructure::repositories::iaas_orbstack::OrbStackAdapter,
};

pub mod iaas_orbstack;

#[derive(Clone)]
pub enum IaasRepository {
    Orbstack(OrbStackAdapter),
}

impl IaasPort for IaasRepository {
    async fn create_vm(&self, name: &str, role: NodeRole) -> Result<Node, FleetError> {
        match self {
            IaasRepository::Orbstack(adapter) => adapter.create_vm(name, role).await,
        }
    }

    async fn delete_vm(&self, id: &str) -> Result<(), FleetError> {
        match self {
            IaasRepository::Orbstack(adapter) => adapter.delete_vm(id).await,
        }
    }

    async fn list_vms(&self) -> Result<Vec<Node>, FleetError> {
        match self {
            IaasRepository::Orbstack(adapter) => adapter.list_vms().await,
        }
    }
}
