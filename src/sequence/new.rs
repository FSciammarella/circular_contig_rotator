use crate::sequence::Sequence;
use crate::alnparser::Alphabet;

impl Sequence{
    pub fn new(alphabet:Alphabet)->Sequence{
        Sequence{
            alphabet:alphabet,
            defline:String::new(),
            seq: Vec::new(),
        }
    }
}