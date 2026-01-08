use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "csvu", version, about = "CSV utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    EmptyCols { path: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::EmptyCols { path } => {
            let mut rdr = match csv::ReaderBuilder::new().from_path(path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Unable to read file: {e}");
                    return ExitCode::FAILURE;
                }
            };

            let headers = match rdr.headers() {
                Ok(h) => h.clone(),
                Err(e) => {
                    eprintln!("Failed to read headers: {e}");
                    return ExitCode::FAILURE;
                }
            };

            let mut empty_col_idx_counts = vec![0usize; headers.len()];
            let mut num_records = 0usize;

            for result in rdr.records() {
                let record = match result {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Unable to read record: {e}");
                        return ExitCode::FAILURE;
                    }
                };

                num_records += 1;

                for (i, field) in record.iter().enumerate() {
                    if field.trim().is_empty() {
                        empty_col_idx_counts[i] += 1;
                    }
                }
            }

            let mut empty_col_idxs: Vec<usize> = Vec::new();

            for (i, count) in empty_col_idx_counts.iter().enumerate() {
                if *count == num_records {
                    empty_col_idxs.push(i);
                }
            }

            if !empty_col_idxs.is_empty() {
                println!("The following columns are completely empty:");

                for column_idx in empty_col_idxs {
                    let name = headers
                        .get(column_idx)
                        .filter(|s| !s.is_empty())
                        .unwrap_or("<unnamed>");

                    println!("- {name} (index {column_idx})");
                }
            } else {
                println!("No empty columns found.");
            }

            ExitCode::SUCCESS
        }
    }
}
