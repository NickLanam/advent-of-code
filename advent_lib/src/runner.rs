use crate::color::*;
use cargo_metadata::MetadataCommand;
use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};
use std::{path::PathBuf, process::Command};

/// Solutions for a given day call this function to organize their solvers.
/// The solution to use is executed as a child process from that year's bin.
/// That child process is also written in Rust, and imports this library too.
pub fn exec_day(year: u16, day: u16, workspace_root: &PathBuf) {
  let res = Command::new("cargo")
    .current_dir(workspace_root)
    .arg("-q")
    .arg("run")
    .arg("--package")
    .arg(format!("advent_of_code_{year}"))
    .arg("--bin")
    .arg(format!("day{day:0>2}"))
    .spawn();

  match res {
    Ok(mut child) => {
      child.wait().expect("Child process failed to start");
    }
    Err(err) => {
      eprintln!(
        "{RED} ✕ {BOLD}RUN FAILED:{RESET}{}\n",
        err
          .to_string()
          .split('\n')
          .map(|l| String::from("   ") + l)
          .collect::<Vec<String>>()
          .join("\n")
      );
      std::process::exit(1);
    }
  }
}

pub enum PartId {
  P1,
  P2,
}

/// The required functions to solve a given day's puzzle.
pub struct RunDetails<Parsed, Part1Solution, Part2Solution> {
  pub year: u16,
  pub day: u16,
  /// The file has already been opened, trimmed, and converted to a Vec of lines.
  ///
  /// Turn it into whatever format you need here.
  ///
  /// If not set, the default implementation is a passthrough (only worked if Parsed is still Vec<String>!)
  pub parse: fn(raw: Vec<String>, sample_name: Option<String>, for_part: PartId) -> Parsed,
  /// If sample_name is set, it declares _which_ sample is being run.
  pub part1: fn(parsed: Parsed, sample_name: Option<String>) -> Part1Solution,
  /// If sample_name is set, it declares _which_ sample is being run.
  pub part2: fn(parsed: Parsed, sample_name: Option<String>) -> Part2Solution,
}

fn find_sample_names(year: u16, day: u16, workspace_root: &PathBuf) -> Vec<String> {
  let re =
    Regex::new(format!("^day{day:0>2}\\.(?P<name>[^\\.]+)\\.sample\\.txt$").as_str()).unwrap();

  let listing =
    fs::read_dir(workspace_root.join(format!("{year}/input"))).expect("Failed to open input dir");

  let mut sample_names: Vec<String> = Vec::new();
  for f in listing.into_iter().filter_map(|l| l.ok()) {
    let file_name = String::from(f.file_name().to_str().unwrap());
    if let Some(caps) = re.captures(file_name.as_str()) {
      let name = &caps["name"];

      // Make sure that the expectations exist too. If not, stop and tell the user so.
      let p1expect = workspace_root.join(format!("{year}/input/day{day:0>2}.{name}.expect.1.txt"));
      let p2expect = workspace_root.join(format!("{year}/input/day{day:0>2}.{name}.expect.2.txt"));
      if !p1expect.exists() || !p2expect.exists() {
        panic!("Sample file {file_name} should be accompanied by two expect files, day{day:0>2}.{name}.expect.1.txt and day{day:0>2}.{name}.expect.2.txt");
      }
      sample_names.push(name.to_string());
    }
  }
  return sample_names;
}

fn load(path: PathBuf) -> Vec<String> {
  fs::read_to_string(&path)
    .expect(format!("Failed to read {:?}", &path.file_name()).as_str())
    .trim()
    .split('\n')
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn duration_string(duration: Duration) -> String {
  if duration.as_micros() < 1 {
    return format!(" {: >3}{CYAN}ns{RESET}", duration.subsec_nanos());
  } else if duration.as_millis() < 1 {
    return format!(" {: >3}{GREEN}µs{RESET}", duration.subsec_micros());
  } else if duration.as_millis() <= 99 {
    return format!(
      " {: >4.1}{BRIGHT_BLACK}ms{RESET}",
      (duration.subsec_micros() as f64) / 1_000.0
    );
  } else if duration.as_millis() <= 999 {
    return format!(" {}{BRIGHT_BLACK}ms{RESET}", duration.subsec_millis());
  } else if duration.as_secs() <= 9 {
    return format!(
      "{:.1}{RED}sec{RESET}",
      (duration.as_millis() as f64) / 1_000.0
    );
  } else {
    return format!("{} {RED}seconds{RESET}", duration.as_secs());
  }
}

/// Note: This runs in a process controlled by dayXX.rs, not the initial executable.
/// Yes, both import the same library. I should split things better for faster compilation and less mind-bending.
pub fn run<Parsed, Part1Solution, Part2Solution>(
  details: RunDetails<Parsed, Part1Solution, Part2Solution>,
) where
  Part1Solution: std::fmt::Debug,
  Part2Solution: std::fmt::Debug,
  Parsed: std::fmt::Debug,
{
  let RunDetails {
    year,
    day,
    parse,
    part1,
    part2,
  } = details;

  let binding = MetadataCommand::new().exec().unwrap().workspace_root;
  let workspace_root = PathBuf::from(&binding);

  println!("🎄 {BOLD}Advent of Code {GREEN}{year}{RESET}, Day {BOLD}{GREEN}{day}{RESET} 🎄");

  let sample_names = find_sample_names(year, day, &workspace_root);
  let sample_files = sample_names
    .iter()
    .map(|n| {
      let sample = load(workspace_root.join(format!("{year}/input/day{day:0>2}.{n}.sample.txt")));
      let expect1 =
        load(workspace_root.join(format!("{year}/input/day{day:0>2}.{n}.expect.1.txt")));
      let expect2 =
        load(workspace_root.join(format!("{year}/input/day{day:0>2}.{n}.expect.2.txt")));
      return (n.clone(), sample, expect1, expect2);
    })
    .collect::<Vec<_>>();
  let real_lines = load(workspace_root.join(format!("{year}/input/day{day:0>2}.real.txt")));

  // Each input has to run through the code's parser twice, once for each part.
  // For most days, this is redundant, but some days need slightly different parse
  // logic for each part of the puzzle. For example, 2023 day 23 or 2024 day 15.

  let cold_start = Instant::now();

  // Check that part 1 passes all samples. If it does, run it on the real input.
  let mut part1_test_failures = 0;
  for (sample_name, sample_lines, expect1_lines, _) in sample_files.iter() {
    let parsed = parse(
      sample_lines.clone(),
      Some(sample_name.to_string()),
      PartId::P1,
    );
    let out = part1(parsed, Some(sample_name.to_string()));
    let out_string = format!("{out:#?}");
    let expect_string = expect1_lines.join("\n");
    if out_string == expect_string {
      // TODO: some test_only flag that renders this line and skips running against real
      // println!(" {GREEN}✓ {RESET}Part 1 test {YELLOW}{sample_name}{RESET} {GREEN}passed{RESET}");
    } else {
      eprintln!(" {RED}✕ {RESET}Part 1 test {YELLOW}{sample_name}{RESET} {RED}failed{RESET}\n   {GREEN}Expected: {RESET}{expect_string}\n   {RED}Received: {RESET}{out_string}");
      part1_test_failures += 1;
    }
  }
  if part1_test_failures == 0 {
    let parse_start = Instant::now();
    let parsed = parse(real_lines.clone(), None, PartId::P1);
    let parse_duration = parse_start.elapsed();
    let solve_start = Instant::now();
    let out = part1(parsed, None);
    let solve_duration = solve_start.elapsed();
    let time_str = format!(
      "{BRIGHT_BLACK}(Parse {RESET}{}{BRIGHT_BLACK}, Solve {RESET}{}{BRIGHT_BLACK}){RESET}",
      duration_string(parse_duration),
      duration_string(solve_duration)
    );
    println!(" {BOLD}{BRIGHT_YELLOW}★ {RESET}Star 1 {time_str}: {YELLOW}{out:?}{RESET}");
  } else {
    eprintln!(" {RED}★ {RESET}Star 1: {RED}{part1_test_failures:?} failed test(s){RESET}");
  }

  // Same process for part 2.
  let mut part2_test_failures = 0;
  for (sample_name, sample_lines, _, expect2_lines) in sample_files.iter() {
    let parsed = parse(
      sample_lines.clone(),
      Some(sample_name.to_string()),
      PartId::P2,
    );
    let out = part2(parsed, Some(sample_name.to_string()));
    let out_string = format!("{out:#?}");
    let expect_string = expect2_lines.join("\n");
    if out_string == expect_string {
      // TODO: some test_only flag that renders this line and skips running against real
      // println!(" {GREEN}✓ {RESET}Part 2 test {YELLOW}{sample_name}{RESET} {GREEN}passed{RESET}");
    } else {
      eprintln!(" {RED}✕ {RESET}Part 2 test {YELLOW}{sample_name}{RESET} {RED}failed{RESET}\n   {GREEN}Expected: {RESET}{expect_string}\n   {RED}Received: {RESET}{out_string}");
      part2_test_failures += 1;
    }
  }
  if part2_test_failures == 0 {
    let parse_start = Instant::now();
    let parsed = parse(real_lines.clone(), None, PartId::P2);
    let parse_duration = parse_start.elapsed();
    let solve_start = Instant::now();
    let out = part2(parsed, None);
    let solve_duration = solve_start.elapsed();
    let time_str = format!(
      "{BRIGHT_BLACK}(Parse {RESET}{}{BRIGHT_BLACK}, Solve {RESET}{}{BRIGHT_BLACK}){RESET}",
      duration_string(parse_duration),
      duration_string(solve_duration)
    );
    println!(" {BOLD}{BRIGHT_YELLOW}★ {RESET}Star 2 {time_str}: {YELLOW}{out:?}{RESET}");
  } else {
    eprintln!(" {RED}★ {RESET}Star 2: {RED}{part2_test_failures:?} failed test(s){RESET}");
  }

  println!(
    "⌛{BOLD}{BRIGHT_BLACK} Total:{RESET}{}",
    duration_string(cold_start.elapsed())
  );
}
