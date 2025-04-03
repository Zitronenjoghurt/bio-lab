use crate::lab::config::codon_value_config::CodonValueConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod codon_value_config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabConfig {
    pub codon_strength: CodonValueConfig<f64>,
}

impl LabConfig {
    pub fn load(path: &Path) -> Result<Self, serde_yaml::Error> {
        let f = std::fs::File::open(path).expect("Failed to open config file");
        serde_yaml::from_reader(f)
    }
}
