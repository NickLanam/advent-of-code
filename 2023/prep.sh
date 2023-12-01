#!/bin/bash
if [ -z "${1}" ]; then
  echo "Please provide day number as an argument"
  exit 1
fi

YEAR=2023
BASE="$(dirname "$0")"

echo "Setup: Advent of Code ${YEAR}, Day ${1}, in ${BASE}"

touch "${BASE}/input/day${1}.sample.txt"

curl "https://adventofcode.com/${YEAR}/day/${1}/input" \
 --silent \
 -H "Cookie: session=$(cat "${BASE}/input/cookie.txt")" \
 -o "${BASE}/input/day${1}.txt"

CODE_FILE="${BASE}/day${1}.mjs"

if [[ -f "$CODE_FILE" ]]; then
  echo "day${1}.mjs already exists. Not overwriting." >&2
  exit 0
fi

awk '{printf "%s\n", l $0; l=RT}' > "$CODE_FILE" << EOF
(await import('./aoc.mjs')).default(
  ${YEAR}, ${1},
  (data) => {
    return 'NYI';
  }, 'NYI',
  (data) => {
    return 'NYI';
  }, 'NYI',
  data => data
);
EOF
