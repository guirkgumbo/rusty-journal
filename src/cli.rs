use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
  /// Write tasks to the journal
  Add {
    /// The task description text
    #[structopt()]
    text: String,
  },
  /// Remove an entry from the journal file by position
  Done {
    #[structopt()]
    position: usize,
  },
  /// List all tasks in journal file
  List,
}

// WoW so thats how you make command line nots
// That is pretty sweet
#[derive(Debug, StructOpt)]
#[structopt(
  name = "Rust Journal",
  about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub action: Action,

  /// Use a differenct journal file
  #[structopt(parse(from_os_str), short, long)]
  pub journal_file: Option<PathBuf>,
}