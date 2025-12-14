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
