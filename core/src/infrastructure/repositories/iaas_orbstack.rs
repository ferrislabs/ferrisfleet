use std::process::Command;

use crate::domain::{
    entities::node::{Node, NodeRole, NodeStatus},
    error::FleetError,
    ports::iaas::IaasPort,
};

#[derive(Clone)]
pub struct OrbStackAdapter;

impl OrbStackAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OrbStackAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl IaasPort for OrbStackAdapter {
    async fn create_vm(&self, name: &str, role: NodeRole) -> Result<Node, FleetError> {
        let output = Command::new("orbctl")
            .args([
                "vm", "create", "--name", name, "--cpu", "2", "--memory", "2048",
            ])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: e.to_string(),
            })?;

        if !output.status.success() {
            return Err(FleetError::InternalServerError {
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        // TODO: récupérer l'ID réel de la VM depuis OrbStack (pour l’instant = name)
        Ok(Node {
            id: name.to_string(),
            name: name.to_string(),
            role,
            status: NodeStatus::Provisioning,
        })
    }

    async fn delete_vm(&self, id: &str) -> Result<(), FleetError> {
        let output = Command::new("orbctl")
            .args(["vm", "delete", id])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: e.to_string(),
            })?;

        if !output.status.success() {
            return Err(FleetError::InternalServerError {
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    async fn list_vms(&self) -> Result<Vec<Node>, FleetError> {
        let output = Command::new("orbctl")
            .args(["list"])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: e.to_string(),
            })?;

        if !output.status.success() {
            return Err(FleetError::InternalServerError {
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let nodes = stdout
            .lines()
            .skip(1) // skip header
            .map(|line| {
                let cols: Vec<&str> = line.split_whitespace().collect();
                Node {
                    id: cols[0].to_string(),
                    name: cols[1].to_string(),
                    role: NodeRole::Worker,
                    status: NodeStatus::Ready,
                }
            })
            .collect();

        Ok(nodes)
    }
}
