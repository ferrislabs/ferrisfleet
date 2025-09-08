use uuid::Uuid;

use crate::{
    domain::{
        entities::{
            cluster::Cluster,
            node::{Node, NodeRole},
        },
        error::FleetError,
        ports::{cluster::ClusterService, iaas::IaasPort},
    },
    infrastructure::repositories::IaasRepository,
};

#[derive(Clone)]
pub struct FerrisFleetService {
    iaas_repository: IaasRepository,
}

impl ClusterService for FerrisFleetService {
    async fn create_cluster(
        &self,
        name: &str,
        control_planes: usize,
        workers: usize,
    ) -> Result<Cluster, FleetError> {
        let mut cluster = Cluster::new(Uuid::new_v4().to_string(), name.to_string());

        for i in 0..control_planes {
            let node = self
                .iaas_repository
                .create_vm(&format!("{name}-cp-{i}"), NodeRole::ControlPlane)
                .await?;
            cluster.add_node(node);
        }

        for i in 0..workers {
            let node = self
                .iaas_repository
                .create_vm(&format!("{name}-worker-{i}"), NodeRole::Worker)
                .await?;
            cluster.add_node(node);
        }

        Ok(cluster)
    }

    async fn list_vms(&self) -> Result<Vec<Node>, FleetError> {
        self.iaas_repository.list_vms().await
    }
}
