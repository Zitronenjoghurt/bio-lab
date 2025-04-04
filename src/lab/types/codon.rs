use crate::lab::types::nucleotide::{Nucleotide, Nucleotide::*};
use rand::rng;
use rand::seq::IndexedRandom;

const IDENTIFYING_CODON_TYPES: [CodonType; 19] = [
    CodonType::Ala,
    CodonType::Arg,
    CodonType::Asn,
    CodonType::Asp,
    CodonType::Cys,
    CodonType::Gln,
    CodonType::Glu,
    CodonType::Gly,
    CodonType::His,
    CodonType::Ile,
    CodonType::Leu,
    CodonType::Lys,
    CodonType::Phe,
    CodonType::Pro,
    CodonType::Ser,
    CodonType::Thr,
    CodonType::Trp,
    CodonType::Tyr,
    CodonType::Val,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Codon {
    Met,
    Stop(CodonSTOP),

    Ala(CodonALA),
    Arg(CodonARG),
    Asn(CodonASN),
    Asp(CodonASP),
    Cys(CodonCYS),
    Gln(CodonGLN),
    Glu(CodonGLU),
    Gly(CodonGLY),
    His(CodonHIS),
    Ile(CodonILE),
    Leu(CodonLEU),
    Lys(CodonLYS),
    Phe(CodonPHE),
    Pro(CodonPRO),
    Ser(CodonSER),
    Thr(CodonTHR),
    Trp,
    Tyr(CodonTYR),
    Val(CodonVAL),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonType {
    Met,
    Stop,
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
}

impl Codon {
    pub fn from_nucleotides(sequence: &[Nucleotide]) -> Option<Self> {
        if sequence.len() < 3 {
            return None;
        }

        let n1 = sequence[0];
        let n2 = sequence[1];
        let n3 = sequence[2];

        let codon = match (n1, n2, n3) {
            (T, T, T) => Self::Phe(CodonPHE::TTT),
            (T, T, C) => Self::Phe(CodonPHE::TTC),
            (T, T, A) => Self::Leu(CodonLEU::TTA),
            (T, T, G) => Self::Leu(CodonLEU::TTG),
            (C, T, T) => Self::Leu(CodonLEU::CTT),
            (C, T, C) => Self::Leu(CodonLEU::CTC),
            (C, T, A) => Self::Leu(CodonLEU::CTA),
            (C, T, G) => Self::Leu(CodonLEU::CTG),
            (A, T, T) => Self::Ile(CodonILE::ATT),
            (A, T, C) => Self::Ile(CodonILE::ATC),
            (A, T, A) => Self::Ile(CodonILE::ATA),
            (A, T, G) => Self::Met,
            (G, T, T) => Self::Val(CodonVAL::GTT),
            (G, T, C) => Self::Val(CodonVAL::GTC),
            (G, T, A) => Self::Val(CodonVAL::GTA),
            (G, T, G) => Self::Val(CodonVAL::GTG),
            (T, C, T) => Self::Ser(CodonSER::TCT),
            (T, C, C) => Self::Ser(CodonSER::TCC),
            (T, C, A) => Self::Ser(CodonSER::TCA),
            (T, C, G) => Self::Ser(CodonSER::TCG),
            (C, C, T) => Self::Pro(CodonPRO::CCT),
            (C, C, C) => Self::Pro(CodonPRO::CCC),
            (C, C, A) => Self::Pro(CodonPRO::CCA),
            (C, C, G) => Self::Pro(CodonPRO::CCG),
            (A, C, T) => Self::Thr(CodonTHR::ACT),
            (A, C, C) => Self::Thr(CodonTHR::ACC),
            (A, C, A) => Self::Thr(CodonTHR::ACA),
            (A, C, G) => Self::Thr(CodonTHR::ACG),
            (G, C, T) => Self::Ala(CodonALA::GCT),
            (G, C, C) => Self::Ala(CodonALA::GCC),
            (G, C, A) => Self::Ala(CodonALA::GCA),
            (G, C, G) => Self::Ala(CodonALA::GCG),
            (T, A, T) => Self::Tyr(CodonTYR::TAT),
            (T, A, C) => Self::Tyr(CodonTYR::TAC),
            (T, A, A) => Self::Stop(CodonSTOP::TAA),
            (T, A, G) => Self::Stop(CodonSTOP::TAG),
            (C, A, T) => Self::His(CodonHIS::CAT),
            (C, A, C) => Self::His(CodonHIS::CAC),
            (C, A, A) => Self::Gln(CodonGLN::CAA),
            (C, A, G) => Self::Gln(CodonGLN::CAG),
            (A, A, T) => Self::Asn(CodonASN::AAT),
            (A, A, C) => Self::Asn(CodonASN::AAC),
            (A, A, A) => Self::Lys(CodonLYS::AAA),
            (A, A, G) => Self::Lys(CodonLYS::AAG),
            (G, A, T) => Self::Asp(CodonASP::GAT),
            (G, A, C) => Self::Asp(CodonASP::GAC),
            (G, A, A) => Self::Glu(CodonGLU::GAA),
            (G, A, G) => Self::Glu(CodonGLU::GAG),
            (T, G, T) => Self::Cys(CodonCYS::TGT),
            (T, G, C) => Self::Cys(CodonCYS::TGC),
            (T, G, A) => Self::Stop(CodonSTOP::TGA),
            (T, G, G) => Self::Trp,
            (C, G, T) => Self::Arg(CodonARG::CGT),
            (C, G, C) => Self::Arg(CodonARG::CGC),
            (C, G, A) => Self::Arg(CodonARG::CGA),
            (C, G, G) => Self::Arg(CodonARG::CGG),
            (A, G, T) => Self::Ser(CodonSER::AGT),
            (A, G, C) => Self::Ser(CodonSER::AGC),
            (A, G, A) => Self::Arg(CodonARG::AGA),
            (A, G, G) => Self::Arg(CodonARG::AGG),
            (G, G, T) => Self::Gly(CodonGLY::GGT),
            (G, G, C) => Self::Gly(CodonGLY::GGC),
            (G, G, A) => Self::Gly(CodonGLY::GGA),
            (G, G, G) => Self::Gly(CodonGLY::GGG),
        };

        Some(codon)
    }

    pub fn to_nucleotides(&self) -> [Nucleotide; 3] {
        match self {
            Self::Phe(CodonPHE::TTT) => [T, T, T],
            Self::Phe(CodonPHE::TTC) => [T, T, C],
            Self::Leu(CodonLEU::TTA) => [T, T, A],
            Self::Leu(CodonLEU::TTG) => [T, T, G],
            Self::Leu(CodonLEU::CTT) => [C, T, T],
            Self::Leu(CodonLEU::CTC) => [C, T, C],
            Self::Leu(CodonLEU::CTA) => [C, T, A],
            Self::Leu(CodonLEU::CTG) => [C, T, G],
            Self::Ile(CodonILE::ATT) => [A, T, T],
            Self::Ile(CodonILE::ATC) => [A, T, C],
            Self::Ile(CodonILE::ATA) => [A, T, A],
            Self::Met => [A, T, G],
            Self::Val(CodonVAL::GTT) => [G, T, T],
            Self::Val(CodonVAL::GTC) => [G, T, C],
            Self::Val(CodonVAL::GTA) => [G, T, A],
            Self::Val(CodonVAL::GTG) => [G, T, G],
            Self::Ser(CodonSER::TCT) => [T, C, T],
            Self::Ser(CodonSER::TCC) => [T, C, C],
            Self::Ser(CodonSER::TCA) => [T, C, A],
            Self::Ser(CodonSER::TCG) => [T, C, G],
            Self::Ser(CodonSER::AGT) => [A, G, T],
            Self::Ser(CodonSER::AGC) => [A, G, C],
            Self::Pro(CodonPRO::CCT) => [C, C, T],
            Self::Pro(CodonPRO::CCC) => [C, C, C],
            Self::Pro(CodonPRO::CCA) => [C, C, A],
            Self::Pro(CodonPRO::CCG) => [C, C, G],
            Self::Thr(CodonTHR::ACT) => [A, C, T],
            Self::Thr(CodonTHR::ACC) => [A, C, C],
            Self::Thr(CodonTHR::ACA) => [A, C, A],
            Self::Thr(CodonTHR::ACG) => [A, C, G],
            Self::Ala(CodonALA::GCT) => [G, C, T],
            Self::Ala(CodonALA::GCC) => [G, C, C],
            Self::Ala(CodonALA::GCA) => [G, C, A],
            Self::Ala(CodonALA::GCG) => [G, C, G],
            Self::Tyr(CodonTYR::TAT) => [T, A, T],
            Self::Tyr(CodonTYR::TAC) => [T, A, C],
            Self::Stop(CodonSTOP::TAA) => [T, A, A],
            Self::Stop(CodonSTOP::TAG) => [T, A, G],
            Self::Stop(CodonSTOP::TGA) => [T, G, A],
            Self::His(CodonHIS::CAT) => [C, A, T],
            Self::His(CodonHIS::CAC) => [C, A, C],
            Self::Gln(CodonGLN::CAA) => [C, A, A],
            Self::Gln(CodonGLN::CAG) => [C, A, G],
            Self::Asn(CodonASN::AAT) => [A, A, T],
            Self::Asn(CodonASN::AAC) => [A, A, C],
            Self::Lys(CodonLYS::AAA) => [A, A, A],
            Self::Lys(CodonLYS::AAG) => [A, A, G],
            Self::Asp(CodonASP::GAT) => [G, A, T],
            Self::Asp(CodonASP::GAC) => [G, A, C],
            Self::Glu(CodonGLU::GAA) => [G, A, A],
            Self::Glu(CodonGLU::GAG) => [G, A, G],
            Self::Cys(CodonCYS::TGT) => [T, G, T],
            Self::Cys(CodonCYS::TGC) => [T, G, C],
            Self::Trp => [T, G, G],
            Self::Arg(CodonARG::CGT) => [C, G, T],
            Self::Arg(CodonARG::CGC) => [C, G, C],
            Self::Arg(CodonARG::CGA) => [C, G, A],
            Self::Arg(CodonARG::CGG) => [C, G, G],
            Self::Arg(CodonARG::AGA) => [A, G, A],
            Self::Arg(CodonARG::AGG) => [A, G, G],
            Self::Gly(CodonGLY::GGT) => [G, G, T],
            Self::Gly(CodonGLY::GGC) => [G, G, C],
            Self::Gly(CodonGLY::GGA) => [G, G, A],
            Self::Gly(CodonGLY::GGG) => [G, G, G],
        }
    }

    pub fn random_from_type(codon_type: CodonType) -> Self {
        match codon_type {
            CodonType::Met => Self::Met,
            CodonType::Stop => Self::Stop(CodonSTOP::random()),
            CodonType::Ala => Self::Ala(CodonALA::random()),
            CodonType::Arg => Self::Arg(CodonARG::random()),
            CodonType::Asn => Self::Asn(CodonASN::random()),
            CodonType::Asp => Self::Asp(CodonASP::random()),
            CodonType::Cys => Self::Cys(CodonCYS::random()),
            CodonType::Gln => Self::Gln(CodonGLN::random()),
            CodonType::Glu => Self::Glu(CodonGLU::random()),
            CodonType::Gly => Self::Gly(CodonGLY::random()),
            CodonType::His => Self::His(CodonHIS::random()),
            CodonType::Ile => Self::Ile(CodonILE::random()),
            CodonType::Leu => Self::Leu(CodonLEU::random()),
            CodonType::Lys => Self::Lys(CodonLYS::random()),
            CodonType::Phe => Self::Phe(CodonPHE::random()),
            CodonType::Pro => Self::Pro(CodonPRO::random()),
            CodonType::Ser => Self::Ser(CodonSER::random()),
            CodonType::Thr => Self::Thr(CodonTHR::random()),
            CodonType::Trp => Self::Trp,
            CodonType::Tyr => Self::Tyr(CodonTYR::random()),
            CodonType::Val => Self::Val(CodonVAL::random()),
        }
    }
}

impl CodonType {
    pub fn gene_type_id(value: u32, length: u8) -> Vec<Self> {
        let mut result = Vec::with_capacity(length as usize);
        let mut remaining = value;

        for _ in (0..length).rev() {
            let index = (remaining as usize % IDENTIFYING_CODON_TYPES.len());
            result.push(IDENTIFYING_CODON_TYPES[index]);
            remaining /= IDENTIFYING_CODON_TYPES.len() as u32;
        }

        result
    }
}

trait CodonVariant: Sized + Copy {
    fn all() -> Vec<Self>;
    fn random() -> Self {
        *Self::all().choose(&mut rng()).unwrap()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonALA {
    GCT,
    GCC,
    GCA,
    GCG,
}

impl CodonVariant for CodonALA {
    fn all() -> Vec<Self> {
        vec![Self::GCT, Self::GCC, Self::GCA, Self::GCG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonARG {
    CGT,
    CGC,
    CGA,
    CGG,
    AGA,
    AGG,
}

impl CodonVariant for CodonARG {
    fn all() -> Vec<Self> {
        vec![
            Self::CGT,
            Self::CGC,
            Self::CGA,
            Self::CGG,
            Self::AGA,
            Self::AGG,
        ]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonASN {
    AAT,
    AAC,
}

impl CodonVariant for CodonASN {
    fn all() -> Vec<Self> {
        vec![Self::AAT, Self::AAC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonASP {
    GAT,
    GAC,
}

impl CodonVariant for CodonASP {
    fn all() -> Vec<Self> {
        vec![Self::GAT, Self::GAC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonCYS {
    TGT,
    TGC,
}

impl CodonVariant for CodonCYS {
    fn all() -> Vec<Self> {
        vec![Self::TGT, Self::TGC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLN {
    CAA,
    CAG,
}

impl CodonVariant for CodonGLN {
    fn all() -> Vec<Self> {
        vec![Self::CAA, Self::CAG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLU {
    GAA,
    GAG,
}

impl CodonVariant for CodonGLU {
    fn all() -> Vec<Self> {
        vec![Self::GAA, Self::GAG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLY {
    GGT,
    GGC,
    GGA,
    GGG,
}

impl CodonVariant for CodonGLY {
    fn all() -> Vec<Self> {
        vec![Self::GGT, Self::GGC, Self::GGA, Self::GGG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonHIS {
    CAT,
    CAC,
}

impl CodonVariant for CodonHIS {
    fn all() -> Vec<Self> {
        vec![Self::CAT, Self::CAC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonILE {
    ATT,
    ATC,
    ATA,
}

impl CodonVariant for CodonILE {
    fn all() -> Vec<Self> {
        vec![Self::ATT, Self::ATC, Self::ATA]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonLEU {
    TTA,
    TTG,
    CTT,
    CTC,
    CTA,
    CTG,
}

impl CodonVariant for CodonLEU {
    fn all() -> Vec<Self> {
        vec![
            Self::TTA,
            Self::TTG,
            Self::CTT,
            Self::CTC,
            Self::CTA,
            Self::CTG,
        ]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonLYS {
    AAA,
    AAG,
}

impl CodonVariant for CodonLYS {
    fn all() -> Vec<Self> {
        vec![Self::AAA, Self::AAG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonPHE {
    TTT,
    TTC,
}

impl CodonVariant for CodonPHE {
    fn all() -> Vec<Self> {
        vec![Self::TTT, Self::TTC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonPRO {
    CCT,
    CCC,
    CCA,
    CCG,
}

impl CodonVariant for CodonPRO {
    fn all() -> Vec<Self> {
        vec![Self::CCT, Self::CCC, Self::CCA, Self::CCG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonSER {
    TCT,
    TCC,
    TCA,
    TCG,
    AGT,
    AGC,
}

impl CodonVariant for CodonSER {
    fn all() -> Vec<Self> {
        vec![
            Self::TCT,
            Self::TCC,
            Self::TCA,
            Self::TCG,
            Self::AGT,
            Self::AGC,
        ]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonSTOP {
    TAA,
    TAG,
    TGA,
}

impl CodonVariant for CodonSTOP {
    fn all() -> Vec<Self> {
        vec![Self::TAA, Self::TAG, Self::TGA]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonTHR {
    ACT,
    ACC,
    ACA,
    ACG,
}

impl CodonVariant for CodonTHR {
    fn all() -> Vec<Self> {
        vec![Self::ACT, Self::ACC, Self::ACA, Self::ACG]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonTYR {
    TAT,
    TAC,
}

impl CodonVariant for CodonTYR {
    fn all() -> Vec<Self> {
        vec![Self::TAT, Self::TAC]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonVAL {
    GTT,
    GTC,
    GTA,
    GTG,
}

impl CodonVariant for CodonVAL {
    fn all() -> Vec<Self> {
        vec![Self::GTT, Self::GTC, Self::GTA, Self::GTG]
    }
}
