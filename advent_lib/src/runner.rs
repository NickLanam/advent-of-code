use crate::color::*;
use anyhow::{Context, Result, bail};
use cargo_metadata::MetadataCommand;
use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};
use std::{path::PathBuf, process::Command};

/// Solutions for a given day call this function to organize their solvers.
/// The solution to use is executed as a child process from that year's bin.
/// That child process is also written in Rust, and imports this library too.
pub fn exec_day(year: u16, day: u16, workspace_root: &PathBuf) -> Result<()> {
  let exit_status = Command::new("cargo")
    .current_dir(workspace_root)
    .arg("-q")
    .arg("run")
    .arg("--release")
    .arg("--package")
    .arg(format!("advent_of_code_{year}"))
    .arg("--bin")
    .arg(format!("day{day:0>2}"))
    .spawn()
    .with_context(|| {
      format!("Failed to execute `cargo run --package advent_of_code_{year} --bin day{day:0>2}`")
    })?
    .wait()
    .context("Child process crashed")?;

  if !exit_status.success() {
    bail!("Child exited with {exit_status:?}");
  }
  Ok(())
}

#[derive(PartialEq)]
pub enum PartId {
  P1,
  P2,
}

fn duration_string(duration: Duration) -> String {
  if duration.as_micros() < 1 {
    format!("{: >4}{CYAN}ns{RESET}", duration.subsec_nanos())
  } else if duration.as_millis() < 1 {
    format!("{: >4}{GREEN}µs{RESET}", duration.subsec_micros())
  } else if duration.as_millis() <= 99 {
    format!(
      "{: >4.1}{BRIGHT_BLACK}ms{RESET}",
      (duration.subsec_micros() as f64) / 1_000.0
    )
  } else if duration.as_millis() <= 999 {
    format!("{: >4}{BRIGHT_BLACK}ms{RESET}", duration.subsec_millis())
  } else if duration.as_secs() <= 9 {
    format!(
      "{: >3.1}{RED}sec{RESET}",
      (duration.as_millis() as f64) / 1_000.0
    )
  } else {
    format!("{} {RED}seconds{RESET}", duration.as_secs())
  }
}

pub struct Sample {
  name: String,
  lines: Vec<String>,
  expect_lines_1: Option<Vec<String>>,
  expect_lines_2: Option<Vec<String>>,
}

pub trait Day<
  Parsed,
  Part1Solution: std::fmt::Debug + std::fmt::Display,
  Part2Solution: std::fmt::Debug + std::fmt::Display,
>
{
  /// The file has already been opened, trimmed, and converted to a Vec of lines.
  ///
  /// Turn it into whatever format you need here.
  ///
  /// If not set, the default implementation is a passthrough (only worked if Parsed is still Vec<String>!)
  fn parse(
    &self,
    raw: Vec<String>,
    sample_name: Option<String>,
    for_part: PartId,
  ) -> Result<Parsed>;

  /// If sample_name is set, it declares _which_ sample is being run.
  fn part1(&self, parsed: &Parsed, sample_name: Option<String>) -> Result<Part1Solution>;

  /// If sample_name is set, it declares _which_ sample is being run.
  fn part2(&self, parsed: &Parsed, sample_name: Option<String>) -> Result<Part2Solution>;

  fn load_inputs(&self, year: u16, day: u16) -> Result<(Vec<Sample>, Vec<String>)> {
    let binding = MetadataCommand::new()
      .exec()
      .context("Could not detect Cargo workspace root when loading inputs")?
      .workspace_root;
    let workspace_root = PathBuf::from(&binding);

    let load_input = |name: String| -> Result<Vec<String>> {
      let path = workspace_root.join(format!("{year}/input/day{day:0>2}.{name}.txt"));
      let mut out = fs::read_to_string(&path)
        .context("Failed to read input named {name}")?
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

      // Some puzzles have a blank line in the middle,
      // and some have meaningful leading whitespace.
      // Instead of trimming those, only remove blank
      // lines from the beginning and end of the file.
      // THOSE are never part of the puzzle (at least
      // for the first ten years of Advent of Code).
      // TODO: When let-chaining stabilizes, make a better condition check here.
      while !out.is_empty() && out.last().unwrap().is_empty() {
        out.pop();
      }
      while !out.is_empty() && out.first().unwrap().is_empty() {
        out.remove(0);
      }
      Ok(out)
    };

    let re = Regex::new(format!("^day{day:0>2}\\.(?P<name>[^\\.]+)\\.sample\\.txt$").as_str())
      .context("Faulty regex searching for sample input files")?;

    let listing = fs::read_dir(workspace_root.join(format!("{year}/input")))
      .context("Failed to open input dir")?;

    let sample_files = listing
      .into_iter()
      .filter_map(|l| l.ok())
      .map(|f| {
        let file_name = String::from(f.file_name().to_str().unwrap());
        if let Some(caps) = re.captures(file_name.as_str()) {
          let name = caps.name("name").unwrap().as_str().to_string();
          let lines = load_input(format!("{name}.sample"))?;
          let expect_lines_1 = load_input(format!("{name}.expect.1")).ok();
          let expect_lines_2 = load_input(format!("{name}.expect.2")).ok();

          Ok(Sample {
            name,
            lines,
            expect_lines_1,
            expect_lines_2,
          })
        } else {
          bail!("That isn't a sample file");
        }
      })
      .filter_map(|r| r.ok())
      .collect::<Vec<Sample>>();

    let real_lines = load_input("real".to_string())?;

    Ok((sample_files, real_lines))
  }

  fn run(&self, year: u16, day: u16) -> Result<()> {
    println!("🎄 {BOLD}Advent of Code {GREEN}{year}{RESET}, Day {BOLD}{GREEN}{day}{RESET} 🎄");

    // Each input has to run through the code's parser twice, once for each part.
    // For most days, this is redundant, but some days need slightly different parse
    // logic for each part of the puzzle. For example, 2023 day 23 or 2024 day 15.

    let (sample_files, real_lines) = &self.load_inputs(year, day)?;

    let cold_start = Instant::now();

    // Check that part 1 passes all samples. If it does, run it on the real input.
    let mut part1_test_failures = 0;
    for Sample {
      name,
      lines,
      expect_lines_1,
      ..
    } in sample_files.iter()
    {
      if expect_lines_1.is_none() {
        continue;
      }
      let parsed = &self
        .parse(lines.clone(), Some(name.to_string()), PartId::P1)
        .with_context(|| format!("Parsing error for sample {name}"))?;
      let out = &self
        .part1(parsed, Some(name.to_string()))
        .with_context(|| format!(" {RED}✕ {RESET}Part 1 error on sample {YELLOW}{name}{RESET}"))?;
      let out_string = out.to_string();
      let expect_string = expect_lines_1.to_owned().unwrap().join("\n");
      if out_string == expect_string {
        // TODO: some test_only flag that renders this line and skips running against real
        // println!(" {GREEN}✓ {RESET}Part 1 test {YELLOW}{sample_name}{RESET} {GREEN}passed{RESET}");
      } else {
        eprintln!(
          " {RED}✕ {RESET}Part 1 test {YELLOW}{name}{RESET} {RED}failed{RESET}\n   {GREEN}Expected: {RESET}{expect_string}\n   {RED}Received: {RESET}{out_string}"
        );
        part1_test_failures += 1;
      }
    }
    if part1_test_failures == 0 {
      let parse_start = Instant::now();
      let parsed = &self
        .parse(real_lines.clone(), None, PartId::P1)
        .context("Parse error on real input")?;
      let parse_duration = parse_start.elapsed();
      let solve_start = Instant::now();
      let out = &self
        .part1(parsed, None)
        .with_context(|| format!(" {RED}✕ {RESET}Part 1 error on real data"))?;
      let solve_duration = solve_start.elapsed();
      let time_str = format!(
        "{BRIGHT_BLACK}(Parse {RESET}{}{BRIGHT_BLACK}, Solve {RESET}{}{BRIGHT_BLACK}){RESET}",
        duration_string(parse_duration),
        duration_string(solve_duration)
      );
      println!(" {BOLD}{BRIGHT_YELLOW}★ {RESET}Star 1 {time_str}: {YELLOW}{out}{RESET}");
    } else {
      eprintln!(" {RED}★ {RESET}Star 1: {RED}{part1_test_failures:?} failed test(s){RESET}");
    }

    // Same process for part 2.
    let mut part2_test_failures = 0;
    for Sample {
      name,
      lines,
      expect_lines_2,
      ..
    } in sample_files.iter()
    {
      if expect_lines_2.is_none() {
        continue;
      }
      let parsed = &self
        .parse(lines.clone(), Some(name.to_string()), PartId::P2)
        .with_context(|| format!("Parsing error for sample {name}"))?;
      let out = &self
        .part2(parsed, Some(name.to_string()))
        .with_context(|| format!(" {RED}✕ {RESET}Part 2 error on sample {YELLOW}{name}{RESET}"))?;
      let out_string = out.to_string();
      let expect_string = expect_lines_2.to_owned().unwrap().join("\n");
      if out_string == expect_string {
        // TODO: some test_only flag that renders this line and skips running against real
        // println!(" {GREEN}✓ {RESET}Part 2 test {YELLOW}{name}{RESET} {GREEN}passed{RESET}");
      } else {
        eprintln!(
          " {RED}✕ {RESET}Part 2 test {YELLOW}{name}{RESET} {RED}failed{RESET}\n   {GREEN}Expected: {RESET}{expect_string}\n   {RED}Received: {RESET}{out_string}"
        );
        part2_test_failures += 1;
      }
    }
    if part2_test_failures == 0 {
      let parse_start = Instant::now();
      let parsed = &self
        .parse(real_lines.clone(), None, PartId::P2)
        .context("Parse error on real input")?;
      let parse_duration = parse_start.elapsed();
      let solve_start = Instant::now();
      let out = &self
        .part2(parsed, None)
        .with_context(|| format!(" {RED}✕ {RESET}Part 2 error on real data"))?;
      let solve_duration = solve_start.elapsed();
      let time_str = format!(
        "{BRIGHT_BLACK}(Parse {RESET}{}{BRIGHT_BLACK}, Solve {RESET}{}{BRIGHT_BLACK}){RESET}",
        duration_string(parse_duration),
        duration_string(solve_duration)
      );
      println!(" {BOLD}{BRIGHT_YELLOW}★ {RESET}Star 2 {time_str}: {YELLOW}{out}{RESET}");
    } else {
      eprintln!(" {RED}★ {RESET}Star 2: {RED}{part2_test_failures:?} failed test(s){RESET}");
    }

    println!(
      "⌛{BOLD}{BRIGHT_BLACK} Total:{RESET}{}",
      duration_string(cold_start.elapsed())
    );
    Ok(())
  }
}
