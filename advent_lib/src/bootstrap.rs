use crate::color::*;
use crate::paths::{RelevantPaths, get_relevant_paths};
use anyhow::{Context, Result, bail};
use jiff::{Unit, Zoned, ZonedDifference, civil::date};
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::{thread, time::Duration};

type TaskResult = Result<Option<String>>;

/// Prepares solution files for a given day of Advent of Code.
/// * If that year's project is not yet set up, creates it first.
/// * If that day's code is not yet set up, creates it first.
/// * If that day's input is not yet downloaded, checks if it's available and then downloads it if so.
/// * If the input is not available yet, waits until it is and shows a countdown clock until then.
pub fn setup(year: u16, day: u16, workspace_root: &Path) -> Result<()> {
  let this_year = Zoned::now()
    .year()
    .try_into()
    .context("Couldn't get current year")?;

  if year < 2015 || year > this_year {
    bail!("Advent of Code does not exist for the year {year}.");
  }

  let final_puzzle_day = if year < 2025 { 25 } else { 12 };

  if !(1..=final_puzzle_day).contains(&day) {
    bail!(
      "Advent of Code {year} runs from December 1 through December {final_puzzle_day}, not {day}."
    );
  }

  let paths = get_relevant_paths(year, day, workspace_root);

  let need_file = |path: &Path| !path.exists() || fs::metadata(path).unwrap().size() == 0;
  let setup_required = !paths.src_bin.exists()
    || !paths.year_input.exists()
    || need_file(&paths.year_cargo_toml)
    || need_file(&paths.year_cookie)
    || need_file(&paths.day_rs)
    || need_file(&paths.real_input);

  if !setup_required {
    return Ok(());
  }

  println!(
    "🔧 {BRIGHT_BLACK}Setup:{RESET} {BOLD}Advent of Code {GREEN}{year}{RESET}, Day {BOLD}{GREEN}{day}{RESET} 🔧"
  );

  let tasks: Vec<Box<dyn Fn() -> TaskResult>> = vec![
    Box::new(|| maybe_init_year(year, &paths).with_context(|| "Failed to init year {year}")),
    Box::new(|| {
      maybe_init_day(year, day, &paths).with_context(|| "Failed to init day {year}-{day:0>2}")
    }),
    Box::new(|| {
      maybe_download_input(year, day, &paths)
        .with_context(|| "Failed to download input for {year} day {day:0>2}")
    }),
  ];

  for task in tasks.iter() {
    if let Some(msg) = task()? {
      println!(" {GREEN}✓ {RESET}{msg}");
    }
  }

  Ok(())
}

fn ask_for_cookie(cookie_path: &Path) -> Result<()> {
  print!(" • Paste your session cookie and press Enter: ");
  io::stdout().flush()?;
  let mut cookie = String::new();
  io::stdin()
    .read_line(&mut cookie)
    .context("Tried to read a line from stdin and was not able")?;

  fs::write(cookie_path, cookie.trim()).context("Failed to write cookie file")?;

  Ok(())
}

fn maybe_init_year(year: u16, paths: &RelevantPaths) -> TaskResult {
  let mut changed_something = false;

  if !paths.src_bin.exists() {
    fs::create_dir_all(&paths.src_bin)
      .with_context(|| format!("Could not create {:?}", paths.src_bin))?;
    changed_something = true;
  }

  if !paths.year_input.exists() {
    fs::create_dir_all(&paths.year_input)
      .with_context(|| format!("Could not create {:?}", paths.year_input))?;
    changed_something = true;
  }

  if !paths.year_cargo_toml.exists() {
    let template = fs::read_to_string(&paths.template_cargo_toml)
      .context("Failed to read Cargo.toml template")?;
    let contents = template.replace("%YEAR%", year.to_string().as_str());
    fs::write(&paths.year_cargo_toml, contents)
      .with_context(|| format!("Could not create {:?}", paths.year_cargo_toml))?;
    changed_something = true;
  }

  if !paths.year_cookie.exists() {
    ask_for_cookie(&paths.year_cookie)?;
    changed_something = true;
  }

  Ok(if changed_something {
    Some(format!("Created workspace for year {GREEN}{year}{RESET}"))
  } else {
    None
  })
}

fn maybe_init_day(year: u16, day: u16, paths: &RelevantPaths) -> TaskResult {
  if !paths.day_rs.exists() {
    let template =
      fs::read_to_string(&paths.template_day_rs).expect("Failed to read dayXX.rs template");
    let contents = template
      .replace("%YEAR%", year.to_string().as_str())
      .replace("%DAY%", day.to_string().as_str());
    let res = fs::write(&paths.day_rs, contents);
    if let Err(err) = res {
      bail!(err.to_string());
    }
    println!(
      "{GREEN} ✓{RESET} Created {GREEN}{UNDERLINE}{year}{BRIGHT_BLACK}/src/bin/{GREEN}{BOLD}day{day:0>2}.rs{RESET}"
    );
  }
  Ok(None)
}

fn wait_for_input_available(year: u16, day: u16) -> Result<bool> {
  // Puzzles unlock at midnight on the east coast of the USA.
  let unlock_datetime = date(year as i16, 12, day as i8)
    .at(0, 0, 0, 0)
    .in_tz("America/New_York")?;
  let mut now_datetime = Zoned::now().in_tz("America/New_York")?;

  let span_until = ZonedDifference::new(&unlock_datetime)
    .smallest(Unit::Second)
    .largest(Unit::Day);

  if now_datetime < unlock_datetime {
    let mut delta = now_datetime.until(span_until)?;
    println!("{RED} ✕{RESET} Waiting until puzzle unlocks in {RED}{delta:#}{RESET}");
    while now_datetime < unlock_datetime {
      thread::sleep(Duration::from_secs(1));
      now_datetime = Zoned::now().in_tz("America/New_York")?;
      delta = now_datetime.until(span_until)?;
      println!(
        "{CLEAR_TO_START_OF_PREVIOUS_LINE}{RED} ✕{RESET} Waiting until puzzle unlocks in {RED}{delta:#}{RESET}"
      );
    }
    print!("{CLEAR_TO_START_OF_PREVIOUS_LINE}");
    Ok(true)
  } else {
    Ok(false)
  }
}

fn maybe_download_input(year: u16, day: u16, paths: &RelevantPaths) -> TaskResult {
  let mut changed_something = false;

  // Create the input files if needed, even before downloading the actual input.
  // I do this so that I can open the files in my editor a few minutes before the puzzle unlocks.

  let sample_name = "test01";
  let sample_in = paths
    .year_input
    .join(format!("day{day:0>2}.{sample_name}.sample.txt"));
  let sample_out_1 = paths
    .year_input
    .join(format!("day{day:0>2}.{sample_name}.expect.1.txt"));
  let sample_out_2 = paths
    .year_input
    .join(format!("day{day:0>2}.{sample_name}.expect.2.txt"));
  let real_in = paths.year_input.join(format!("day{day:0>2}.real.txt"));

  if !sample_in.exists() {
    fs::write(sample_in, "SAMPLE").expect("Could not write sample input");
    fs::write(sample_out_1, "1").expect("Could not write sample output 1");
    fs::write(sample_out_2, "1").expect("Could not write sample output 2");
    changed_something = true;
  }

  if !real_in.exists() {
    fs::write(&real_in, "").expect("Could not create blank input file");
    changed_something = true;
  }

  // Show a live countdown timer until the puzzle unlocks
  if let Ok(true) = wait_for_input_available(year, day) {
    changed_something = true;
  }

  // Puzzle is unlocked, download the input if we need to
  // Note: this file is created above if it didn't already exist, so failing to read it is serious indeed.
  let real_in_contents =
    fs::read_to_string(&real_in).with_context(|| "Input file {real_in:?} went missing")?;

  if real_in_contents.is_empty() || real_in_contents.contains("Please don't repeatedly request") {
    let cookie = fs::read_to_string(&paths.year_cookie).context("Cookie file went missing")?;
    let client = reqwest::blocking::Client::new();
    let response = client
      .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
      .header("Cookie", format!("session={cookie}"))
      .send()
      .context("Failed to download input file")?;

    if response.status() == 500 {
      // TODO: Ask the user to input it again, then repeat, instead of crashing.
      bail!("Server gave a 500 response, cookie is likely stale");
    } else {
      let body = response.text().expect("Could not get body text properly");
      if body.contains("Please log in") {
        // TODO: Ask the user to input it again, then repeat, instead of crashing.
        bail!("Server says you aren't logged in, cookie is likely stale");
      } else {
        fs::write(&real_in, body.clone())
          .with_context(|| format!("Failed to save body to file:\n{body}"))?;
      }
    }
    changed_something = true;
  }

  if changed_something {
    Ok(Some(String::from("Downloaded input")))
  } else {
    Ok(None)
  }
}
