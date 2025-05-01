mod assembler;
mod cli;
mod parser;

use std::{error::Error, fs::File};

use assembler::assemble_file;
use clap::Parser;

use crate::cli::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        cli::Commands::Parse { input, output } => {
            println!("Assembling file \"{}\"...", input);

            let mut source = File::open(input)?;
            assemble_file(&mut source, output.as_str())?;

            println!("Successfully assembled source to \"{}\".", output);
        }
    };

    Ok(())
}
