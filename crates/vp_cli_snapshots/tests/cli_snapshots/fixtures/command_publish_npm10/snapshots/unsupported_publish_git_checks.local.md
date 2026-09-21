# unsupported_publish_git_checks

## `vpt json-edit package.json scripts.prepack 'node -e "require('\''node:fs'\'').writeFileSync('\''prepack-ran'\'', '\''unexpected'\'')"'`


## `vp pm publish --dry-run --no-git-checks`

Reject the pnpm-only option before running prepack.

**Exit code:** 1

```
npm does not support --no-git-checks.
```

## `vpt stat-file prepack-ran --assert missing`

```
prepack-ran: missing
```
