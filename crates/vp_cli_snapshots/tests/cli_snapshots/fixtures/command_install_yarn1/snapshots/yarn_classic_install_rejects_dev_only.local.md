# yarn_classic_install_rejects_dev_only

## `vp install -D`

reject dev-only install before installing the production dependency

**Exit code:** 1

```
yarn does not support --dev.
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vp install --dev --frozen-lockfile`

dev-only install is also rejected in frozen-lockfile mode

**Exit code:** 1

```
yarn does not support --dev.
```

## `vp install --dev --resolution-only --silent`

silent mode still reports all unsupported options

**Exit code:** 1

```
yarn does not support --dev.
yarn does not support --resolution-only.
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
