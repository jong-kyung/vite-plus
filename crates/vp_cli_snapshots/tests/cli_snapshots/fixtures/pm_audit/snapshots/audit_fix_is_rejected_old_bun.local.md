# audit_fix_is_rejected_old_bun

## `vpt json-edit package.json packageManager bun@1.3.11`


## `vp pm audit --fix -- --help`

Bun before 1.4 rejects the named option instead of reporting a successful Noop.

**Exit code:** 1

```
bun < 1.4 does not support --fix.
```

## `vp pm audit --fix --production -- --help`

Collect fix and production errors together.

**Exit code:** 1

```
bun < 1.4 does not support --fix.
bun does not support --production.
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```
