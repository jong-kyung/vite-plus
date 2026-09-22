# install_without_packages_rejects_add_only_options

## `vpt cp package.json before.json`


## `vp install --save-exact --save-peer --save-optional --save-catalog --lockfile-only`

reject add-only options without package names instead of reporting manager support

**Exit code:** 1

```
install without package names does not support --save-exact.
install without package names does not support --save-peer.
install without package names does not support --save-optional.
install without package names does not support --save-catalog.
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `node -e 'const fs = require('\''node:fs'\''); require('\''node:assert/strict'\'').equal(fs.readFileSync('\''package.json'\'', '\''utf8'\''), fs.readFileSync('\''before.json'\'', '\''utf8'\''))'`

```
```

## `vpt print-file package.json`

```
{
  "name": "install-package-options",
  "version": "1.0.0",
  "private": true,
  "packageManager": "npm@11.13.0"
}
```
