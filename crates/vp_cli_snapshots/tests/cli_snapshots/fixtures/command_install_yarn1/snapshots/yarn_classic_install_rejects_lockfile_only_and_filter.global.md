# yarn_classic_install_rejects_lockfile_only_and_filter

## `vp install --lockfile-only --filter app --silent`

report both unsupported options even in silent mode

**Exit code:** 1

```
yarn < 2 does not support --lockfile-only.
yarn < 2 does not support --filter.
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt print-file package.json`

```
{
  "name": "command-install-yarn1",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "yarn@1.22.22"
}
```

## `vp install --ignore-scripts`

ordinary installation remains supported

```
VITE+ - The Unified Toolchain for the Web

yarn install <version>
info No lockfile found.
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
warning Ignored scripts due to flag.
success Saved lockfile.

Done in <duration>.
```

## `vpt stat-file node_modules/prod-only-fixture/package.json --assert file`

```
node_modules/prod-only-fixture/package.json: file
```
