const assert = require('node:assert/strict');
const fs = require('node:fs');
const { createRequire } = require('node:module');
const { resolve } = require('node:path');
const prepare = process.argv[2] === 'prepare';
for (const dir of ['.', 'packages/app', 'packages/utils']) {
  const file = resolve(dir, 'package.json');
  if (prepare) {
    const manifest = JSON.parse(fs.readFileSync(file, 'utf8'));
    manifest.dependencies.testnpm2 = '^1.0.0';
    manifest.dependencies['@test/scoped'] = 'npm:testnpm2@^1.0.0';
    fs.writeFileSync(file, JSON.stringify(manifest, null, 2) + '\n');
    fs.copyFileSync(file, file + '.before');
  } else {
    assert.equal(fs.readFileSync(file, 'utf8'), fs.readFileSync(file + '.before', 'utf8'));
    const requireFrom = createRequire(file);
    assert.equal(requireFrom('testnpm2/package.json').version, process.argv[3] || '1.0.1');
    assert.equal(requireFrom('@test/scoped/package.json').version, process.argv[4] || '1.0.0');
    assert.equal(requireFrom('is-number/package.json').version, '6.0.0');
  }
}
if (prepare) {
  // Keep 1.0.0 installed and locked while widening its descriptor to an outdated range.
  let lock = fs.readFileSync('yarn.lock', 'utf8');
  assert.equal(lock.split('testnpm2: "npm:1.0.0"').length, 4);
  assert.equal(lock.split('"@test/scoped": "npm:testnpm2@1.0.0"').length, 4);
  const descriptor = '"@test/scoped@npm:testnpm2@1.0.0, testnpm2@npm:1.0.0":';
  assert.ok(lock.includes(descriptor));
  lock = lock.replaceAll('testnpm2: "npm:1.0.0"', 'testnpm2: "npm:^1.0.0"');
  lock = lock.replaceAll('"@test/scoped": "npm:testnpm2@1.0.0"', '"@test/scoped": "npm:testnpm2@^1.0.0"');
  lock = lock.replace(descriptor, '"@test/scoped@npm:testnpm2@^1.0.0, testnpm2@npm:^1.0.0":');
  fs.writeFileSync('yarn.lock', lock);
}
