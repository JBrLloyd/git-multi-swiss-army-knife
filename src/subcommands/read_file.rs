use std::{fs::File, io::{BufRead, BufReader}};

use clap::Args;
use super::subcommand_handler::SubcommandHandler;

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

        let mut has_matched_line = false;

        for (idx, line) in reader.lines().enumerate() {
            let line_content = line.unwrap();

            match &self.pattern {
                Some(x) if line_content.contains(x) => {
                    has_matched_line = true;
                    println!("{:?}|\t{}", idx + 1, line_content)
                },
                Some(_) => (),
                None => {
                    has_matched_line = true;
                    println!("{:?}|\t{}", idx + 1, line_content)
                },
            }
        }

        if !has_matched_line {
            println!("No matching lines found")
        }
    }
}