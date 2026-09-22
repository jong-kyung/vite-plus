const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');

for (const file of ['package.json', 'package-lock.json']) {
  assert.equal(readFileSync(file, 'utf8'), readFileSync(`${file}.before`, 'utf8'));
  console.log(`${file}: unchanged`);
}
