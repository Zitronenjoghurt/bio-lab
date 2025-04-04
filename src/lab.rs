use crate::lab::config::LabConfig;
use crate::lab::types::gene_type::GeneTypeTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod config;
pub mod types;

#[derive(Debug)]
pub struct Lab {
    config: LabConfig,
    genes: HashMap<u64, Box<dyn GeneTypeTrait>>,
}

impl Lab {
    pub fn new(config: LabConfig, genes: HashMap<u64, Box<dyn GeneTypeTrait>>) -> Self {
        Self { config, genes }
    }

    pub fn register_gene(&mut self, gene_type: Box<dyn GeneTypeTrait>) {
        self.genes.insert(gene_type.id_sequence_hash(), gene_type);
    }
}
