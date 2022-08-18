use pest_derive::Parser;


#[derive(Copy,Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum Alphabet{
    DNA,
    RNA
}

#[derive(Parser,Debug)]
#[grammar= "grammars/fasta.pest"]
pub(crate) struct AlnParser;
