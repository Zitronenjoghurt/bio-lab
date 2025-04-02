use crate::types::nucleotide::{Nucleotide, Nucleotide::*};

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
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonALA {
    GCT,
    GCC,
    GCA,
    GCG,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonASN {
    AAT,
    AAC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonASP {
    GAT,
    GAC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonCYS {
    TGT,
    TGC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLN {
    CAA,
    CAG,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLU {
    GAA,
    GAG,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonGLY {
    GGT,
    GGC,
    GGA,
    GGG,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonHIS {
    CAT,
    CAC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonILE {
    ATT,
    ATC,
    ATA,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonLYS {
    AAA,
    AAG,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonPHE {
    TTT,
    TTC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonPRO {
    CCT,
    CCC,
    CCA,
    CCG,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonSTOP {
    TAA,
    TAG,
    TGA,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonTHR {
    ACT,
    ACC,
    ACA,
    ACG,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonTYR {
    TAT,
    TAC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodonVAL {
    GTT,
    GTC,
    GTA,
    GTG,
}
