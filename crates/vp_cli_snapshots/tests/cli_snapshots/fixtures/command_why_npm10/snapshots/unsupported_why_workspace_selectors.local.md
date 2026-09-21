# unsupported_why_workspace_selectors

## `vp why react --recursive --workspace-root --filter app --json -- --help`

Reject selectors instead of silently narrowing the request to the filter.

**Exit code:** 1

```
npm does not support --recursive.
npm does not support --workspace-root.
```

## `vp why react -r`

**Exit code:** 1

```
npm does not support --recursive.
```

## `vp why react -w`

**Exit code:** 1

```
npm does not support --workspace-root.
```

## `vpt json-edit package.json packageManager npm@12.0.2`


## `vp why react -r -w -- --help`

Current npm must not silently ignore the same named selectors.

**Exit code:** 1

```
npm does not support --recursive.
npm does not support --workspace-root.
```
