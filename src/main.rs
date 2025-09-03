// Path: main.rs
mod cli;
mod error;
mod typst_converter;
use crate::cli::{Cli, Command};
use clap::Parser;
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::SingleFile(args) => {
            if !args.input_file.exists() {
                anyhow::bail!("Input file does not exist: {}", args.input_file.display());
            }
            let output_path = args.output_file.clone().unwrap_or_else(|| {
                args.input_file.with_extension("pdf")
            });
            println!("Converting {} to {}...", args.input_file.display(), output_path.display());
            typst_converter::convert_file(&args.input_file, &output_path)?;
            println!("✅ Conversion successful!");
        }
        Command::DirectoryBatch(args) => {
            if !args.input_dir.exists() {
                anyhow::bail!("Input directory does not exist: {}", args.input_dir.display());
            }
            println!("Converting all Markdown files in {}...", args.input_dir.display());
            typst_converter::convert_directory(&args)?;
        }
    }
    Ok(())
}
