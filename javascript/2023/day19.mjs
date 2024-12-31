import aoc from './aoc.mjs';

const part1expected = 19114;
const part2expected = 167_409_079_868_000;

/** @typedef {{x: number, m: number, a: number, s: number}} Part */
/** @typedef {{ always: boolean, condition: (part: Part) => boolean, field?: string, op?: string, cmp?: number, action: string}} Rule */
/** @typedef {[start: number, end: number]} FieldRange */
/** @typedef {{ x: FieldRange, m: FieldRange, a: FieldRange, s: FieldRange }} PartRange */
/** @typedef {{ workflows: Record<string, Rule[]>, parts: Part[] }} ParsedInput */

/**
 * @param {string[]} lines 
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const splitPoint = lines.indexOf('');

  const workflows = lines.slice(0, splitPoint).map((rawWorkflow) => {
    const [id, rest] = rawWorkflow.split('{');
    const rules = rest.substring(0, rest.length - 1).split(',').map((rawRule) => {
      if (rawRule === 'A') return { always: true, condition: () => true, action: 'ACCEPT' };
      if (rawRule === 'R') return { always: true, condition: () => true, action: 'REJECT' };
      if (rawRule.indexOf(':') !== -1) {
        const [left, right] = rawRule.split(':');
        const [,field, op, cmp] = left.match(/^([xmas])([<>])(\d+)$/);
        return {
          always: false,
          field, op, cmp: +cmp,
          condition: (part) => {
            const f = part[field];
            const d = cmp;
            if (op === '<' && f < d) return true;
            if (op === '>' && f > d) return true;
            return false;
          },
          action: {'A': 'ACCEPT', 'R': 'REJECT'}[right] ?? right,
        };
      } else {
        return { always: true, condition: () => true, action: rawRule };
      }
    });
    
    return { id, rules };
  }).reduce((a, c) => ({ ...a, [c.id]: c.rules }), {});

  let parts = lines.slice(splitPoint + 1).map((rawPart) => {
    const fields = rawPart.substring(1, rawPart.length - 1).split(',').map(f => f.split('='));
    return fields.reduce((a, c) => ({ ...a, [c[0]]: +c[1] }), {});
  });

  return { workflows, parts };
};

/**
 * @param {ParsedInput} parsedInput
 * @returns number
 */
const part1 = ({ workflows, parts }) => {
  if (!workflows['in']) throw new Error('There was no rule named "in", that would be against the rules');

  const accepted = parts.filter(part => {
    let flow = workflows['in'];
    flowLoop: while (true) {
      for (const rule of flow) {
        if (rule.condition(part)) {
          switch (rule.action) {
            case 'ACCEPT':
              return true;
            case 'REJECT':
              return false;
            default:
              flow = workflows[rule.action];
              continue flowLoop;
          }
        }
      }
      throw new Error(`Workflow reached the end of its rules without jumping, accepting, or rejecting!`);
    }
  });

  return accepted.map(part => Object.values(part).reduce((a, c) => a + c, 0)).reduce((a, c) => a + c, 0);
};

/**
 * DFS into the tree of rules, slicing out parts of a range as we follow
 * rules leading down to an acceptor.
 * When we find one, we count the number of combinations the reduced range has.
 * Adding all of those up gives us the answer!
 *
 * By slicing ranges up in this way, we avoid any mess with duplicates or reconciliation.
 * Which is what my first attempt was doing, and it was overcomplicating things.
 *
 * @param {ParsedInput} parsedInput
 * @returns number
 */
const part2 = ({ workflows }) => {
  let out = 0;

  /** @type {{ flowId: string, range: PartRange }[]} */
  let stack = [{ flowId: 'in', range: { x: [1, 4000], m: [1, 4000], a: [1, 4000], s: [1, 4000] }}];

  stackLoop: while (stack.length) {
    const { flowId, range } = stack.pop();
    if (flowId === 'ACCEPT') {
      out += Object.values(range).reduce((a, [lo, hi]) => a * (hi - lo + 1), 1);
      continue;
    } else if (flowId === 'REJECT') {
      continue;
    }

    for (const rule of workflows[flowId]) {
      if (rule.always) {
        stack.push({ flowId: rule.action, range });
        continue stackLoop;
      }
      const { field, op, cmp, action } = rule;
      const newRange = {
        x: [...range.x],
        m: [...range.m],
        a: [...range.a],
        s: [...range.s],
        [field]: op === '>' ? [cmp + 1, range[field][1]] : [range[field][0], cmp - 1],
      };
      range[field][op === '>' ? 1 : 0] = cmp; // Will affect processing the remaining rules in this flow
      stack.push({ flowId: action, range: newRange });
    }
  }
  return out;
};

aoc(2023, 19, part1, part1expected, part2, part2expected, parse);
