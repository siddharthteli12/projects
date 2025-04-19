mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use commands::{Cli, Mode};
use png::Png;
use std::fs;
use std::str::FromStr;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.mode {
        Mode::Encode => {
            let file = fs::read(&args.file)?;
            let chunk_type = args.chunk_type.ok_or("Can't be None")?;
            let chunk_type = ChunkType::from_str(&chunk_type)?;
            let mut png = Png::try_from(file.as_slice())?;
            let message = args.message.ok_or("Not allowed")?;
            // Creating chunk from chunk type.
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            png.append_chunk(chunk);

            fs::write(args.file, png.as_bytes())?;
        }
        Mode::Decode => {
            let file = fs::read(args.file)?;

            // First get png from file as usual.
            let png = Png::try_from(file.as_slice())?;

            let chunk_type = args.chunk_type.ok_or("Can't be None")?;
            let result = png.chunk_by_type(&chunk_type).ok_or("Chunk not found")?;

            println!("{}", result.data_as_string()?);
        }
        Mode::Print => {
            let file = fs::read(args.file)?;
            println!("{}", Png::try_from(file.as_slice())?)
        }
        Mode::Remove => {
            let file = fs::read(&args.file)?;
            let chunk_type = args.chunk_type.ok_or("Can't be None")?;
            let mut png = Png::try_from(file.as_slice())?;

            let chunk = png.remove_first_chunk(&chunk_type)?;
            println!("{chunk}");

            fs::write(args.file, png.as_bytes())?;
        }
    }
    Ok(())
}
