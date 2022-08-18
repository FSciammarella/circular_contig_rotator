use pest::iterators::Pair;

use crate::alignment::Alignment;
use crate::alnparser::{Alphabet,Rule};
use crate::sequence::Sequence;

impl Alignment {
    pub fn from(file: Pair<Rule>) -> Alignment {
        let alph = match file.as_rule() {
            Rule::DNA_alignment => Alphabet::DNA,
            Rule::RNA_alignment => Alphabet::RNA,
            _ => unreachable!()
        };
        let mut new_align = Self::new(alph);
        for records in file.into_inner() {
            match records.as_rule() {
                Rule::fasta_dna_rec | Rule::fasta_rna_rec => new_align.sequences.push(Sequence::from(new_align.alphabet, records)),
                _ => ()
            }
        }
        new_align
    }
}