# npm_install_rejects_dev_only

## `vpt cp package.json package.json.before`


## `vp install --dev`

reject dev-only install rather than installing every dependency type

**Exit code:** 1

```
npm does not support --dev.
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vp install --dev --fix-lockfile --silent`

silent mode still reports all unsupported options

**Exit code:** 1

```
npm does not support --dev.
npm does not support --fix-lockfile.
```

## `vp install --ignore-scripts`


## `vpt cp package-lock.json package-lock.json.before`


## `vpt mkdir node_modules`


## `vpt write-file node_modules/preserved.txt keep`


## `vp install --dev --frozen-lockfile --silent`

reject before npm ci can delete existing node_modules

**Exit code:** 1

```
npm does not support --dev.
```

## `vpt stat-file node_modules/preserved.txt --assert file`

```
node_modules/preserved.txt: file
```

## `node assert-unchanged.cjs`

```
package.json: unchanged
package-lock.json: unchanged
```
