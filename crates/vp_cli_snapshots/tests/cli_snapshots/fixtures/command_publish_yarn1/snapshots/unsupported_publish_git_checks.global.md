# unsupported_publish_git_checks

## `vpt json-edit package.json scripts.prepack 'node -e "require('\''node:fs'\'').writeFileSync('\''prepack-ran'\'', '\''unexpected'\'')"'`


## `vp pm publish --dry-run --no-git-checks --publish-branch main`

Aggregate unsupported options before the npm fallback.

**Exit code:** 1

```
yarn does not support --no-git-checks.
yarn does not support --publish-branch.
```

## `vpt stat-file prepack-ran --assert missing`

```
prepack-ran: missing
```
