import aoc from './aoc.mjs';

const part1expected = 19114;
const part2expected = 167_409_079_868_000;

const parse = (lines, whichStar) => {
  const splitPoint = lines.indexOf('');

  const workflows = lines.slice(0, splitPoint).map((rawWorkflow) => {
    const [id, rest] = rawWorkflow.split('{');
    const rules = rest.substring(0, rest.length - 1).split(',').map((rawRule) => {
      if (rawRule === 'A') return { condition: () => true, action: 'ACCEPT' };
      if (rawRule === 'R') return { condition: () => true, action: 'REJECT' };
      if (rawRule.indexOf(':') !== -1) {
        const [left, right] = rawRule.split(':');
        const [,field, op, cmp] = left.match(/^([xmas])([<>])(\d+)$/);
        return {
          condition: (part) => {
            const f = +part[field];
            const d = +cmp;
            if (op === '<' && f < d) return true;
            if (op === '>' && f > d) return true;
            return false;
          },
          action: {'A': 'ACCEPT', 'R': 'REJECT'}[right] ?? right,
        };
      } else {
        return { condition: () => true, action: rawRule };
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

const part1 = ({ workflows, parts }) => {
  if (!workflows['in']) throw new Error('There was no rule named "in", that would be against the rules');

  const accepted = [];
  partLoop: for (const part of parts) {
    // console.log('ANALYZE part', part);
    let flow = workflows['in'];
    flowLoop: while (true) {
      // console.log('SWITCH WORKFLOW');
      for (const rule of flow) {
        // console.log('  NEXT RULE');
        if (rule.condition(part)) {
          switch (rule.action) {
            case 'ACCEPT':
              // console.log('  ACCEPT part:', part);
              accepted.push(part);
              continue partLoop;
            case 'REJECT':
              // console.log('  REJECT part:', part);
              continue partLoop;
            default:
              // console.log('  JUMP to another workflow:', rule.action);
              flow = workflows[rule.action];
              continue flowLoop;
          }
        }
      }
      throw new Error(`Workflow reached the end of its rules without jumping, accepting, or rejecting!`);
    }
  }

  return accepted.map(part => Object.values(part).reduce((a, c) => a + c, 0)).reduce((a, c) => a + c, 0);
};

const part2 = (data) => {
  return 'NYI';
};

aoc(2023, 19, part1, part1expected, part2, part2expected, parse);
