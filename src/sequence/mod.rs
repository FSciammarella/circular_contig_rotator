use std::fmt;

use crate::alnparser::Alphabet;
use crate::residue::Residue;

mod new;
mod from;
mod reverse;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Sequence{
    pub(crate) alphabet:Alphabet,
    pub(crate) defline:String,
    pub(crate) seq:Vec<Residue>,
}

impl fmt::Display for Sequence{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
 //       write!(f, ">{}\n",self.defline);
        for ch in self.seq.clone().iter(){
            write!(f,"{}",ch);
        }
        write!(f,"")
    }
}