use std::{path::PathBuf, process::ExitCode};

#[derive(clap::Args, Debug)]
pub struct Args {
    pub path: PathBuf,
}

pub fn run(args: Args) -> ExitCode {
    let mut rdr = match csv::ReaderBuilder::new().from_path(&args.path) {
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

    let empty_col_idxs: Vec<usize> = empty_col_idx_counts
        .iter()
        .enumerate()
        .filter_map(|(i, count)| (*count == num_records).then_some(i))
        .collect();

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
