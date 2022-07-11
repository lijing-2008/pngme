use std::fs;
use std::str::FromStr;

use clap::Parser;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::PngError;
use crate::png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod error;
mod utils;

// cargo run
fn main() -> Result<(), PngError> {
    let cli = args::Cli::parse();

    match &cli.command {
        args::Command::Encode(cmd) => {
            // get basic info
            let input_file = &cmd.file_path;
            let chunk_type = ChunkType::from_str(cmd.chunk_type.as_str())?;
            let chunk_data = cmd.message.clone().into_bytes();
            let message_chunk = Chunk::new(chunk_type, chunk_data);
            // create Png from file path
            let mut png = Png::from_file(input_file)?;
            // add secret message chunk
            png.append_chunk(message_chunk);
            // output new file path
            if let Some(output_path) = &cmd.out_path {
                fs::write(output_path, png.as_bytes())?;
            }
        }
        args::Command::Decode(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let png = Png::from_file(input_file)?;
            let message = png.data_string_by_type(chunk_type);
            match message {
                None => { println!("no such message for chunk_type: {}.", chunk_type) }
                Some(msg) => {
                    println!("secret msg for {} is: {}", chunk_type, msg)
                }
            }
        }
        args::Command::Remove(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let mut png = Png::from_file(input_file)?;
            let chunk_removed = png.remove_chunk(chunk_type)?;
            fs::write(input_file, png.as_bytes())?;
            println!("remove chunk type: {}", chunk_type);
        }
        args::Command::Print(cmd) => {
            let input_file = &cmd.file_path;
            let mut png = Png::from_file(input_file)?;
            let v = &png.chunks;
            let num = v.len();
            println!("====================all chunk type({num})====================");
            for i in 0..num {
                print!("{}:{};", i + 1, v[i].chunk_type().to_string());
            }
        }
    }

    Ok(())
}
