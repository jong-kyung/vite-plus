# yarn_update_selectors_rejected

## `vp update -D -P --no-optional --workspace`

report every unsupported dependency selector before updating

**Exit code:** 1

```
yarn does not support --dev.
yarn does not support --prod.
yarn does not support --no-optional.
yarn does not support --workspace.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-yarn1",
  "version": "1.0.0",
  "private": true,
  "packageManager": "yarn@1.22.22"
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
