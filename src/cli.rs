use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    Parse {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    }
}
