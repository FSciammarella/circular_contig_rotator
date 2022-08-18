use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[allow(unused_imports)]
use pest::Parser;
/// Aligment parser module, written with pest-rs.
mod alnparser;
/// Alignment related functionality and struct definition.
mod alignment;
/// Sequence related functionality and struct definition.
mod sequence;
/// Residue enum and related functionality.
mod residue;

use alnparser::{AlnParser,Rule};
use alignment::Alignment;
use residue::Residue;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rotator", about =
"CONTACT:
E-mail = fl.sciammarella+degen@ufrj.br

Laboratório de Biodiversidade Genômica
Centro de Ciências da Saúde
Universidade Federal do Rio de Janeiro - UFRJ

LICENSE: [ MIT | APACHE 2.0 ]

DESCRIPTION:
    A little tool that takes a fasta sequence and orients it according to coordinates given.",
author="Author = Sciammarella, F, BSc")]
struct Opt{
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short,long,hidden=true)]
    debug:bool,

    /// Input file
    #[structopt(short,long,parse(from_os_str))]
    input: PathBuf,
}


fn main() {
    let opt = Opt::from_args();
    if opt.debug{
        eprintln!("{:?}",opt);
        eprintln!("{:?}", Residue::N==Residue::Gap);
    }
    let mut  unparsed = File::open(opt.input).expect("Failed to open File");

    let mut buffer = String::new();
    unparsed.read_to_string(&mut buffer).unwrap();

    let tokens = AlnParser::parse(Rule::DNA_alignment, &buffer).expect("Failed to parse file, is the file a Fasta DNA/RNA file?").next().unwrap();

    let new_alignment = Alignment::from(tokens);
    let size = new_alignment.sequences[0].seq.len();
    if new_alignment.sequences.len() != 2{
	    eprintln!("Should have exactly 2 sequence!");
	    exit(1)
	}

    let result = new_alignment.sequences[1].trim_seq(&new_alignment.sequences[0]);
    println!("{}",result); 
   
}
