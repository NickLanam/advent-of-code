import aoc from './aoc.mjs';

const part1expected = 11_687_500;
const part2expected = 'N/A'; // The sample doesn't have an rx module attached

/** @typedef {'lo'|'hi'} PulseType */
/** @typedef {'broadcaster'|'flipflop'|'conjunction'|'sink'} ModuleType */
/** @typedef {{ type: ModuleType, id: string, outs: string[], lastState: boolean, lastIns: Map<string, boolean> }} Module */

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
function buttonPress(modules) {
  let countLoSent = 0;
  let countHiSent = 0;
  let didRxGetLo = false; // Shawty
  let stack = [{ send: 'lo', to: 'broadcaster', from: 'button' }];
  while (stack.length > 0) {
    // console.log('Process...', stack[0]);
    const { send, to, from } = stack.shift(); // Shift because rules ask for BFS explicitly
    if (to === 'rx' && send  === 'lo') didRxGetLo = true;
    if (send === 'lo') countLoSent++;
    if (send === 'hi') countHiSent++;
    const next = applySignal(modules, to, send, from);
    for (const [nextTo, nextSend] of next.entries()) {
      stack.push({ send: nextSend, to: nextTo, from: to });
    }
  }
  return { countLoSent, countHiSent, didRxGetLo };
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

const part2 = (modules, isSample) => {
  // TODO: This does not complete in a reasonable time frame.
  // Looking at my input, it turns out there's a reason the example didn't have an example result:
  // - The rx module output to by only one other module, which is a conjunction
  // - That conjunction has exactly four inputs
  // - Those inputs all have their own, fully independent sub-networks it would seem
  // - So, the trick is to figure out how long each one of those takes to flip to hi,
  //   then take the lcm of those lengths (manually checked, at least one of those loops is clean with no offset)
  //   and return that (because when they all go hi at once, rx gets a lo signal).
  // - Don't get an off-by-one with this!
  // - And then find out if this is the same discovery everyone else made! Probably is.
  if (isSample) return 'N/A'; // The sample doesn't have an rx module attached
  for (let i = 1; i <= 1_000_000_000_000; i++) {
    const { didRxGetLo } = buttonPress(modules);
    if (didRxGetLo) return i;
  }
  throw new Error('Module rx never got a lo signal after a trillion button presses. Something is wrong.');
};

aoc(2023, 20, part1, part1expected, part2, part2expected, parse);
