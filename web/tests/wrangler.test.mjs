// Tests scripts/wrangler.mjs. The same file is used by every kitao.dev site; keep the copies identical.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spawnSync } from 'node:child_process';
import { EventEmitter } from 'node:events';
import { mkdtempSync, rmSync, symlinkSync } from 'node:fs';
import { devNull, tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  runWrangler,
  siteStatusArguments,
  wranglerArguments,
  wranglerEnvironment,
} from '../scripts/wrangler.mjs';

const project = fileURLToPath(new URL('..', import.meta.url));
const script = fileURLToPath(new URL('../scripts/wrangler.mjs', import.meta.url));
const config = fileURLToPath(new URL('../wrangler.jsonc', import.meta.url));
const wrangler = fileURLToPath(
  new URL('../node_modules/wrangler/bin/wrangler.js', import.meta.url),
);
// A fixed environment, so a token or override in the shell running the tests changes nothing.
const env = { PATH: '/usr/bin' };

// A stand-in for child_process.spawn whose process exits with `code` (null: only when killed).
function fakeSpawn(calls, code = 0) {
  return (command, args, options) => {
    const child = new EventEmitter();
    child.kill = (signal) => {
      calls.push({ killed: signal });
      setImmediate(() => child.emit('exit', null, signal));
    };
    calls.push({ command, args, options });
    if (code !== null) setImmediate(() => child.emit('exit', code, null));
    return child;
  };
}

test('only deploys and a local preview without hotkeys are passed to Wrangler', () => {
  assert.deepEqual(wranglerArguments(['deploy', '--dry-run']), ['deploy', '--dry-run']);
  assert.deepEqual(wranglerArguments(['deploy', '--strict']), ['deploy', '--strict']);
  assert.deepEqual(wranglerArguments(['dev', '--port', '4173']), [
    'dev',
    '--local',
    '--ip',
    '127.0.0.1',
    '--port',
    '4173',
    '--show-interactive-dev-session=false',
  ]);
  for (const args of [
    [],
    ['deploy'],
    ['deploy', '--strict', '--dry-run'],
    ['deploy', '--env-file', '.env'],
    ['deploy', '--envFile=.env'],
    ['deploy', '--config', 'other.jsonc'],
    ['dev'],
    ['dev', '--port', '0'],
    ['dev', '--port', '65536'],
    ['dev', '--port', '4173.5'],
    ['dev', '--port', '4173', '--remote'],
    ['login'],
    ['delete'],
  ])
    assert.throws(() => wranglerArguments(args), /Use deploy --dry-run/, args.join(' '));
});

test('the wrapper runs when started directly or through a symbolic link', (t) => {
  const directory = mkdtempSync(join(tmpdir(), 'wrangler-test-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const link = join(directory, 'wrangler.mjs');
  symlinkSync(script, link);
  // An unknown command stops the wrapper with an error before it starts Wrangler.
  for (const path of [script, link]) {
    const result = spawnSync(process.execPath, [path, 'login'], { encoding: 'utf8', env });
    assert.equal(result.status, 1, path);
    assert.match(result.stderr, /Use deploy --dry-run/, path);
  }
});

test('environment credentials are refused and endpoint or account overrides are dropped', () => {
  for (const name of ['CLOUDFLARE_API_TOKEN', 'CF_API_TOKEN', 'CLOUDFLARE_API_KEY', 'cf_api_key'])
    assert.throws(() => wranglerEnvironment({ [name]: 'test-placeholder' }), /keychain/, name);
  assert.deepEqual(
    wranglerEnvironment({
      PATH: '/usr/bin',
      CLOUDFLARE_API_TOKEN: '',
      WRANGLER_TOKEN_URL: 'https://example.invalid/token',
      CLOUDFLARE_API_BASE_URL: 'https://example.invalid/api',
      cf_account_id: 'test-placeholder',
      CLOUDFLARE_AUTH_USE_KEYRING: 'false',
      WRANGLER_SEND_METRICS: 'true',
    }),
    {
      PATH: '/usr/bin',
      CLOUDFLARE_AUTH_USE_KEYRING: 'true',
      WRANGLER_SEND_METRICS: 'false',
      WRANGLER_WRITE_LOGS: 'false',
    },
  );
});

test('Wrangler runs from this project with its config and an empty env file', async () => {
  const calls = [];
  assert.equal(await runWrangler(['deploy', '--dry-run'], { env, spawn: fakeSpawn(calls) }), 0);
  assert.equal(calls.length, 1);
  const [{ command, args, options }] = calls;
  assert.equal(command, process.execPath);
  assert.deepEqual(args, [
    wrangler,
    'deploy',
    '--dry-run',
    '--config',
    config,
    '--env-file',
    devNull,
  ]);
  assert.equal(options.cwd, project);
  assert.equal(options.stdio, 'inherit');
  assert.equal(options.env.CLOUDFLARE_AUTH_USE_KEYRING, 'true');
  await assert.rejects(runWrangler(['login'], { env, spawn: fakeSpawn(calls) }), /Use deploy/);
  await assert.rejects(
    runWrangler(['deploy', '--dry-run'], { env: { CF_API_TOKEN: 'x' }, spawn: fakeSpawn(calls) }),
    /keychain/,
  );
  assert.equal(calls.length, 1);
  assert.equal(await runWrangler(['deploy', '--dry-run'], { env, spawn: fakeSpawn([], 3) }), 3);
  const failing = () => {
    const child = new EventEmitter();
    setImmediate(() => child.emit('error', new Error('Spawn failed')));
    return child;
  };
  await assert.rejects(
    runWrangler(['deploy', '--dry-run'], { env, spawn: failing }),
    /Spawn failed/,
  );
});

// A time limit turns a missing signal forward into a failure instead of a hang.
test('stopping the wrapper stops Wrangler too', { timeout: 5000 }, async () => {
  const listeners = process.listenerCount('SIGTERM');
  const calls = [];
  const running = runWrangler(['dev', '--port', '4173'], { env, spawn: fakeSpawn(calls, null) });
  await new Promise(setImmediate);
  process.emit('SIGTERM', 'SIGTERM');
  assert.equal(await running, 1);
  assert.deepEqual(calls.at(-1), { killed: 'SIGTERM' });
  assert.equal(process.listenerCount('SIGTERM'), listeners);
});

test('the site files to check are this project and the sources the build watches', () => {
  const status = ['status', '--porcelain', '--untracked-files=all', '--', '.'];
  assert.deepEqual(siteStatusArguments({}), status);
  assert.deepEqual(siteStatusArguments({ build: { watch_dir: '../docs' } }), [
    ...status,
    '../docs',
  ]);
  assert.deepEqual(siteStatusArguments({ build: { watch_dir: ['a.html', 'assets'] } }), [
    ...status,
    'a.html',
    'assets',
  ]);
});

test('a strict deploy runs only when the site files are committed', async () => {
  const read = async () => ({ build: { watch_dir: ['index.html'] } });
  const deploy = async (gitResult, environment = env) => {
    const git = [];
    const calls = [];
    const spawnSync = (command, args, options) => {
      git.push({ command, args, options });
      return gitResult;
    };
    const code = await runWrangler(['deploy', '--strict'], {
      env: environment,
      spawn: fakeSpawn(calls),
      spawnSync,
      read,
    });
    return { code, git, calls };
  };
  const clean = await deploy({ status: 0, stdout: '' }, { PATH: '/usr/bin', GIT_DIR: '/x' });
  assert.equal(clean.code, 0);
  assert.equal(clean.calls.length, 1);
  const [git] = clean.git;
  assert.equal(git.command, 'git');
  assert.deepEqual(git.args, siteStatusArguments(await read()));
  assert.equal(git.options.cwd, project);
  // Inherited variables such as GIT_DIR must not redirect the check.
  assert.deepEqual(git.options.env, { PATH: '/usr/bin' });
  await assert.rejects(
    deploy({ status: 0, stdout: ' M index.html\n' }),
    /Commit these files[^]*index\.html/,
  );
  await assert.rejects(
    deploy({ status: 128, stdout: '', stderr: 'not a git repository' }),
    /not a git/,
  );
  // The real config is read with Wrangler's reader, which accepts comments in wrangler.jsonc.
  const checked = [];
  await runWrangler(['deploy', '--strict'], {
    env,
    spawn: fakeSpawn([]),
    spawnSync: (command, args) => {
      checked.push(args);
      return { status: 0, stdout: '' };
    },
  });
  assert.deepEqual(checked[0].slice(0, 5), siteStatusArguments({}));
});
