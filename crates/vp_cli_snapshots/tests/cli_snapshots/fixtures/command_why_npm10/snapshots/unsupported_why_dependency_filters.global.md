# unsupported_why_dependency_filters

## `vp why react --no-optional --exclude-peers --json -- --help`

Reject both filters instead of discarding them before npm explain.

**Exit code:** 1

```
npm does not support --no-optional.
npm does not support --exclude-peers.
```

## `vp why react --no-optional`

**Exit code:** 1

```
npm does not support --no-optional.
```

## `vp why react --exclude-peers`

**Exit code:** 1

```
npm does not support --exclude-peers.
```

## `vpt json-edit package.json packageManager npm@12.0.2`


## `vp why react --no-optional --exclude-peers --json -- --help`

Current npm still does not filter these dependency types in explain.

**Exit code:** 1

```
npm does not support --no-optional.
npm does not support --exclude-peers.
```
