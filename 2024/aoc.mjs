import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

import { bold, dim, green, red, brightBlack, brightYellow, grey } from './utils/color.mjs';

// __dirname isn't available in ES modules, but it can still be determined.
// This would do strange things under Yarn PnP and other similar systems.
const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * 
 * @param {number} ms How many milliseconds in the duration. May be partial
 * @returns A string, including left padding and ANSI-escape-code-colored units.
 *   Distinguishes between microseconds, milliseconds, <10 seconds, and >=10 seconds.
 */
function formatDuration(ms) {
  if (ms < 1) {
    return ` ${String(Math.round(ms * 1000)).padStart(3, ' ')}${dim(green('µs'))}`;
  } else if (ms <= 99) {
    return `${ms.toFixed(1).padStart(4, ' ')}${grey('ms')}`;
  } else if (ms <= 999) {
    return ` ${Math.ceil(ms)}${grey('ms')}`;
  } else if (ms <= 9_999) {
    return `${(Math.ceil(ms) / 1_000).toFixed(1).padStart(3, ' ')}${red('sec')}`;
  } else {
    return `${Math.round(ms / 1_000)}${red(' seconds')}`;
  }
}

/**
 * Set up and run an Advent of Code challenge.
 * Takes the year, day number, a function to run for each star, and the expected output from
 * running those functions on that day's sample input.
 * 
 * Reads sample input from `input/day${day}.sample.txt`, and real from `input/day${day}.txt`.
 *
 * @template [ParsedInput=string[]]
 * @template [Part1Solution=number]
 * @template [Part2Solution=number]
 *
 * @param {number} year Integer, 2015 <= year <= $CURRENT_YEAR
 * @param {number} day Integer, 1 <= day <= 25
 * @param {(parsed: ParsedInput, isSample: boolean) => Part1Solution} part1 (parsedData, isSample) => solution
 * @param {Part1Solution} part1expected Expected output from running part1 on day$DAY.sample.txt
 * @param {(parsed: ParsedInput, isSample: boolean) => Part2Solution} part2 (parsedData, isSample) => solution
 * @param {Part2Solution} part2expected Expected output from running part2 on day$DAY.sample.txt
 * @param {(lines: string[], forPart:1|2) => ParsedInput} parse Transforms raw input lines into useful structures
 */
function aocActual(
  year,
  day,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
  trimLines = true, // Some challenges intentionally have leading or trailing whitespace in the lines.
  testOnly = false,
) {
  if (Number.isNaN(year) || year < 2015 || year > (new Date()).getFullYear() || Math.floor(year) !== year) {
    throw new Error(`Year must be an integer from 2015 to ${(new Date().getFullYear)} (inclusive), got ${year}`);
  }
  if (Number.isNaN(day) || day < 1 || day > 25 || Math.floor(day) !== day) {
    throw new Error(`Day must be an integer from 1 to 25 (inclusive), got ${day}`);
  }

  console.log(`🎄 ${bold('Advent of Code')} ${bold(green(year))}, Day ${bold(green(day))} 🎄`);

  const parser = typeof parse === 'function' ? parse : x => x;

  const runPart = (part, isSample) => {
    const solver = part === 1 ? part1 : part2;
    const expect = part === 1 ? part1expected : part2expected;

    const rawInput = load(day, isSample, trimLines);

    const parseStart = performance.now();
    const parsedInput = parser(rawInput, part);
    const parseEnd = performance.now();
    const parseTime = parseEnd - parseStart;

    const solveStart = performance.now();
    const result = solver(parsedInput, isSample);
    const solveEnd = performance.now();
    const solveTime = solveEnd - solveStart;

    const timeStr = `${grey('(Parse ')}${formatDuration(parseTime)}${grey(', Solve ')}${formatDuration(solveTime)}${grey(')')}`;

    if (isSample) {
      if (result !== expect) {
        console.error(bold(red(` ✕ Test for star ${part} failed!`)));
        console.error(`${bold(brightBlack('   Expected'))}:`, expect);
        console.error(`${bold(brightBlack('   Actual'))}:`, result);
        return { pass: false };
      } else if (testOnly) {
        console.info(`${bold(green('✓'))}  Test for star ${part} passed.`);
      }
      return { pass: true, parseTime, solveTime };
    } else {
      console.log(` ${bold(brightYellow('★'))} Star ${part} ${timeStr}:`, result);
      return { pass: true, parseTime, solveTime };
    }
  };

  const runStart = performance.now();

  const p1TestOut = runPart(1, true);
  if (p1TestOut.pass && !testOnly) runPart(1, false);
  const p2TestOut = runPart(2, true);
  if (p2TestOut.pass && !testOnly) runPart(2, false);

  console.debug(`${bold(grey('⌛ Total: '))}${formatDuration(performance.now() - runStart)}`);
}

function parse(raw, trimLines = true) {
  const lines = String(raw).split('\n').map(l => trimLines ? l.trim() : l);
  // Drop leading and trailing newlines, as those are usually artifacts.
  while (lines[0] === '') lines.shift();
  while (lines[lines.length - 1] === '') lines.pop();

  // Check that the input actually had contents, and that it wasn't downloaded too early
  if (lines.length === 0 || lines[0].startsWith("Please don't")) {
    console.error(red(' ✕ Sample is blank, or input is blank, or input was downloaded too early.'));
    process.exit(1);
  }
  return lines;
}

function load(day, sample = false, trimLines = true) {
  const raw = readFileSync(`${__dirname}/input/day${String(day).padStart(2, '0')}${sample ? '.sample' : ''}.txt`);
  return parse(raw, trimLines);
}

export default function aoc(...args) {
  if (args.length === 1 && args[0] != null && typeof args[0] === 'object' && !Array.isArray(args[0])) {
    const {
      year, day, part1, part1expected, part2, part2expected, parse, trimLines, testOnly,
    } = args[0];
    aocActual(year, day, part1, part1expected, part2, part2expected, parse, trimLines, testOnly)
  } else {
    aocActual(...args);
  }
}
