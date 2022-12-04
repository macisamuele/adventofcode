use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
struct Opts {
    /// Sets a custom input file for the assignment. Passing no value implies the usage of the "official" assignment input.
    // #[clap(short, long)]
    input_filename: Option<String>,
}

fn read_lines(filename: &str) -> std::io::Result<Box<dyn Iterator<Item = String>>> {
    Ok(Box::new(
        BufReader::new(File::open(filename)?)
            .lines()
            .into_iter()
            .filter_map(Result::ok),
    ))
}

/// # Errors
///
/// Will return `Err` if the `filename` provided in the CLI argumens does not exist
/// or the user does not have permission to read it.
pub fn input_lines(input: &str) -> anyhow::Result<Vec<String>> {
    let mut res: Vec<String> = match Opts::parse().input_filename.as_deref() {
        Some(path) => {
            eprintln!("Loading input from custom file: {}", &path);
            read_lines(&path)?.collect()
        }
        None => input.split('\n').map(ToString::to_string).collect(),
    };

    if res[res.len() - 1].is_empty() {
        res.remove(res.len() - 1);
    }
    Ok(res)
}
