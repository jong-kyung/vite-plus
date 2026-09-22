# yarn_update_selectors_rejected

## `vp update -D -P --no-optional`

report every unsupported dependency selector before updating

**Exit code:** 1

```
yarn does not support --dev.
yarn does not support --prod.
yarn does not support --no-optional.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-yarn4",
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
  "packageManager": "yarn@4.10.3"
}
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
