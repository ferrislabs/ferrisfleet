use crate::{
    domain::error::FleetError,
    infrastructure::repositories::{IaasRepository, iaas_orbstack::OrbStackAdapter},
};

pub mod repositories;

pub struct Repositories {
    pub iaas_repo: IaasRepository,
}

pub async fn build_repos_from_conf() -> Result<Repositories, FleetError> {
    let iaas_repo = IaasRepository::Orbstack(OrbStackAdapter::new());
    Ok(Repositories { iaas_repo })
}
