use std::fmt;

use crate::alnparser::Alphabet;
use crate::sequence::Sequence;

mod new;
mod from;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Alignment{
    pub(crate) alphabet:Alphabet,
    pub(crate) sequences:Vec<Sequence>
}


impl fmt::Display for Alignment{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        for rec in self.sequences.iter(){
            write!(f,"{}\n",rec);
        }
        write!(f,"")
    }
}