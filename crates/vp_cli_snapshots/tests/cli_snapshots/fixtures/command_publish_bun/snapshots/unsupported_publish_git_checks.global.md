# unsupported_publish_git_checks

## `vpt json-edit package.json scripts.prepack 'node -e "require('\''node:fs'\'').writeFileSync('\''prepack-ran'\'', '\''unexpected'\'')"'`


## `vpt json-edit package.json packageManager bun@1.4.2`


## `vp pm publish --dry-run --no-git-checks`

Reject before invoking Bun, without requiring authentication.

**Exit code:** 1

```
bun does not support --no-git-checks.
```

## `vpt stat-file prepack-ran --assert missing`

```
prepack-ran: missing
```
