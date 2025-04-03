use crate::lab::config::LabConfig;
use std::path::PathBuf;

fn get_default_config() -> LabConfig {
    let path = PathBuf::from("./data/config/default_lab_config.yaml");
    LabConfig::load(&path).unwrap()
}

#[test]
fn test_config() {
    let config = get_default_config();
    assert_eq!(config.codon_strength.ala, 1.0);
}
