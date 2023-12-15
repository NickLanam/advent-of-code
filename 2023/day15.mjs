import aoc from './aoc.mjs';

const part1expected = 1320;
const part2expected = 145;

const parse = (data) => data[0].split(',');

const hash = (id) => id.split('').reduce((a, c) => ((a + c.charCodeAt(0)) * 17) % 256, 0);

const part1 = (instructions) => {
  return instructions.map(chars => hash(chars)).reduce((a, c) => a + c, 0);
};

const part2 = (instructions) => {
  const boxes = Array(256).fill(0).map(() => []);
  for (const instruction of instructions) {
    if (instruction.endsWith('-')) {
      const id = instruction.replace(/\-$/, '');
      const boxId = hash(id);
      const existing = boxes[boxId].findIndex(([eid]) => eid === id);
      if (!Number.isNaN(existing) && existing >= 0) {
        boxes[boxId].splice(existing, 1);
      }
    } else if (instruction.includes('=')) {
      let [id, strength] = instruction.split('=');
      strength = +strength;
      const boxId = hash(id);
      const existing = boxes[boxId].findIndex(([eid]) => eid === id);
      if (!Number.isNaN(existing) && existing >= 0) {
        boxes[boxId][existing][1] = strength;
      } else {
        boxes[boxId].push([id, strength]);
      }
    }
  }
  return boxes.reduce((boxSum, box, b) => boxSum + (
    box.reduce((lensSum, [, strength], slot) => lensSum + (strength * (slot + 1) * (b + 1)), 0)
  ), 0);
};

aoc(2023, 15, part1, part1expected, part2, part2expected, parse);
