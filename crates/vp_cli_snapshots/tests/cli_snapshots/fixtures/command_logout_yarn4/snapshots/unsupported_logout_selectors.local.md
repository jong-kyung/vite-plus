# unsupported_logout_selectors

## `vp pm logout --registry http://127.0.0.1:9 --scope company -- --help`

Berry supports scope but not an explicit registry URL flag.

**Exit code:** 1

```
yarn does not support --registry.
```

## `vp pm logout --scope @company`

Berry rejects an @-prefixed scope, so the prefix is stripped before forwarding.

```
➤ YN0000: Successfully logged out from company
➤ YN0000: Done in <duration>
```
