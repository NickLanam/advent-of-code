use advent_lib::{bootstrap, color::*, runner};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use chrono::{prelude::Utc, Datelike};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct CommandLineArgs {
  #[arg(long, short)]
  year: Option<u16>,
  day: u16,
}

fn main() -> Result<()> {
  let args = CommandLineArgs::parse();
  let day = args.day;
  let year = args.year.unwrap_or(Utc::now().year().try_into().unwrap());

  let binding = MetadataCommand::new().exec().unwrap().workspace_root;
  let workspace_root = PathBuf::from(&binding);

  bootstrap::setup(year, day, &workspace_root)
    .with_context(|| format!("{RED} ✕ {BOLD}BOOTSTRAP FAILED{RESET}"))?;
  runner::exec_day(year, day, &workspace_root)
    .with_context(|| format!("{RED} ✕ {BOLD}RUNNER FAILED{RESET}"))?;
  Ok(())
}
