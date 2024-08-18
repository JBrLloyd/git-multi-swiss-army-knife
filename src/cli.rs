use crate::subcommands::subcommand_handler::SubcommandHandler;
use clap::{Subcommand, Parser};

use crate::subcommands::read_file::ReadFileArgs;

#[derive(Debug, Subcommand)]
enum Commands {
    // #[command(arg_required_else_help true)]
    ReadFile(ReadFileArgs),
}

#[derive(Debug, Parser)]
#[command(name = "repos")]
#[command(about = "A CLI for managing multiple repos easily.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub fn handle() {
    let args = Cli::parse();
    
    match args.command {
        Commands::ReadFile(args) => args.handle(),
    }
}