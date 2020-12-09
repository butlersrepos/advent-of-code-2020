const fs = require('fs');

function crunchNums1(nums) {
  for (num1 of nums) {
    for (num2 of nums) {
      if (num1 + num2 == 2020) {
        return [num1, num2];
      }
    }
  }
}

function crunchNums2(nums) {
  for (num1 of nums) {
    for (num2 of nums) {
      for (num3 of nums) {
        if (num1 + num2 + num3 == 2020) {
          return [num1, num2, num3];
        }
      }
    }
  }
}

const contents = fs.readFileSync('day-01-input.txt', { encoding: 'utf8' });
const nums = contents.split('\n').map(x => parseInt(x));

const answer1 = crunchNums1(nums);
console.log(`Part 1 - ${answer1[0]} & ${answer1[1]} = ${answer1[0] * answer1[1]}`);

const answer2 = crunchNums2(nums);
console.log(`Part 2 - ${answer2[0]} & ${answer2[1]} & ${answer2[2]} = ${answer2[0] * answer2[1] * answer2[2]}`);
