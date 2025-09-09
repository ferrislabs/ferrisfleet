use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::domain::{
    entities::node::{Node, NodeRole, NodeStatus},
    error::FleetError,
    ports::iaas::IaasPort,
};

#[derive(Clone)]
pub struct OrbStackAdapter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrbStackVmInfo {
    #[serde(flatten)]
    pub record: OrbStackVmRecord,
    pub disk_size: u64,
    pub ip4: Option<String>,
    pub ip6: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrbStackVmRecord {
    pub id: String,
    pub name: String,
    pub image: VmImage,
    pub config: VmConfig,
    pub builtin: bool,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VmImage {
    pub distro: String,
    pub version: String,
    pub arch: String,
    pub variant: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VmConfig {
    pub isolated: bool,
    pub default_username: String,
}

impl From<OrbStackVmRecord> for OrbStackVmInfo {
    /// Convert a list record to info structure (without IP/disk info)
    fn from(record: OrbStackVmRecord) -> Self {
        Self {
            record,
            disk_size: 0,
            ip4: None,
            ip6: None,
        }
    }
}

impl OrbStackVmInfo {
    /// Get the VM record regardless of source (list or info)
    pub fn record(&self) -> &OrbStackVmRecord {
        &self.record
    }
}

impl OrbStackAdapter {
    pub fn new() -> Self {
        Self
    }

    pub async fn exec_on_vm(&self, vm_name: &str, command: &str) -> Result<String, FleetError> {
        let output = Command::new("orbctl")
            .args(["vm", "exec", vm_name, "--", "sh", "-c", command])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: format!("failed to execute command on VM: {}", e),
            })?;

        if output.status.success() {
            return Err(FleetError::InternalServerError {
                message: format!(
                    "command failed on VM {}: {}",
                    vm_name,
                    String::from_utf8_lossy(&output.stderr)
                ),
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// List VMs using JSON format (basic info from list)
    async fn list_vm_records(&self) -> Result<Vec<OrbStackVmRecord>, FleetError> {
        let output = Command::new("orbctl")
            .args(["list", "--format", "json"])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: format!("failed to list VMs: {}", e),
            })?;

        if !output.status.success() {
            return Err(FleetError::InternalServerError {
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let vm_records: Vec<OrbStackVmRecord> =
            serde_json::from_slice(&output.stdout).map_err(|e| {
                FleetError::InternalServerError {
                    message: format!("failed to parse VM list JSON: {}", e),
                }
            })?;

        Ok(vm_records)
    }

    fn vm_record_to_node(&self, vm_record: &OrbStackVmRecord, role: Option<NodeRole>) -> Node {
        let status = match vm_record.state.as_str() {
            "running" => NodeStatus::Ready,
            "stopped" | "stopping" => NodeStatus::Failed,
            _ => NodeStatus::Provisioning,
        };

        let node_role = role.unwrap_or_else(|| {
            if vm_record.name.contains("cp")
                || vm_record.name.contains("control")
                || vm_record.name.contains("master")
            {
                NodeRole::ControlPlane
            } else {
                NodeRole::Worker
            }
        });

        Node {
            id: vm_record.id.clone(),
            name: vm_record.name.clone(),
            role: node_role,
            status,
        }
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
        let vm_records = self.list_vm_records().await?;

        // Convert to Node entities
        let nodes = vm_records
            .iter()
            .map(|vm_record| self.vm_record_to_node(vm_record, None))
            .collect();

        Ok(nodes)
    }

    async fn get_vm_ip(&self, name: &str) -> Result<String, FleetError> {
        let output = Command::new("orbctl")
            .args(["info", name, "--format", "json"])
            .output()
            .map_err(|e| FleetError::InternalServerError {
                message: format!("failed to get VM IP: {}", e),
            })?;

        if !output.status.success() {
            return Err(FleetError::InternalServerError {
                message: format!(
                    "Failed to get VM IP: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            });
        }

        let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(ip)
    }
}
