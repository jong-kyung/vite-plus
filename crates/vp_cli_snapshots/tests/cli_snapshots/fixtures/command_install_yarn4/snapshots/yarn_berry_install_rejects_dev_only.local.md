# yarn_berry_install_rejects_dev_only

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

report all unsupported options, including Berry's existing silent restriction

**Exit code:** 1

```
yarn does not support --dev.
yarn does not support --resolution-only.
yarn >= 2 does not support --silent.
```

## `vpt print-file package.json`

```
{
  "name": "command-install-yarn4",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "yarn@4.16.0"
}
```
