function runBlueprint(p, timeLimit) {
  // Since we can only make one bot per turn, we do not have any reason to produce more resources than any one bot costs.
  const maxBots = {
    ore: Math.max(p.botCosts.ore.ore, p.botCosts.clay.ore, p.botCosts.obsidian.ore, p.botCosts.geode.ore),
    clay: p.botCosts.obsidian.clay,
    obsidian: p.botCosts.geode.obsidian,
    geode: Infinity,
  };
  
  // Heuristic to rank how good a potential future state is. At each minute in the simulation, we generate every
  // possible next state (one for each bot that we can make, one for not making any bots), then we cull the least
  // desirable states and repeat this until time is up. Then we pick the state that has the most geodes.
  const score = (t, state) => (
    ((state.materials.geode + (timeLimit - t) * state.bots.geode) << 16)
    + (state.bots.obsidian << 8)
    + (state.bots.clay << 4)
    + state.bots.ore
  );

  let queue = [
    { materials: { ore: 0, clay: 0, obsidian: 0, geode: 0 }, bots: { ore: 1, clay: 0, obsidian: 0, geode: 0 } },
    { materials: { ore: 1, clay: 0, obsidian: 0, geode: 0 }, bots: { ore: 1, clay: 0, obsidian: 0, geode: 0 } },
  ];

  for (let t = 0; t < timeLimit - 1; t++) { // Since we queue the state _after_ the current one, we stop one early
    const nextQueue = [];
    for (const qr of queue) {
      for (const bt of ['geode', 'obsidian', 'clay', 'ore']) {
        if (
          qr.bots[bt] >= maxBots[bt]
          || qr.materials.ore < p.botCosts[bt].ore
          || qr.materials.clay < p.botCosts[bt].clay
          || qr.materials.obsidian < p.botCosts[bt].obsidian
        ) continue;

        const nextState = {
          materials: {
            ore: qr.materials.ore + qr.bots.ore - p.botCosts[bt].ore,
            clay: qr.materials.clay + qr.bots.clay - p.botCosts[bt].clay,
            obsidian: qr.materials.obsidian + qr.bots.obsidian - p.botCosts[bt].obsidian,
            geode: qr.materials.geode + qr.bots.geode,
          },
          bots: {
            ore: qr.bots.ore + (bt === 'ore'),
            clay: qr.bots.clay + (bt === 'clay'),
            obsidian: qr.bots.obsidian + (bt === 'obsidian'),
            geode: qr.bots.geode + (bt === 'geode'),
          },
        }
        nextQueue.push(nextState);
      }
      // Also consider not building a bot
      nextQueue.push({
        materials: {
          ore: qr.materials.ore + qr.bots.ore,
          clay: qr.materials.clay + qr.bots.clay,
          obsidian: qr.materials.obsidian + qr.bots.obsidian,
          geode: qr.materials.geode + qr.bots.geode,
        },
        bots: { ...qr.bots },
      });
    }
    nextQueue.sort((a, b) => score(t, b) - score(t, a));
    // Only keep the best options we've seen so far.
    // Some of the best options at the end have a slow start, so we need to keep a pretty big buffer not to cull those early.
    // For my input, 50k wasn't a big enough buffer to keep the best path, but 75k was plenty.
    // This number is the biggest factor in runtime - if we only keep 5k, the solution takes 0.2sec. With 75k, it takes over a minute.
    queue = nextQueue.slice(0, 75_000);
  }

  queue.sort((a, b) => score(timeLimit, b) - score(timeLimit, a));

  return { ...p, ...queue[0] };
}

(await import('./aoc.mjs')).default(
  2022, 19,
  (blueprints) => blueprints.map(p => runBlueprint(p, 24)).reduce((a, p) => a + (p.id * p.materials.geode), 0), 33,
  (blueprints) => blueprints.slice(0, 3).map(p => runBlueprint(p, 32)).reduce((a, p) => a * p.materials.geode, 1), 3472,
  data => data.map(line => {
    const matched = line.match(
      /^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$/
    );
    return {
      id: +matched[1],
      botCosts: {
        ore:      { ore: +matched[2], clay: 0,           obsidian: 0 },
        clay:     { ore: +matched[3], clay: 0,           obsidian: 0 },
        obsidian: { ore: +matched[4], clay: +matched[5], obsidian: 0 },
        geode:    { ore: +matched[6], clay: 0,           obsidian: +matched[7] },
      },
      bots:      { ore: 1, clay: 0, obsidian: 0, geode: 0 },
      materials: { ore: 0, clay: 0, obsidian: 0, geode: 0 },
    };
  })
);