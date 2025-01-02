#!/bin/bash

wasTooEarly() {
  grep -q "Please don't repeatedly request" "$1" 2>/dev/null
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
    cd "$tot/javascript"

    local year="${YEAR:-$(date +"%Y")}"
    cd "./$year" || exit 2

    if [[ "x$1" == "x" ]]; then
      echo "Need to specify a day!" >&2
      exit 3
    fi

    # In 2023, I started 0-padding the day number so that GitHub would sort properly.
    # So, need to detect if I'm using the new way or the old way here.
    local DAY="$1"
    local PADDED_DAY="$(printf "%02d" "$DAY")"

    if [[ \
      ( (! -f "day${DAY}.mjs") && (! -f "day${PADDED_DAY}.mjs") ) \
      || ( \
         ( (! -f "input/day${DAY}.txt") || (! -s "input/day${DAY}.txt") ) \
         && ( (! -f "input/day${PADDED_DAY}.txt") || (! -s "input/day${PADDED_DAY}.txt") ) \
      ) \
      || ( \
        ( $(wasTooEarly "input/day${DAY}.txt") -eq 0 ) \
        && ( $(wasTooEarly "input/day${PADDED_DAY}.txt") -eq 0 ) 
      )
    ]]; then
      ./prep.sh "$DAY"
      if [[ $? -ne 0 ]]; then
        exit 1
      fi
      echo
    fi

    if [ -s "day${PADDED_DAY}.mjs" ]; then
      node "day${PADDED_DAY}.mjs"
    else
      node "day${DAY}.mjs"
    fi
}

adventOfCode $@