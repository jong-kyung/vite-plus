# remove_selectors_rejected

## `vp remove --recursive testnpm2`

reject recursive removal instead of passing Classic's ineffective --all flag

**Exit code:** 1

```
yarn < 2 does not support --recursive.
```

## `vp remove --filter app testnpm2`

report unsupported workspace filtering before execution

**Exit code:** 1

```
yarn < 2 does not support --filter.
```

## `vp remove --save-dev --save-optional --save-prod --filter app --recursive testnpm2`

reject all unsupported removal selectors without changing the project

**Exit code:** 1

```
yarn does not support --save-dev.
yarn does not support --save-optional.
yarn does not support --save-prod.
yarn < 2 does not support --filter.
yarn < 2 does not support --recursive.
```

## `vpt print-file package.json`

```
{
  "name": "command-remove-yarn1",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "testnpm2": "1.0.0"
  },
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
