use crate::types::nucleotide::Nucleotide;
use rand::random_range;

const BYTE_SEQUENCE_HEADER_SIZE: usize = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct NucleotideSequence {
    sequence: Vec<Nucleotide>,
}

impl NucleotideSequence {
    pub fn new(sequence: Vec<Nucleotide>) -> Self {
        Self { sequence }
    }

    pub fn random(length: u64) -> Self {
        let mut sequence = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let value = random_range(0..4) as u8;
            sequence.push(value.into());
        }
        Self::new(sequence)
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        // Will determine if the last byte contains an incomplete sequence of less than 4 bytes
        let mask_byte = (self.sequence.len() % 4) as u8;
        let byte_count = (self.sequence.len() + 3) / 4;

        let mut bytes = vec![0u8; byte_count + 1];
        bytes[0] = mask_byte;

        for (i, nucleotide) in self.sequence.iter().enumerate() {
            let byte_index = i / 4 + 1;
            let bit_position = 6 - 2 * (i % 4);
            bytes[byte_index] |= (*nucleotide as u8) << bit_position;
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::new(vec![]);
        }

        let mask_byte = bytes[0] as usize;
        let mut sequence = Vec::new();

        let last_byte_index = bytes.len().saturating_sub(1 + BYTE_SEQUENCE_HEADER_SIZE);

        bytes
            .iter()
            .skip(BYTE_SEQUENCE_HEADER_SIZE)
            .enumerate()
            .for_each(|(i, byte)| {
                let nucleotides_in_byte = if i == last_byte_index {
                    if mask_byte == 0 { 4 } else { mask_byte }
                } else {
                    4
                };

                for i in 0..nucleotides_in_byte {
                    let bit_position = 6 - 2 * i;
                    let bits = (byte >> bit_position) & 0b11;
                    sequence.push(bits.into())
                }
            });

        Self::new(sequence)
    }

    pub fn get_bit_string(&self) -> String {
        self.get_bytes()
            .iter()
            .skip(BYTE_SEQUENCE_HEADER_SIZE)
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
}
