# remove_selectors_rejected

## `vp remove --save-dev --save-optional --save-prod testnpm2`

reject all unsupported removal selectors without changing the project

**Exit code:** 1

```
npm does not support --save-dev.
npm does not support --save-optional.
npm does not support --save-prod.
```

## `vpt print-file package.json`

```
{
  "name": "command-remove-npm10",
  "version": "1.0.0",
  "packageManager": "npm@10.9.4"
}
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```
