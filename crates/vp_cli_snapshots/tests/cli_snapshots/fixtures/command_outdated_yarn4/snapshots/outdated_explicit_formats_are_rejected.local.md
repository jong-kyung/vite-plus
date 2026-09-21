# outdated_explicit_formats_are_rejected

## `vp outdated --format table -- --help`

Explicit table must fail before invoking upgrade-interactive, even when help is passed through.

**Exit code:** 1

```
yarn >= 2 does not support --format.
```

## `vp outdated --format list -- --help`

**Exit code:** 1

```
yarn >= 2 does not support --format.
```

## `vp outdated --format json -- --help`

**Exit code:** 1

```
yarn >= 2 does not support --format.
```

## `vp outdated --format json --long`

Report format alongside other unsupported named options.

**Exit code:** 1

```
yarn does not support --long.
yarn >= 2 does not support --format.
```
