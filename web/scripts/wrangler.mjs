// Runs this project's Wrangler for the maintainer's deploys and for local previews.
// The same file is used by every kitao.dev site; keep the copies identical.
import { spawn as spawnChild, spawnSync as spawnChildSync } from 'node:child_process';
import { once } from 'node:events';
import { realpathSync } from 'node:fs';
import { devNull } from 'node:os';
import { fileURLToPath } from 'node:url';

const project = fileURLToPath(new URL('..', import.meta.url));
const wrangler = fileURLToPath(
  new URL('../node_modules/wrangler/bin/wrangler.js', import.meta.url),
);
const config = fileURLToPath(new URL('../wrangler.jsonc', import.meta.url));

// Any other flag could change the account, the configuration, or the environment files.
export function wranglerArguments([command, ...flags]) {
  if (command === 'deploy' && flags.length === 1 && ['--dry-run', '--strict'].includes(flags[0]))
    return [command, ...flags];
  const port = Number(flags[1]);
  const validPort = Number.isInteger(port) && port > 0 && port < 65536;
  // Dev hotkeys could open a public tunnel or switch to remote mode, so they are off.
  if (command === 'dev' && flags.length === 2 && flags[0] === '--port' && validPort)
    return [
      command,
      '--local',
      '--ip',
      '127.0.0.1',
      '--port',
      String(port),
      '--show-interactive-dev-session=false',
    ];
  throw new Error('Use deploy --dry-run, deploy --strict, or dev --port <port>.');
}

// Cloudflare credentials come only from OAuth in the OS keychain.
export function wranglerEnvironment(env) {
  const names = Object.keys(env).filter((name) => env[name]);
  if (names.some((name) => /^(CLOUDFLARE|CF)_API_(TOKEN|KEY)$/i.test(name)))
    throw new Error('Use Cloudflare OAuth with the OS keychain. Unset API token/key variables.');
  return {
    // Settings such as WRANGLER_TOKEN_URL or CLOUDFLARE_API_BASE_URL could redirect the token.
    ...Object.fromEntries(
      Object.entries(env).filter(([name]) => !/^(CLOUDFLARE|CF|WRANGLER)_/i.test(name)),
    ),
    // Fail when the keychain is unavailable instead of falling back to a plaintext file.
    CLOUDFLARE_AUTH_USE_KEYRING: 'true',
    WRANGLER_SEND_METRICS: 'false',
    WRANGLER_WRITE_LOGS: 'false',
  };
}

// Wrangler's own reader, so wrangler.jsonc may hold comments.
async function readConfig() {
  const { experimental_readRawConfig } = await import('wrangler');
  return experimental_readRawConfig({ config }).rawConfig;
}

// A deploy publishes only committed site files: this project and the sources the build watches.
export function siteStatusArguments(rawConfig) {
  const watched = [rawConfig.build?.watch_dir ?? []].flat();
  return ['status', '--porcelain', '--untracked-files=all', '--', '.', ...watched];
}

export async function runWrangler(
  args,
  { env = process.env, spawn = spawnChild, spawnSync = spawnChildSync, read = readConfig } = {},
) {
  const forwarded = wranglerArguments(args);
  const environment = wranglerEnvironment(env);
  if (forwarded.includes('--strict')) {
    const status = spawnSync('git', siteStatusArguments(await read()), {
      cwd: project,
      encoding: 'utf8',
      // Inherited variables such as GIT_DIR must not point the check at another repository.
      env: Object.fromEntries(Object.entries(env).filter(([name]) => !/^GIT_/i.test(name))),
    });
    if (status.error || status.status !== 0)
      throw new Error(`git status failed: ${status.error?.message ?? status.stderr.trim()}`);
    if (status.stdout.trim())
      throw new Error(`Commit these files before deploying:\n${status.stdout.trimEnd()}`);
  }
  // The pinned config ignores wrangler.json in parent directories and deploy redirects,
  // and the empty env file keeps Wrangler from loading .env and .env.local.
  const options = ['--config', config, '--env-file', devNull];
  const child = spawn(process.execPath, [wrangler, ...forwarded, ...options], {
    cwd: project,
    stdio: 'inherit',
    env: environment,
  });
  // Stopping this process also stops Wrangler, so no preview keeps serving on its own.
  const signals = ['SIGINT', 'SIGTERM', 'SIGHUP'];
  const forward = (signal) => child.kill(signal);
  for (const signal of signals) process.on(signal, forward);
  try {
    const [code] = await once(child, 'exit');
    return code ?? 1;
  } finally {
    for (const signal of signals) process.off(signal, forward);
  }
}

if (process.argv[1] && realpathSync(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    process.exitCode = await runWrangler(process.argv.slice(2));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
