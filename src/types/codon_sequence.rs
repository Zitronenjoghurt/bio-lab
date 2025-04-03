use crate::types::codon::Codon;
use crate::types::nucleotide::Nucleotide;
use crate::types::nucleotide_sequence::NucleotideSequence;

#[derive(Debug, Clone, PartialEq)]
pub struct CodonSequence {
    sequence: Vec<Codon>,
    remainder: Vec<Nucleotide>,
}

impl CodonSequence {
    pub fn new(sequence: Vec<Codon>, remainder: Vec<Nucleotide>) -> Self {
        Self {
            sequence,
            remainder,
        }
    }

    pub fn get_sequence(&self) -> &[Codon] {
        &self.sequence
    }

    pub fn random(length: u32) -> Self {
        let nucleotide_sequence = NucleotideSequence::random(length as u64 * 3);
        Self::from_nucleotides(nucleotide_sequence.get_sequence())
    }

    pub fn from_nucleotides(nucleotides: &[Nucleotide]) -> Self {
        let chunks = nucleotides.chunks_exact(3);
        let remainder = chunks.remainder();
        let codons = chunks.filter_map(Codon::from_nucleotides).collect();
        Self::new(codons, remainder.to_vec())
    }

    pub fn to_nucleotides(&self) -> Vec<Nucleotide> {
        let mut nucleotides = Vec::with_capacity(self.sequence.len() * 3 + self.remainder.len());

        for codon in &self.sequence {
            nucleotides.extend_from_slice(&codon.to_nucleotides())
        }
        nucleotides.extend_from_slice(&self.remainder);

        nucleotides
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let nucleotides = self.to_nucleotides();
        let nucleotide_sequence = NucleotideSequence::new(nucleotides);
        nucleotide_sequence.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let nucleotide_sequence = NucleotideSequence::from_bytes(bytes);
        let nucleotides = nucleotide_sequence.get_sequence();
        Self::from_nucleotides(nucleotides)
    }

    pub fn get_bit_string(&self) -> String {
        let nucleotides = self.to_nucleotides();
        let nucleotide_sequence = NucleotideSequence::new(nucleotides);
        nucleotide_sequence.get_bit_string()
    }

    pub fn get_code_string(&self) -> String {
        let nucleotides = self.to_nucleotides();
        let nucleotide_sequence = NucleotideSequence::new(nucleotides);
        nucleotide_sequence.get_code_string()
    }
}
