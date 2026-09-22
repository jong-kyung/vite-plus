# bun_install_rejects_dev_only

## `vp install -D`

reject dev-only install before installing the production dependency

**Exit code:** 1

```
bun does not support --dev.
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```

## `vp install --dev --frozen-lockfile`

dev-only install is also rejected in frozen-lockfile mode

**Exit code:** 1

```
bun does not support --dev.
```

## `vp install --dev --prefer-offline --silent`

silent mode still reports all unsupported options

**Exit code:** 1

```
bun does not support --dev.
bun does not support --prefer-offline.
```

## `vpt print-file package.json`

```
{
  "name": "command-install-bun",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "bun@1.4.0"
}
```
