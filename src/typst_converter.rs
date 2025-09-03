// Path: typst_converter.rs
use crate::cli::DirArgs;
use crate::error::{AppError, Result};
use indicatif::{ProgressBar, ProgressStyle};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;
fn markdown_to_typst(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut typst_markup = String::new();
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => (),
                Tag::Heading { level, .. } => {
                    typst_markup.push_str(&format!("\n#heading(level: {})[", level as i64));
                }
                Tag::List(_) => (),
                Tag::Item => typst_markup.push_str("\n- "),
                Tag::Emphasis => typst_markup.push('_'),
                Tag::Strong => typst_markup.push('*'),
                _ => (),
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Paragraph => typst_markup.push_str("\n\n"),
                TagEnd::Heading { .. } => typst_markup.push_str("]\n"),
                TagEnd::List { .. } => typst_markup.push('\n'),
                TagEnd::Emphasis => typst_markup.push('_'),
                TagEnd::Strong => typst_markup.push('*'),
                _ => (),
            },
            Event::Text(text) => typst_markup.push_str(&text),
            Event::Code(code) => typst_markup.push_str(&format!("`{}`", code)),
            Event::SoftBreak => typst_markup.push_str("\\\n"),
            Event::HardBreak => typst_markup.push_str("\\\n"),
            _ => (),
        }
    }
    typst_markup
}
pub fn convert_file(input_path: &Path, output_path: &Path) -> Result<()> {
    let markdown_content = fs::read_to_string(input_path)?;
    let typst_source_text = markdown_to_typst(&markdown_content);
    let temp_dir = tempfile::tempdir()?;
    let temp_file_path = temp_dir.path().join("main.typ");
    fs::write(&temp_file_path, typst_source_text)?;
    let output = Command::new("typst")
        .arg("compile")
        .arg(&temp_file_path)
        .arg(output_path)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::ConversionError {
                    source: anyhow::anyhow!("The 'typst' command was not found.\nPlease ensure Typst is installed and in your system's PATH."),
                }
            } else {
                AppError::Io(e)
            }
        })?;
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::ConversionError {
            source: anyhow::anyhow!(
                "Typst compilation failed for {}:\n{}",
                input_path.display(),
                error_message
            ),
        });
    }
    Ok(())
}
pub fn convert_directory(args: &DirArgs) -> Result<()> {
    if !args.input_dir.is_dir() {
        return Err(AppError::InvalidInputPath {
            path: args.input_dir.clone(),
        });
    }
    let md_files: Vec<std::path::PathBuf> = WalkDir::new(&args.input_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .map(|e| e.path().to_path_buf())
        .collect();
    if md_files.is_empty() {
        println!("No Markdown files found in the specified directory.");
        return Ok(());
    }
    let bar = ProgressBar::new(md_files.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    for input_path in md_files {
        let file_name = input_path.file_name().unwrap().to_str().unwrap();
        bar.set_message(format!("Converting {}", file_name));
        let mut output_path = args.output_dir.clone().unwrap_or_else(|| {
            input_path.parent().unwrap().to_path_buf()
        });
        fs::create_dir_all(&output_path)?;
        output_path.push(input_path.file_stem().unwrap());
        output_path.set_extension("pdf");
        if !args.overwrite && output_path.exists() {
            bar.inc(1);
            continue;
        }
        if let Err(e) = convert_file(&input_path, &output_path) {
            bar.println(format!("⚠️ Failed to convert {}: {}", file_name, e));
        }
        bar.inc(1);
    }
    bar.finish_with_message("✅ All files processed!");
    Ok(())
}
