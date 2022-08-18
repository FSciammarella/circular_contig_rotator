use crate::alignment::Alignment;
use crate::alnparser::Alphabet;

impl Alignment{
    pub fn new(alphabet:Alphabet)->Alignment{
        Alignment{
            alphabet,
            sequences:Vec::new(),
        }
    }
}