# bun_update_workspace_rejected

## `vp update --workspace`

Bun 1.4 still cannot restrict updates to workspace dependencies

**Exit code:** 1

```
bun does not support --workspace.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-bun14",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "is-number": "6.0.0"
  },
  "devDependencies": {
    "yocto-queue": "0.1.0"
  },
  "optionalDependencies": {
    "isarray": "1.0.0"
  },
  "packageManager": "bun@1.4.0"
}
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
