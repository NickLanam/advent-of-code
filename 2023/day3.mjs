(await import('./aoc.mjs')).default(
  2023, 3,
  parsed => parsed.partNumbersSum, 4361,
  parsed => parsed.gearRatiosSum, 467835,
  (rawLines) => {
    const grid = rawLines.map(line => line.split(''));
    const gears = {};

    // Answer storage
    let partNumbersSum = 0;
    let gearRatiosSum = 0;

    // First pass: find the gears
    // Finding the other symbols happens as a side effect in the second pass
    for (let gy = 0; gy < grid.length; gy++) {
      for (let gx = 0; gx < grid[gy].length; gx++) {
        let cell = grid[gy][gx];
        if (cell === '*') {
          gears[`${gx},${gy}`] = [];
        }
      }
    }

    // Second pass: find the numbers and their positions
    let digitY = 0;
    let digitXs= [];
    for (let y = 0; y < grid.length; y++) {
      for (let x = 0; x < grid[y].length; x++) {
        if (!Number.isNaN(+grid[y][x])) {
          // Hold onto digits while they're coming in sequence.
          // Note: lines may start or end in a number, but there are no lines
          // that start with a number right after lines that end with one.
          digitY = y;
          digitXs.push(x);
        } else if (digitXs.length) {
          // We reached something other than a digit, assemble the held number
          const left = digitXs[0];
          const right = digitXs[digitXs.length - 1];
          const num = +grid[digitY].slice(left, right + 1).join('');
          if (Number.isNaN(num)) {
            console.error('Missed a spot', {y,left,right,num});
            throw new Error();
          }

          // Check the neighbors (including diagonals) of the number
          // Numbers with neighboring non-. symbols are part numbers
          // Asterisks ("gears" in the story) memoize neighbor numbers
          let hasNeighborSymbol = false;
          for (let ly = digitY - 1; ly <= digitY + 1; ly++) {
            for (let lx = left - 1; lx <= right + 1; lx++) {
              if (grid[ly]?.[lx] == null) continue;
              const cell = grid[ly][lx];
              if (cell === '*') {
                // Tell the found gear that it has this neighbor
                gears[`${lx},${ly}`].push(num);
              }
              if (Number.isNaN(+cell) && cell !== '.') {
                // Mark this number as a part number (only once)
                if (!hasNeighborSymbol) {
                  partNumbersSum += num;
                  hasNeighborSymbol = true;
                }
              }
            }
          }
          digitXs = [];
        }
      }
    }

    // Last step: for gears that have exacty two numeric neighbors,
    // the ratio is those numbers multiplied together
    for (const block of Object.values(gears)) {
      if (block.length === 2) {
        gearRatiosSum += block[0] * block[1];
      }
    }
    
    // Solutions to both part 1 and part 2
    return { partNumbersSum, gearRatiosSum };
  }
);
