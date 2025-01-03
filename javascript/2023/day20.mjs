import aoc from './aoc.mjs';

const part1expected = 11_687_500;
const part2expected = 'N/A'; // The sample doesn't have an rx module attached

/** @typedef {'lo'|'hi'} PulseType */
/** @typedef {'broadcaster'|'flipflop'|'conjunction'|'sink'} ModuleType */
/** @typedef {{ type: ModuleType, id: string, outs: string[], lastState: string, lastIns: Map<string, boolean> }} Module */

/**
 * @param {string[]} data
 * @param {1|2} part
 * @returns {Map<string,Module>}
 */
const parse = (data, part) => {
  /** @type Map<string,Module> */
  const modules = new Map();
  /** @type Set<string> */
  const seenOuts = new Set();
  for (const line of data) {
    const [left, right] = line.split(' -> ');
    let type;
    let id;
    switch (left.substring(0, 1)) {
      case 'b':
        type = 'broadcaster';
        id = 'broadcaster';
        break;
      case '%':
        type = 'flipflop';
        id = left.substring(1);
        break;
      case '&':
        type = 'conjunction';
        id = left.substring(1);
        break;
      default:
        throw new Error(`Not sure what type this is: ${line}`);
    }
    const outs = right.split(', ');
    for (const out of outs) {
      seenOuts.add(out);
    }
    modules.set(id, { type, id, outs, lastState: 'lo', lastIns: new Map() });
  }
  for (const seen of seenOuts) {
    if (!modules.has(seen)) {
      modules.set(seen, { type: 'sink', id: seen, outs: [], lastState: 'lo', lastIns: new Map() });
    }
  }
  // All conjunction modules need to be aware of ALL of their possible inputs, so that it can treat them as lo by default
  // It can't _just_ check that all inputs it already received are hi, per the rules.
  for (const module of modules.values()) {
    for (const out of module.outs) {
      if (modules.get(out)?.type === 'conjunction') {
        modules.get(out).lastIns.set(module.id, 'lo');
      }
    }
  }
  return modules;
};

/**
 * Does two things:
 * - Mutates the target module by running the given pulse into it
 * - Returns a map of other modules and which signal it should send them after the operation
 *
 * @param {Map<string,Module>} modules 
 * @param {string} target 
 * @param {PulseType} pulseType
 * @param {string} source Used by conjunction modules to decide what they'll output, 
 * @returns {Map<string, PulseType>} Which modules will receive which signals as a result.
 */
function applySignal(modules, target, pulseType, source) {
  const module = modules.get(target);
  if (!module) throw new Error(`Target module "${target}" doesn't exist!`);

  /** @type Map<string, PulseType> */
  const next = new Map();
  
  switch (module.type) {
    case 'broadcaster':
      for (const out of module.outs) {
        next.set(out, pulseType);
      }
      break;
    case 'flipflop':
      if (pulseType === 'lo') {
        module.lastState = module.lastState === 'lo' ? 'hi' : 'lo';
        for (const out of module.outs) {
          next.set(out, module.lastState);
        }
      }
      break;
    case 'conjunction':
      module.lastIns.set(source, pulseType);
      const toSend = [...module.lastIns.values()].every(v => v === 'hi') ? 'lo' : 'hi';
      module.lastState = toSend;
      // if (target === 'dh' && modules.get(target).lastState === 'hi' && module.lastState === modules.get(target).lastState) console.log('dh should have just emitted hi');
      for (const out of module.outs) {
        next.set(out, toSend);
      }
      break;
    case 'sink':
      // Nowhere to send it, so don't.
      break;
    default:
      throw new Error(`Module "${target}" has an unhandled type: ${module.type}`);
  }

  return next;
}

/**
 * @param {Map<string,Module>} modules 
 */
function buttonPress(modules, watchThese = []) {
  let countLoSent = 0;
  let countHiSent = 0;
  const cyclesFound = watchThese.reduce((a, c) => ({ ...a, [c]: false }), {});
  let stack = [{ send: 'lo', to: 'broadcaster', from: 'button' }];
  while (stack.length > 0) {
    const { send, to, from } = stack.shift(); // Shift because rules ask for BFS explicitly
    if (watchThese.includes(from) && send === 'hi') {
      cyclesFound[from] = true;
    }
    if (send === 'lo') countLoSent++;
    if (send === 'hi') countHiSent++;
    const next = applySignal(modules, to, send, from);
    for (const [nextTo, nextSend] of next.entries()) {
      stack.push({ send: nextSend, to: nextTo, from: to });
    }
  }
  return { countLoSent, countHiSent, cyclesFound };
}

const part1 = (modules) => {
  let totalLoSent = 0;
  let totalHiSent = 0;
  for (let i = 0; i < 1_000; i++) {
    const { countLoSent, countHiSent } = buttonPress(modules);
    totalLoSent += countLoSent;
    totalHiSent += countHiSent;
  }
  return totalLoSent * totalHiSent;
};

// We're using these exactly the same way as we did in day 8, and for the same reason.
// There are cycles we need to detect, and then the answer is the lcm of their lengths.
const gcd = (a, b) => b === 0 ? a : gcd(b, a % b);
const lcm = (a, b) => a / gcd(a, b) * b;
const lcmAll = (...all) => all.reduce(lcm, 1);

/**
 * The problem description doesn't say it, but the input does.
 * Part 2 of this problem is exactly the same thing as day 8:
 * - We have a network of actions that can cycle
 * - There are several subnetworks, disconnected from each other
 * - We're waiting for their cycle lengths to align
 * - The answer is the lowest common multiple of their cycle lengths
 * 
 * As such, after a little bit of setup to find the subnetworks,
 * we just do the same thing we did in day 8 part 2.
 * 
 * Specifics:
 * - The rx module has only one input, which is a conjunction module.
 * - That conjunction module has exactly four inputs, each themselves a conjunction.
 * - Those four are all part of independent subnetworks, and they cycle after a given number of button presses.
 * - So, find those four conjunctions, start pressing the button until we find all of their cycle lengths.
 * - Once we know how long each one takes to flip hi, take the lcm of those lengths and return it.
 *
 * @param {Map<string, Module>} modules 
 * @param {boolean} isSample 
 * @returns {number}
 */
const part2 = (modules, isSample) => {
  // The sample doesn't have the same properties that make part 2 a puzzle.
  if (isSample) return 'N/A';

  // First, find the subnetworks that feed into rx.
  let rxParent = [...modules.keys()].find(k => modules.get(k).outs.includes('rx'));
  const networkLeaves = [...modules.keys()].filter(k => modules.get(k).outs.includes(rxParent));
  const loopCheck = networkLeaves.reduce((a, id) => ({ ...a,  [id]: { found: false, at: Infinity } }), {});

  // Sanity: the loops should all be found pretty quickly.
  for (let i = 1; i <= 100_000_000_000; i++) {
    const { cyclesFound } = buttonPress(modules, networkLeaves, i);
    for (const k of networkLeaves) {
      if (cyclesFound[k]) {
        if (!loopCheck[k].found) {
          loopCheck[k].found = true;
          loopCheck[k].at = i;
        }
      }
    }
    if (networkLeaves.every(k => loopCheck[k].found)) {
      return lcmAll(...networkLeaves.map(k => loopCheck[k].at));
    }
  }

  throw new Error('Module rx never got a lo signal after 100 billion button presses. Something is wrong.');
};

aoc(2023, 20, part1, part1expected, part2, part2expected, parse);
