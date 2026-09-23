# npm_install_rejects_dev_only

## `vp install --dev`

reject dev-only install rather than installing every dependency type

**Exit code:** 1

```
VITE+ - The Unified Toolchain for the Web

npm does not support --dev.
```

## `vp install --dev --fix-lockfile --silent`

silent mode still reports all unsupported options

**Exit code:** 1

```
npm does not support --dev.
npm does not support --fix-lockfile.
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vp install --ignore-scripts`


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

## `vpt print-file package.json package-lock.json`

```
{
  "name": "command-install-npm11",
  "version": "1.0.0",
  "private": true,
  "packageManager": "npm@11.16.0"
}
{
  "name": "command-install-npm11",
  "version": "1.0.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "command-install-npm11",
      "version": "1.0.0"
    }
  }
}
```
