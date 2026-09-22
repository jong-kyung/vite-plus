// Yarn's native prompt has no Vite+ milestone, so signal after its first render.
const write = process.stdout.write;
let ready = false;
process.stdout.write = function (chunk, ...args) {
  const result = write.call(this, chunk, ...args);
  if (!ready && String(chunk).includes('Choose which packages to update.')) {
    ready = true;
    setImmediate(() => {
      const name = Buffer.from('checkbox:yarn-upgrade:ready').toString('base64url');
      write.call(process.stdout, `\x1b]2;pty-terminal-test:${'0'.repeat(32)}:${name}\x1b\\`);
    });
  }
  return result;
};
