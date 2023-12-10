#!/bin/bash

wasTooEarly() {
  grep -q "Please don't repeatedly request" "$1"
  echo $?
}

adventOfCode() {
    local invokedPath="$0"
    local fullPath="$(realpath "$0")"
    local realRoot="$(dirname "$fullPath")"

    cd "$realRoot"
    git rev-parse --is-inside-work-tree &>/dev/null
    if [ $? -ne 0 ]; then
      echo "Script '$0' resolved to '$fullPath', which is not in a git repository." >&2
      exit 1
    fi
    local tot="$(git rev-parse --show-toplevel)"
    cd "$tot"

    local year="$(date +"%Y")"
    cd "./$year" || exit 2

    if [[ "x$1" == "x" ]]; then
      echo "Need to specify a day!" >&2
      exit 3
    fi

    if [[ (! -f "day${1}.mjs") || (! -f "input/day${1}.txt") || (! -s "input/day${1}.txt") || ( $(wasTooEarly "input/day${1}.txt") -eq 0 ) ]]; then
      ./prep.sh "$1"
      if [[ $? -ne 0 ]]; then
        exit 1
      fi
      echo
    fi

    node "day${1}.mjs";
}

adventOfCode $@