use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RelevantPaths {
  pub template_cargo_toml: PathBuf,
  pub template_day_rs: PathBuf,
  pub year_cargo_toml: PathBuf,
  pub year_input: PathBuf,
  pub year_cookie: PathBuf,
  pub src_bin: PathBuf,
  pub day_rs: PathBuf,
  pub real_input: PathBuf,
}

pub fn get_relevant_paths(year: u16, day: u16, root: &Path) -> RelevantPaths {
  RelevantPaths {
    template_cargo_toml: root.join("advent_lib/templates/Cargo.toml.tmpl"),
    template_day_rs: root.join("advent_lib/templates/dayXX.rs.tmpl"),
    year_cargo_toml: root.join(format!("{year}/Cargo.toml")),
    year_input: root.join(format!("{year}/input")),
    year_cookie: root.join(format!("{year}/input/cookie.txt")),
    src_bin: root.join(format!("{year}/src/bin")),
    day_rs: root.join(format!("{year}/src/bin/day{day:0>2}.rs")),
    real_input: root.join(format!("{year}/input/day{day:0>2}.real.txt")),
  }
}
