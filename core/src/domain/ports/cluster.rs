use crate::domain::{
    entities::{cluster::Cluster, node::Node},
    error::FleetError,
};

pub trait ClusterService: Clone + Send + Sync {
    fn create_cluster(
        &self,
        name: &str,
        control_planes: usize,
        workers: usize,
    ) -> impl Future<Output = Result<Cluster, FleetError>> + Send;
    fn list_vms(&self) -> impl Future<Output = Result<Vec<Node>, FleetError>> + Send;
}
