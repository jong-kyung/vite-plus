# stage_native_rejects_unsupported_options

## `vp pm stage publish --recursive --filter vp-stage-yarn-a --registry http://127.0.0.1:9 -- --help`

Reject workspace selection that cannot be translated to native staged publishing, even with raw help.

**Exit code:** 1

```
yarn does not support --recursive or --filter for native staged publishing.
```

## `vp pm stage list --registry http://127.0.0.1:9 -- --help`

**Exit code:** 1

```
yarn >= 4.16.0 does not support --registry.
```

## `vp pm stage approve abc123 --registry http://127.0.0.1:9 -- --help`

**Exit code:** 1

```
yarn >= 4.16.0 does not support --registry.
```

## `vp pm stage reject abc123 --registry http://127.0.0.1:9 -- --help`

**Exit code:** 1

```
yarn >= 4.16.0 does not support --registry.
```

## `vpt stat-file .stage-prepack-ran --assert missing`

```
.stage-prepack-ran: missing
```
