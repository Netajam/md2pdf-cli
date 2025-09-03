// Path: cli.rs
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A self-contained CLI tool to convert Markdown files to PDF using Typst.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "file")]
    SingleFile(FileArgs),
    #[command(name = "dir")]
    DirectoryBatch(DirArgs),
}
#[derive(Args, Debug)]
pub struct FileArgs {
    #[arg(required = true)]
    pub input_file: PathBuf,
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<PathBuf>,
}
#[derive(Args, Debug)]
pub struct DirArgs {
    #[arg(default_value = ".")]
    pub input_dir: PathBuf,
    #[arg(short = 'o', long = "output")]
    pub output_dir: Option<PathBuf>,
    #[arg(long = "overwrite", action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}
