# audit_fix_is_rejected_old_bun

## `vpt json-edit package.json packageManager bun@1.3.11`


## `vp pm audit --fix --production -- --help`

Bun before 1.4 rejects fix even when combined with supported production filtering.

**Exit code:** 1

```
bun < 1.4 does not support --fix.
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```
