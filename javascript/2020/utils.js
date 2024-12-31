const fs = require('fs');

function fromRaw(raw, parse = false, delim = '\n') {
  return String(raw).split(delim).map(l => l.trim()).filter(l => l).map(l => parse ? JSON.parse(l) : l);
}

function getInput(day, parse = false, delim = '\n') {
  const raw = fs.readFileSync(`./input/day${day}.txt`);
  return fromRaw(raw.toString(), parse, delim);
};

exports.fromRaw = fromRaw;
exports.getInput = getInput;