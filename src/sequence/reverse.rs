use crate::sequence::Sequence;
use crate::Residue;
use std::collections::VecDeque;


impl Sequence{

    pub fn trim_seq(&self, reference:&Sequence) -> Sequence{
        let mut ring= VecDeque::from(self.seq.clone());
        for r in reference.seq.iter().zip(self.seq.iter()){
            let (refe, target) = r;
            if *refe == Residue::Gap && *target != Residue::Gap{
                ring.rotate_left(1);
            }else if *refe != Residue::Gap && *target == Residue::Gap{
                    ring.rotate_right(1);
            }
            else if *refe != Residue::Gap && *target != Residue::Gap{
               break;
            }
        }

        Sequence{
            alphabet: self.alphabet,
            defline: self.defline.clone(),
            seq: ring.into_iter().filter(|&x| x!=Residue::Gap).collect()
        }

    }

    pub fn reverse(&self) ->Sequence{
        let mut new_seq = Vec::with_capacity(self.seq.len());
        for r in self.seq.iter().rev(){
                new_seq.push(*r)
        }
        Sequence{
            alphabet: self.alphabet,
            defline: self.defline.clone(),
            seq: new_seq
        }
    }
    pub fn complement(&self) ->Sequence{
        let mut new_seq = Vec::with_capacity(self.seq.len());
        for r in self.seq.iter(){
            new_seq.push(r.complement());
        }
        Sequence{
            alphabet: self.alphabet,
            defline: self.defline.clone(),
            seq: new_seq
        }
    }
}

