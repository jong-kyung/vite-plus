const assert = require('node:assert/strict');
const { createRequire } = require('node:module');
const { resolve } = require('node:path');
for (const [index, dir] of ['.', 'packages/app', 'packages/web'].entries()) {
  const requireFrom = createRequire(resolve(dir, 'package.json'));
  assert.equal(requireFrom('is-number/package.json').version, process.argv[index + 2], dir);
}
assert.equal(require('./node_modules/yocto-queue/package.json').version, '0.1.0');
