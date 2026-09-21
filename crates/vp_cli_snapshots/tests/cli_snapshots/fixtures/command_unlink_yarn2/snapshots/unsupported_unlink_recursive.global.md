# unsupported_unlink_recursive

## `vpt touch-file yarn.lock`


## `vp install`


## `vp unlink --recursive linked-lib`

Yarn 2 has no native unlink and must not fall through to the project script.

**Exit code:** 1

```
yarn < 3 does not support --recursive.
```

## `vpt stat-file unlink-script-ran --assert missing`

```
unlink-script-ran: missing
```

## `vp unlink -r`

**Exit code:** 1

```
yarn < 3 does not support --recursive.
```

## `vpt stat-file unlink-script-ran --assert missing`

```
unlink-script-ran: missing
```
