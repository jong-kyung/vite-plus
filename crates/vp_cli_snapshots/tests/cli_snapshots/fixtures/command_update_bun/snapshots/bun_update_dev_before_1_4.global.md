# bun_update_dev_before_1_4

## `vp update --dev --latest`

reject unsupported --dev before updating any dependency

**Exit code:** 1

```
bun < 1.4 does not support --dev.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-bun",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*"
  },
  "packageManager": "bun@1.3.11"
}
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```

## `vp update --dev --workspace-root --filter web`

report every unsupported option together

**Exit code:** 1

```
bun < 1.4 does not support --filter.
bun does not support --workspace-root.
bun < 1.4 does not support --dev.
```
