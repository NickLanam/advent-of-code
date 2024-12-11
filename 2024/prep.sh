#!/bin/bash
if [ -z "${1}" ]; then
  echo "Please provide day number as an argument"
  exit 1
fi

BASE="$(dirname $(realpath "$0"))"
YEAR="$(basename "$BASE")" # Should be a year number!
DAY="$1"
PADDED_DAY="$(printf "%02d" "$DAY")"

CODE_FILE="${BASE}/day${PADDED_DAY}.mjs"

UNLOCK_TS=$(TZ=UTC0 date -d "${YEAR}-12-${PADDED_DAY}T05:00:00Z" +%s)
NOW_TS=$(TZ=UTC0 date +%s)
REMAIN_SEC=$(( $UNLOCK_TS - $NOW_TS ))

echo -e "🔧 \e[90mSetup:\e[0m \e[0;1mAdvent of Code \e[0m\e[32;1m${YEAR}\e[0m, Day \e[32;1m${DAY}\e[0m, in \e[0;4m${BASE}\e[0m 🔧"

# Create the solution file if it doesn't already exist
if [[ -f "$CODE_FILE" ]]; then
  echo -e " \e[32m✓\e[0m \e[0;4mday${PADDED_DAY}.mjs\e[0m \e[32malready exists.\e[0m" >&2
else
  awk '{printf "%s\n", l $0; l=RT}' > "$CODE_FILE" << EOF
import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number|'NYI'} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 'NYI';

/** @type Part2Solution */
const part2expected = 'NYI';

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (parsed) => {
  return 'NYI';
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed) => {
  return 'NYI';
};

aoc({
  year: ${YEAR},
  day: ${DAY},
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
EOF
  echo -e " \e[32m✓\e[0m \e[0;4mday${DAY}.mjs\e[0m \e[32mcreated.\e[0m" >&2
fi

# Create the input files (blank for now)
touch "${BASE}/input/day${PADDED_DAY}.sample.txt"
touch "${BASE}/input/day${PADDED_DAY}.txt"

if [[ $REMAIN_SEC -ge 0 ]]; then
  REMAIN_STRING=$(date -u -d "0 $UNLOCK_TS seconds - $NOW_TS seconds" +"%H:%M:%S")
  echo -e " \e[31m✕\e[0m Puzzle unlocks in \e[31m${REMAIN_STRING}\e[0m."
  echo -e "   Sleeping until puzzle unlocks."
  sleep $REMAIN_SEC
fi

echo -e " • Puzzle is \e[32mavailable\e[0m! Downloading input."
curl "https://adventofcode.com/${YEAR}/day/${DAY}/input" \
  --silent \
  -H "Cookie: session=$(cat "${BASE}/input/cookie.txt")" \
  -o "${BASE}/input/day${PADDED_DAY}.txt"
