# unsupported_why_filters

## `vp why react --depth 0 --no-optional --json -- --help`

Do not silently discard explicit depth zero or optional filtering on Classic.

**Exit code:** 1

```
yarn does not support --depth.
yarn does not support --no-optional.
```

## `vp why react --recursive --exclude-peers --json -- --help`

Reject both unsupported selectors before invoking Classic.

**Exit code:** 1

```
yarn < 2 does not support --recursive.
yarn < 2 does not support --exclude-peers.
```
