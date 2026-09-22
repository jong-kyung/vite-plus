# install_with_packages_rejects_install_only_options

## `vpt cp package.json before.json`


## `vp install ./dep --fix-lockfile --resolution-only --lockfile-only --silent`

reject install-only options before converting to add, even in silent mode

**Exit code:** 1

```
install with package names does not support --fix-lockfile.
install with package names does not support --resolution-only.
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
