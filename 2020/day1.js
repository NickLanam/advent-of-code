const { getInput } = require('./utils');

const day1star1 = (() => {
  const nums = getInput(1, true);
  for (let i = 0; i < nums.length; i++) {
    for (let j = i; j < nums.length; j++) {
      if (nums[i] + nums[j] === 2020) return nums[i] * nums[j];
    }
  }
  throw new Error('No match found for d1s1');
})();

const day1star2 = (() => {
  const nums = getInput(1, true);
  for (let i = 0; i < nums.length; i++) {
    for (let j = i; j < nums.length; j++) {
      for (let k = j; k < nums.length; k++) {
        if (nums[i] + nums[j] + nums[k] === 2020) return nums[i] * nums[j] * nums[k];
      }
    }
  }
  throw new Error('No match found for d1s2');
})();

console.log('Star 1: ', day1star1);
console.log('Star 2: ', day1star2);