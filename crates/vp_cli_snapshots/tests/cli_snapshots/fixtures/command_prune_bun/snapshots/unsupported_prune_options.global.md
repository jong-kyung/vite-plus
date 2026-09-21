# unsupported_prune_options

## `vpt json-edit package.json packageManager bun@1.3.11`


## `vp pm prune`

Without named options, retain the existing command-level Noop.

```
warn: bun prune requires bun >= 1.4. bun install will prune extraneous packages automatically.
```

## `vp pm prune --prod --no-optional -- --help`

An explicit request must fail instead of reporting a successful Noop.

**Exit code:** 1

```
bun < 1.4 does not support --prod.
bun < 1.4 does not support --no-optional.
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
