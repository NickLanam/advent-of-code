import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

import { bold, green, red, brightBlack, brightYellow } from './utils/color.mjs';

// __dirname isn't available in ES modules, but it can still be determined.
// This would do strange things under Yarn PnP and other similar systems.
const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Set up and run an Advent of Code challenge.
 * Takes the year, day number, a function to run for each star, and the expected output from
 * running those functions on that day's sample input.
 * 
 * Reads sample input from `input/day${day}.sample.txt`, and real from `input/day${day}.txt`.
 *
 * @param {number} year Integer, 2015 <= year <= $CURRENT_YEAR
 * @param {number} day  Integer, 1 <= day <= 25
 * @param {function} p1func (parsedData, isSample) => solution
 * @param {any} p1expect Expected output from running p1func on day$DAY.sample.txt
 * @param {function} p2func (parsedData, isSample) => solution
 * @param {any} p2expect Expected output from running p2func on day$DAY.sample.txt
 * @param {function} [parseFunc] Takes lines of the input, returns something useful to p1func and p2func.
 */
export default function aoc(
  year,
  day,
  p1func,
  p1expect,
  p2func,
  p2expect,
  parseFunc,
  trimLines = true, // Some challenges intentionally have leading or trailing whitespace in the lines.
  testOnly = false,
) {
  console.log(`🎄 ${bold('Advent of Code')} ${bold(green(year))}, Day ${bold(green(day))} 🎄`);
  const process = typeof parseFunc === 'function' ? parseFunc : x => x;

  // Re-invoke on each test to guarantee each test gets unmodified input
  const getSample = () => process(load(day, true, trimLines));
  const getInput = () => process(load(day, false, trimLines));

  const p1s = p1func(getSample(), true);
  if (p1s !== p1expect) {
    console.error(bold(red('✕  Test for star 1 failed!')));
    console.error(`${bold(brightBlack('   Expected'))}:`, p1expect);
    console.error(`${bold(brightBlack('   Actual'))}:`, p1s);
  } else if (testOnly) {
    console.log(`${bold(green('✓'))}  Test for star 1 passed.`);
  } else {
    console.log(` ${bold(brightYellow('★'))} Star 1:`, p1func(getInput(), false));
  }

  const p2s = p2func(getSample(), true);
  if (p2s !== p2expect) {
    console.error(bold(red('✕  Test for star 2 failed!')));
    console.error(`${bold(brightBlack('   Expected'))}:`, p2expect);
    console.error(`${bold(brightBlack('   Actual'))}:`, p2s);
  } else if (testOnly) {
    console.log(`${bold(green('✓'))}  Test for star 2 passed.`);
  } else {
    console.log(` ${bold(brightYellow('★'))} Star 2:`, p2func(getInput(), false));
  }
}

function parse(raw, trimLines = true) {
  const lines = String(raw).split('\n').map(l => trimLines ? l.trim() : l);
  // Drop leading and trailing newlines, as those are usually artifacts.
  while (lines[0] === '') lines.shift();
  while (lines[lines.length - 1] === '') lines.pop();

  // Check that the input actually had contents, and that it wasn't downloaded too early
  if (lines.length === 0 || lines[0].startsWith("Please don't")) {
    console.error(red('Sample is blank, or input is blank, or input was downloaded too early.'));
    process.exit(1);
  }
  return lines;
}

function load(day, sample = false, trimLines = true) {
  const raw = readFileSync(`${__dirname}/input/day${day}${sample ? '.sample' : ''}.txt`);
  return parse(raw, trimLines);
}