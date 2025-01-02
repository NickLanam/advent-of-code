# Advent of Code
My solutions for the programming puzzles in [Advent of Code](https://adventofcode.com).

Previous years were written in JavaScript (see `./javascript` for all of that code), but from now on they'll be in Rust.

The `aoc.sh` script at the root is all you need, other than an installation of Rust.
- Running `aoc [-y YEAR] <DAY>` will do the following:
  - If `YEAR` is not set, assumes current year
  - If there's no workspace directory for that year yet, creates it (`Cargo.toml` and the skeletal directories)
  - Prompts you for your session cookie if it's not stored yet
  - If there's no `day${DAY}.rs` file yet, creates it from a template
  - If there are no `day${DAY}.test01.{sample,expect.1,expect.2}.txt` files yet, creates them with trivial contents
  - If your input hasn't been downloaded yet:
    - If there's still time before the day's puzzle unlocks, shows a live countdown timer before continuing.
      This is done so that you can run the command a few minutes early and start opening tabs, and input will download when it's ready.
    - Downloads the input.
  - Compiles your code
  - Runs all of the tests for that day in `input/day${DAY}.*` files
  - If all of the tests pass, runs your compiled code for parts 1 and 2
  - Shows you timings for all of the above, shows failed test results, colorizes the output to be fancy, and generally makes my life a little easier.
