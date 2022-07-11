use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// simple program to hide secret message in png
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// encode: hide secret into png / encode <file_path> <chunk_type> <message> <out_file_path>
    Encode(EncodeArgs),
    /// decode: get secret message by chunk type / decode <file_path> <chunk_type>
    Decode(DecodeArgs),
    /// remove: remove secret message by chunk type / remove <file_path> <chunk_type>
    Remove(RemoveArgs),
    /// print: print all chunk type / print <file_path>
    Print(PrintArgs),
}

#[clap(author, version, about, long_about = None)]
#[derive(Args)]
pub struct EncodeArgs {
    /// input file_path
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,

    #[clap(value_parser)]
    pub message: String,

    #[clap(value_parser)]
    pub out_path: Option<PathBuf>,
}

#[clap(author, version, about, long_about = None)]
#[derive(Args)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[clap(author, version, about, long_about = None)]
#[derive(Args)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[clap(author, version, about, long_about = None)]
#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
}

