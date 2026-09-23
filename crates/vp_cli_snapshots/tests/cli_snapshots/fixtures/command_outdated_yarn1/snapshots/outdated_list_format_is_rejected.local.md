# outdated_list_format_is_rejected

## `vp outdated --format list -- --help`

Reject list before running Yarn, even when help is passed through.

**Exit code:** 1

```
Yarn Classic does not support --format list.
```

## `vp outdated --format table`

An explicit table remains supported.

```
yarn outdated <version>

Done in <duration>.
```

## `vp outdated --format json`

JSON remains supported, with no output for an empty dependency list.

```
```

## `vp outdated`

Omitting the format still runs Yarn outdated.

```
yarn outdated <version>

Done in <duration>.
```
