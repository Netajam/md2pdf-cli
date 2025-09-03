<!-- Path: README.md -->
# MD to PDF CLI Converter (Typst Edition)

`md2pdf-cli` is a simple and fast command-line tool to convert your Markdown files into high-quality PDFs by leveraging the power of the [Typst](https://typst.app/) typesetting engine.

This tool acts as a convenient wrapper around Typst, translating your Markdown into Typst markup and then using your existing Typst installation to compile it into a beautiful, professional-grade document.

## Features

-   **High-Quality Output:** Creates beautiful, typeset PDFs thanks to Typst.
-   **Lightweight:** A single binary that works with your existing Typst installation.
-   Convert a single Markdown file to PDF.
-   Recursively find and convert all `.md` files in a directory.
-   Specify a custom output file or a separate destination directory for converted PDFs.
-   Option to overwrite existing PDFs or skip them (default).
-   Progress bar for directory conversions.

## Prerequisites

This tool requires the **Typst CLI** to be installed and available in your system's `PATH`.

Please follow the official [Typst installation instructions](https://github.com/typst/typst#installation) before using `md2pdf-cli`. You can verify it's installed correctly by running:
```bash
typst --version
```

## Installation

### From GitHub Releases

You can download a pre-compiled binary for your operating system (Windows, macOS, Linux) from the [Releases](https://github.com/Netajam/md2pdf-cli/releases) page. Unzip the archive and place the executable in a directory included in your system's `PATH`.

### From Source

If you have the Rust toolchain installed, you can build and install directly from source:
```bash
cargo install --git https://github.com/Netajam/md2pdf-cli.git
```

## Usage

The tool is organized into two main subcommands: `file` for single files and `dir` for directories.

### Convert a Single File

Use the `file` subcommand to convert one Markdown file.

**1. Basic Conversion**

```bash
md2pdf-cli file <INPUT_FILE>
```
The output PDF will be created in the same directory with the same name (e.g., `doc.md` -> `doc.pdf`).

*Example:*
```bash
md2pdf-cli file my-report.md
```

**2. Specify an Output Path**

Use the `-o` or `--output` flag to specify a different name or location for the PDF.

```bash
md2pdf-cli file <INPUT_FILE> -o <OUTPUT_FILE>
```

*Example:*
```bash
md2pdf-cli file my-report.md --output final-version.pdf
```

---

### Convert a Directory

Use the `dir` subcommand to convert all `.md` files within a directory.

**1. Convert Files in the Current Directory**

Running the command without a path will process the current directory (`.`).

```bash
md2pdf-cli dir
```

**2. Convert Files in a Specific Directory**

Provide a path to the directory you want to process.

```bash
md2pdf-cli dir <INPUT_DIR>
```

*Example:*
```bash
md2pdf-cli dir ./notes/
```

**3. Specify an Output Directory**

Use the `-o` or `--output` flag to place all generated PDFs into a different directory.

```bash
md2pdf-cli dir <INPUT_DIR> -o <OUTPUT_DIR>
```
If the output directory doesn't exist, it will be created.

*Example:*
```bash
md2pdf-cli dir ./notes/ --output ./exported-pdfs/
```

**4. Overwrite Existing Files**

By default, the tool will skip converting a file if a PDF with the target name already exists. To force it to overwrite existing files, use the `--overwrite` flag.

```bash
md2pdf-cli dir ./notes/ --overwrite
```