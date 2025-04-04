use crate::lab::types::codon::Codon;
use crate::lab::types::gene_type::GeneTypeTrait;

#[derive(Debug)]
pub struct Gene {
    gene_type: Box<dyn GeneTypeTrait>,
    node_id: u64,
    strength: f32,
}

impl Gene {
    pub fn new(gene_type: Box<dyn GeneTypeTrait>, node_id: u64, strength: f32) -> Self {
        Self {
            gene_type,
            node_id,
            strength,
        }
    }

    pub fn from_gene_type(gene_type: Box<dyn GeneTypeTrait>, remaining_sequence: &[Codon]) -> Self {
        gene_type.parse_codon_sequence(remaining_sequence)
    }
}
