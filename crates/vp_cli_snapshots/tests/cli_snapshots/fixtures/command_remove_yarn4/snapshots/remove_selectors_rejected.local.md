# remove_selectors_rejected

## `vp remove --save-dev --save-optional --save-prod testnpm2`

reject all unsupported removal selectors without changing the project

**Exit code:** 1

```
yarn does not support --save-dev.
yarn does not support --save-optional.
yarn does not support --save-prod.
```

## `vpt print-file package.json`

```
{
  "name": "command-remove-yarn4",
  "version": "1.0.0",
  "packageManager": "yarn@4.10.3"
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
