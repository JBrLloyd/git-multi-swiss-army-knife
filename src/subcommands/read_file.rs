use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

use clap::Args;
use super::subcommand_handler::SubcommandHandler;
use crate::terminal::{error, warning };

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct ReadFileArgs {
    pub path: std::path::PathBuf,

    #[arg(long)]
    pub pattern: Option<String>,
}

impl SubcommandHandler for ReadFileArgs {
    fn handle(&self) {
        println!("pattern: {:?}, path: {:?}", &self.pattern, &self.path);

        let f = File::open(&self.path).expect("could not read file");
        let reader = BufReader::new(f);

        let start = Instant::now();

        let matched_lines: Vec<String> = reader.lines()
            .filter_map(|line| match line {
                Ok(l) => Some(l),
                Err(_) => {
                    warning("Error with reading line");
                    None
                }
            })
            .collect();

        if matched_lines.len() == 0 {
            error("File had no content.")
        }

        let num_ten_pows = matched_lines.len().ilog10() + 1;

        let mut has_matched_line = false;

        for (idx, line) in matched_lines.iter().enumerate() {
            let padding_len = if idx == 0 { num_ten_pows } else { num_ten_pows - idx.ilog10() };

            match &self.pattern {
                Some(x) if line.contains(x) => {
                    has_matched_line = true;
                    println!("{:width$} |\t{content}", idx, width = padding_len as usize, content = line);
                },
                Some(_) => {}
                None => {},
            }
        }

        let duration = start.elapsed();
        println!("Time elapsed is: {:?}", duration);

        if !has_matched_line {
            println!("No matching lines found.")
        }
    }
}
