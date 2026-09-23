# audit_fix_is_rejected_old_bun

## `vpt json-edit package.json packageManager bun@1.3.11`


## `vp pm audit --fix --production -- --help`

Bun before 1.4 rejects fix even when combined with supported production filtering.

**Exit code:** 1

```
bun < 1.4 does not support --fix.
```

## `vpt json-edit package.json packageManager bun@1.2.20`


## `vp pm audit --production -- --help`

Bun before 1.2.21 silently ignores --production, so reject it instead of auditing everything.

**Exit code:** 1

```
bun < 1.2.21 does not support --production.
```

## `vpt stat-file bun.lock --assert missing`

```
bun.lock: missing
```
