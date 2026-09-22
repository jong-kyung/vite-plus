# classic_add_filter_rejections

## `vp add testnpm2 --filter app`

reject unsupported workspace filtering before execution

**Exit code:** 1

```
yarn < 2 does not support --filter.
```

## `vp add testnpm2 --save-catalog --filter app --workspace -- --help`

include the filter error alongside all other unsupported named options

**Exit code:** 1

```
yarn does not support --save-catalog.
yarn < 2 does not support --filter.
yarn does not support --workspace.
```

## `vp install testnpm2 --save-catalog --filter app`

install with a package still uses add validation

**Exit code:** 1

```
yarn does not support --save-catalog.
yarn < 2 does not support --filter.
```

## `vpt print-file package.json`

```
{
  "name": "command-add-yarn1",
  "version": "1.0.0",
  "private": true,
  "packageManager": "yarn@1.22.22"
}
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```
