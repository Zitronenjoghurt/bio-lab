#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Nucleotide {
    A = 0b00,
    T = 0b01,
    G = 0b10,
    C = 0b11,
}

impl Nucleotide {
    pub fn complement(self) -> Self {
        match self {
            Nucleotide::A => Nucleotide::T,
            Nucleotide::T => Nucleotide::A,
            Nucleotide::G => Nucleotide::C,
            Nucleotide::C => Nucleotide::G,
        }
    }
}

impl From<u8> for Nucleotide {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Nucleotide::A,
            0b01 => Nucleotide::T,
            0b10 => Nucleotide::G,
            0b11 => Nucleotide::C,
            _ => unreachable!(),
        }
    }
}
