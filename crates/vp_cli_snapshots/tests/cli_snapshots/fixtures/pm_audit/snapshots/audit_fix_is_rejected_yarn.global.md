# audit_fix_is_rejected_yarn

## `vpt json-edit package.json packageManager yarn@1.22.22`


## `vp pm audit --fix -- --help`

Yarn Classic rejects the named option instead of reporting a successful Noop.

**Exit code:** 1

```
yarn does not support --fix.
```

## `vpt json-edit package.json packageManager yarn@4.18.0`


## `vp pm audit --fix --json -- --help`

Current Yarn also rejects fix before invoking native help.

**Exit code:** 1

```
yarn does not support --fix.
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```
