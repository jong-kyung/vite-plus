# remove_selectors_rejected

## `vp remove --save-dev --save-optional --save-prod testnpm2`

reject all unsupported removal selectors without changing the project

**Exit code:** 1

```
bun does not support --save-dev.
bun does not support --save-optional.
bun does not support --save-prod.
```

## `vpt print-file package.json`

```
{
  "name": "command-remove-bun",
  "version": "1.0.0",
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
