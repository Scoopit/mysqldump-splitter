use std::{
    fs::{create_dir_all, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use color_eyre::eyre::{Context, Result};
use output::{Output, OutputFile};

/// Split mysql dumps by database/tables
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output directory
    #[clap(short, long, value_parser)]
    output: String,
    /// Compress each output file.
    ///
    /// Output .gz gzipped compressed files instead of plain text .sql files
    #[clap(short, long, value_parser)]
    compress: bool,

    /// Read this file instead of the standard input
    #[clap(short, long, value_parser)]
    input: Option<String>,
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut reader: Box<dyn BufRead> = {
        match &args.input {
            Some(file) => Box::new(BufReader::with_capacity(
                8192 * 1000,
                File::open(file).with_context(|| format!("Cannot open {file}"))?,
            )),
            None => Box::new(io::stdin().lock()),
        }
    };

    let output_dir = PathBuf::from(&args.output);
    create_dir_all(&output_dir)
        .with_context(|| format!("Cannot create output directory {}", args.output))?;

    let mut line_num = 0;

    let mut buf = Vec::with_capacity(8192);
    let mut parser = parser::Parser::new();

    let mut output_file = OutputFile {
        table: None,
        database: None,
    };

    let mut output = output::Output::new(&output_dir, &output_file, args.compress)?;

    loop {
        buf.truncate(0);

        line_num += 1;
        reader
            .read_until(b'\n', &mut buf)
            .with_context(|| format!("Cannot read dump from stdin at line {line_num}"))?;
        if buf.len() == 0 {
            // EOF
            break;
        }

        match parser.parse(&buf)? {
            parser::StateChange::Database(db) => {
                output_file = OutputFile {
                    database: Some(db),
                    table: None,
                };
                output = Output::new(&output_dir, &output_file, args.compress)?;
            }
            parser::StateChange::Table(table) => {
                output_file.table = Some(table);
                output = Output::new(&output_dir, &output_file, args.compress)?;
            }
            parser::StateChange::None => (),
        }

        parser.output(&mut output)?;
    }

    Ok(())
}
mod output;
mod parser;
