use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub mode: Mode,

    // Positional argument for the file path
    #[arg()]
    pub file: String,

    // Chunk type
    #[arg()]
    pub chunk_type: Option<String>,

    // Optional message for encode mode
    #[arg()]
    pub message: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Encode,
    Decode,
    Remove,
    Print,
}
