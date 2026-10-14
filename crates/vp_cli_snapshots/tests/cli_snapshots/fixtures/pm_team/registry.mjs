import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { readFileSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:http';

const { packageManager } = JSON.parse(readFileSync('package.json', 'utf8'));
const fallback = !['npm@11.12.0', 'pnpm@11.13.0'].includes(packageManager);
const requests = [];
let deny = false;
const server = createServer(async (request, response) => {
  let body = '';
  for await (const chunk of request) body += chunk;
  const url = new URL(request.url, 'http://localhost');
  requests.push({
    method: request.method,
    path: url.pathname,
    body: body ? JSON.parse(body) : null,
    auth: request.headers.authorization,
    otp: request.headers['npm-otp'],
  });
  response.setHeader('content-type', 'application/json');
  if (deny) {
    response.writeHead(403).end(JSON.stringify({ error: 'team operation denied' }));
  } else if (request.method === 'GET') {
    const name = url.pathname.endsWith('/user') ? 'alice' : 'developers';
    response.end(JSON.stringify(url.searchParams.get('format') === 'cli' ? [name] : [{ name }]));
  } else {
    response.end(JSON.stringify({ ok: true }));
  }
});
server.listen(0, '127.0.0.1');
await once(server, 'listening');
const registry = `http://127.0.0.1:${server.address().port}/`;
writeFileSync('.npmrc', `registry=${registry}\n//127.0.0.1:${server.address().port}/:_authToken=fixture-token\n`);

async function run(args, method, path, body = null, failure = false) {
  requests.length = 0;
  const argv = ['pm', 'team', ...args, '--json', '--registry', registry, '--otp', '123456'];
  console.log(`vp pm team ${args.join(' ')} --json --registry <registry> --otp <otp>`);
  const windows = process.platform === 'win32';
  const child = spawn(windows ? 'cmd.exe' : 'vp', windows ? ['/d', '/s', '/c', 'vp', ...argv] : argv, {
    env: { ...process.env, npm_config_update_notifier: 'false' },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => { stdout += chunk; });
  child.stderr.on('data', (chunk) => { stderr += chunk; });
  const [code, signal] = await once(child, 'close');
  assert.equal(signal, null, stderr);
  assert.equal(requests.length, 1, `${stdout}\n${stderr}`);
  assert.equal(requests[0].method, method);
  assert.equal(requests[0].path, path);
  assert.deepEqual(requests[0].body, body);
  assert.equal(requests[0].auth, 'Bearer fixture-token');
  if (method !== 'GET') assert.equal(requests[0].otp, '123456');
  assert.equal(stderr.includes('falling back to npm team'), fallback, stderr);
  if (failure) {
    assert.notEqual(code, 0);
    assert.match(stdout + stderr, /team operation denied/);
    console.log('Registry denial returned a non-zero exit code.');
  } else {
    assert.equal(code, 0, `${stdout}\n${stderr}`);
    process.stdout.write(stdout);
    if (fallback) console.log(stderr.trim());
  }
  console.log(`Verified ${method} ${path} with authentication${method === 'GET' ? '' : ' and OTP'}.`);
}

try {
  const teams = '/-/org/org/team';
  const team = '/-/team/org/developers';
  await run(['create', '@org:developers'], 'PUT', teams, { name: 'developers' });
  await run(['add', '@org:developers', 'alice'], 'PUT', `${team}/user`, { user: 'alice' });
  await run(['ls', '@org'], 'GET', teams);
  await run(['list', '@org:developers'], 'GET', `${team}/user`);
  await run(['remove', '@org:developers', 'alice'], 'DELETE', `${team}/user`, { user: 'alice' });
  await run(['destroy', '@org:developers'], 'DELETE', team);
  deny = true;
  await run(['create', '@org:developers'], 'PUT', teams, { name: 'developers' }, true);
} finally {
  server.close();
  server.closeAllConnections();
}
