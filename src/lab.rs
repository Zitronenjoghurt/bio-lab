use crate::lab::config::LabConfig;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lab {
    config: LabConfig,
}

impl Lab {
    pub fn new(config: LabConfig) -> Self {
        Self { config }
    }
}
