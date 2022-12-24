use anyhow::{Context, Result};
use clap::{Arg, Parser};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

pub fn load_data_file() -> Result<String> {
    let args = Args::parse();
    fs::read_to_string(args.file).context("cant load file")
}
