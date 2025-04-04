use crate::lab::types::codon::{Codon, CodonType};
use crate::lab::types::gene::Gene;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

pub trait GeneTypeTrait: Debug {
    fn id_sequence(&self) -> Vec<CodonType>;
    fn parse_codon_sequence(&self, sequence: &[Codon]) -> Gene;
    fn clone_boxed(&self) -> Box<dyn GeneTypeTrait>;

    fn id_sequence_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.id_sequence().hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone)]
pub enum GeneType {
    Color,
}

impl GeneTypeTrait for GeneType {
    fn id_sequence(&self) -> Vec<CodonType> {
        match self {
            GeneType::Color => CodonType::gene_type_id(0, 1),
        }
    }

    fn parse_codon_sequence(&self, sequence: &[Codon]) -> Gene {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn GeneTypeTrait> {
        Box::new(self.clone())
    }
}
