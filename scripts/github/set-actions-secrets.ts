import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import dotenv from 'dotenv';

type ScriptOptions = {
  envFile: string;
  repo?: string;
  environment?: string;
  dryRun: boolean;
};

type SecretMap = Record<string, string>;

function parseArgs(argv: string[]): ScriptOptions {
  const args = [...argv];
  const getArg = (name: string): string | undefined => {
    const idx = args.indexOf(name);
    if (idx === -1) {
      return undefined;
    }
    return args[idx + 1];
  };

  const envFile = getArg('--env-file') ?? path.resolve(process.cwd(), '.env.test.remote');
  const repo = getArg('--repo');
  const environment = getArg('--environment');
  const dryRun = args.includes('--dry-run');

  return { envFile, repo, environment, dryRun };
}

function loadEnvFile(envFile: string): Record<string, string> {
  if (!fs.existsSync(envFile)) {
    throw new Error(`Env file not found: ${envFile}`);
  }
  const content = fs.readFileSync(envFile, 'utf8');
  return dotenv.parse(content);
}

function resolveSecrets(env: Record<string, string>): {
  required: SecretMap;
  optional: SecretMap;
} {
  const webhookSecretPlatform = env.STRIPE_WEBHOOK_SECRET_PLATFORM ?? '';
  const webhookSecretConnect = env.STRIPE_WEBHOOK_SECRET_CONNECT ?? '';

  const required: SecretMap = {
    PLAYWRIGHT_BASE_URL: env.PLAYWRIGHT_BASE_URL ?? '',
    VITE_API_BASE: env.VITE_API_BASE ?? env.TEST_BASE_URL ?? '',
    VITE_SUPABASE_URL: env.VITE_SUPABASE_URL ?? env.SUPABASE_URL ?? '',
    VITE_SUPABASE_ANON_KEY: env.VITE_SUPABASE_ANON_KEY ?? env.SUPABASE_ANON_KEY ?? '',
    SUPABASE_URL: env.SUPABASE_URL ?? env.VITE_SUPABASE_URL ?? '',
    SUPABASE_SERVICE_ROLE_KEY: env.SUPABASE_SERVICE_ROLE_KEY ?? '',
    TEST_USER_PASSWORD: env.TEST_USER_PASSWORD ?? '',
    TEST_RUNS_TOKEN: env.TEST_RUNS_TOKEN ?? '',
    STRIPE_SECRET_KEY: env.STRIPE_SECRET_KEY ?? '',
    STRIPE_WEBHOOK_SECRET_PLATFORM: webhookSecretPlatform,
    STRIPE_WEBHOOK_SECRET_CONNECT: webhookSecretConnect,
    VERCEL_PROTECTION_BYPASS:
      env.VERCEL_PROTECTION_BYPASS ??
      env.VERCEL_AUTOMATION_BYPASS_SECRET ??
      env.VERCEL_BYPASS_TOKEN ??
      '',
  };

  const optional: SecretMap = {};
  if (env.TEST_BASE_URL) {
    optional.TEST_BASE_URL = env.TEST_BASE_URL;
  }
  if (env.SUPABASE_ANON_KEY) {
    optional.SUPABASE_ANON_KEY = env.SUPABASE_ANON_KEY;
  }
  if (env.PLAYWRIGHT_ENV) {
    optional.PLAYWRIGHT_ENV = env.PLAYWRIGHT_ENV;
  }

  return { required, optional };
}

function assertGhAuth(): void {
  const result = spawnSync('gh', ['auth', 'status'], { stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error('GitHub CLI not authenticated. Run: gh auth login');
  }
}

function setSecret(
  name: string,
  value: string,
  options: { repo?: string; environment?: string; dryRun: boolean }
): void {
  const args = ['secret', 'set', name, '--body', value];
  if (options.repo) {
    args.push('--repo', options.repo);
  }
  if (options.environment) {
    args.push('--env', options.environment);
  }

  if (options.dryRun) {
    console.log(`[dry-run] Would set ${name}`);
    return;
  }

  const result = spawnSync('gh', args, { stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error(`Failed to set secret: ${name}`);
  }
}

function validateSecrets(required: SecretMap): void {
  const missing = Object.entries(required)
    .filter(([, value]) => !value)
    .map(([key]) => key);
  if (missing.length > 0) {
    throw new Error(`Missing required secrets in env file: ${missing.join(', ')}`);
  }
}

function resolveProductionSecrets(): SecretMap {
  const prodEnvFile = path.resolve(process.cwd(), '.env.production.remote');
  if (!fs.existsSync(prodEnvFile)) {
    console.log(`No ${prodEnvFile} found, skipping production secrets.`);
    return {};
  }
  const env = loadEnvFile(prodEnvFile);
  const secrets: SecretMap = {};
  if (env.DATABASE_URL) {
    secrets.SUPABASE_DATABASE_URL_PRODUCTION = env.DATABASE_URL;
  }
  return secrets;
}

function main(): void {
  const options = parseArgs(process.argv.slice(2));
  assertGhAuth();

  const env = loadEnvFile(options.envFile);
  const { required, optional } = resolveSecrets(env);
  validateSecrets(required);

  const productionSecrets = resolveProductionSecrets();
  const secrets = { ...required, ...optional, ...productionSecrets };
  const secretNames = Object.keys(secrets);

  console.log(`Setting ${secretNames.length} GitHub secrets from ${options.envFile}...`);
  for (const [name, value] of Object.entries(secrets)) {
    console.log(`- ${name}`);
    setSecret(name, value, options);
  }
  console.log('Done.');
}

main();
