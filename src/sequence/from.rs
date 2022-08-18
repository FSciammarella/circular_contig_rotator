use pest::iterators::Pair;

use crate::Residue;
use crate::sequence::Sequence;
use crate::alnparser::{Rule,Alphabet};

impl Sequence{
    pub fn from(alphabet:Alphabet, seqrec: Pair<Rule>)->Sequence{
        let mut new_seq = Self::new(alphabet);
        for token in seqrec.into_inner(){
            match token.as_rule(){
                Rule::definition=>{
                    new_seq.defline = token.as_str().to_string();
                }
                Rule::dna_sequence | Rule::rna_sequence=>{
                    for seqline in token.into_inner(){
                        for ch in seqline.as_str().chars(){
                            new_seq.seq.push(Residue::from_char(ch.to_ascii_uppercase()))
                        }
                    }
                }
                _=>unreachable!()
            }
        }
    //        eprintln!("{}", &new_seq);
        new_seq
    }
}