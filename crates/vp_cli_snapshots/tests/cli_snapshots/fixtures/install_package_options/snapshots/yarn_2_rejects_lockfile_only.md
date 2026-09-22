# yarn_2_rejects_lockfile_only

## `vpt write-file .node-version '22.18.0
'`


## `vpt json-edit package.json packageManager yarn@2.4.2`


## `vp add ./dep --lockfile-only`

Yarn 2 rejects lockfile-only because add does not support --mode

**Exit code:** 1

```
yarn < 3 does not support --lockfile-only.
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vp install ./dep-v2 --lockfile-only`

positional install rejects the same unsupported option

**Exit code:** 1

```
VITE+ - The Unified Toolchain for the Web

yarn < 3 does not support --lockfile-only.
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt print-file package.json`

```
{
  "name": "install-package-options",
  "packageManager": "yarn@2.4.2",
  "private": true,
  "version": "1.0.0"
}
```
