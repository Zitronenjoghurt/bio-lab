use crate::lab::types::codon::Codon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodonValueConfig<V: Copy> {
    pub met: V,
    pub stop: V,
    pub ala: V,
    pub arg: V,
    pub asn: V,
    pub asp: V,
    pub cys: V,
    pub gln: V,
    pub glu: V,
    pub gly: V,
    pub his: V,
    pub ile: V,
    pub leu: V,
    pub lys: V,
    pub phe: V,
    pub pro: V,
    pub ser: V,
    pub thr: V,
    pub trp: V,
    pub tyr: V,
    pub val: V,
}

impl<V: Copy> CodonValueConfig<V> {
    pub fn get_value(&self, codon: Codon) -> V {
        match codon {
            Codon::Met => self.met,
            Codon::Stop(_) => self.stop,
            Codon::Ala(_) => self.ala,
            Codon::Arg(_) => self.arg,
            Codon::Asn(_) => self.asn,
            Codon::Asp(_) => self.asp,
            Codon::Cys(_) => self.cys,
            Codon::Gln(_) => self.gln,
            Codon::Glu(_) => self.glu,
            Codon::Gly(_) => self.gly,
            Codon::His(_) => self.his,
            Codon::Ile(_) => self.ile,
            Codon::Leu(_) => self.leu,
            Codon::Lys(_) => self.lys,
            Codon::Phe(_) => self.phe,
            Codon::Pro(_) => self.pro,
            Codon::Ser(_) => self.ser,
            Codon::Thr(_) => self.thr,
            Codon::Trp => self.trp,
            Codon::Tyr(_) => self.tyr,
            Codon::Val(_) => self.val,
        }
    }
}
