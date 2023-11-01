# Advent of Code 2023

My solutions for [Advent of Code 2023](https://adventofcode.com/2023).

## Setup

* Create an `input/` directory. This is `.gitignore`'d.
* Put a file named `cookie.txt` in it, containing your login cookie to Advent of Code.
  * To get this: open dev tools in your browser, to the network tab. Reload the page. Check the cookies for the first request that gets made. It should have `session: SOME_STRING`. The `SOME_STRING` part is what goes into `cookie.txt`.
* Run `prep.sh dayNum`, where `dayNum` ranges from 1-25. It does the following:
  * Downloads that day's personal input to `input/dayX.txt`.
  * Creates `input/dayX.sample.txt`, which you can dump that day's sample into.
  * If not already present, creates a `dayX.mjs` in this directory. Its contents are built from a template in `prep.sh`.
* Solve the challenge in code.
* `node dayX.mjs`.

## Implementation

* There's a utility file, `aoc.mjs`, that's imported in every solution. It:
  * Is a JS module (thus `.mjs`), as is every solution. This allows `import` syntax.
  * Exports a default function with a signature like:
    ```typescript
    export default function aoc<Input>(
      year: number, day: number,
      part1SolutionFunc: (data: Input) => any,
      part1ExpectedOutputForSample: any,
      part2SolutionFunc: (data: Input) => any,
      part2ExpectedOutputForSample: any,
      // If omitted, leaves the raw input alone.
      rawInputToUsefulInput?: (rawLines: string[]) => Input,
    ) {...}
    ```
  * When run, it loads `input/day${day}.sample.txt` and `input/day${day}.txt`, splits both on newlines, and trims
    leading and trailing newlines. Then it passes each through `rawInputToUsefulInput` above (if present).
    Then, it runs the given solver functions on the sample input and checks the output against the expectation.
    This outputs an assertion failure if the answer is wrong. Finally, it does the same with your real input.
  * If the real input is malformed, an error is printed. For example, if `prep.sh` was run before the puzzle unlocked, or the sample wasn't populated, then it wouldn't make sense to run the solution.

## Wishlist

Things I want to improve about this setup or this year's solutions, which I might get to before next year.

* `prep.sh` should be adjusted to work through a symlink (it checks `$0` but doesn't `readlink` that).
* A toplevel script for the whole repository that just does `"$(readlink -f "$0")/$(date +%Y)/prep.sh" $@` would be nice to symlink in `~/bin` so I don't have to edit the template again.
