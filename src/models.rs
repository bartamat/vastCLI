use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintRequest {
    pub sdate: String,
    pub duration: u32,
    pub maintenance_reason: String,
    pub maintenance_category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnlistRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinesResponse {
    pub machines: Vec<Machine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Machine {
    pub machine_id: u64,
    pub hostname: String,
    #[serde(default)]
    pub gpu_max_cur_temp: Option<f64>,
    #[serde(default)]
    pub reliability2: Option<f64>,
    #[serde(default)]
    pub gpu_occupancy: Option<String>,
    #[serde(default)]
    pub earn_hour: Option<f64>,
    #[serde(default)]
    pub driver_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedMachine {
    pub machine_id: u64,
    pub hostname: String,
}

impl MaintRequest {
    pub fn new(
        sdate: String,
        duration: u32,
        maintenance_reason: String,
        maintenance_category: String,
    ) -> Self {
        Self {
            sdate,
            duration,
            maintenance_reason,
            maintenance_category,
        }
    }
}

impl UnlistRequest {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
