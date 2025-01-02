#!/usr/bin/env bash
adventOfCode() {
  cd "$(dirname "$(realpath "$0")")"
  exec cargo -q run -- $@
}

adventOfCode $@
