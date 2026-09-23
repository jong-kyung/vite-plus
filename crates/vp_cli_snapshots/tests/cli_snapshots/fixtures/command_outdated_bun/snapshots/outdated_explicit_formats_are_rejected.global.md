# outdated_explicit_formats_are_rejected

## `vp outdated --format json -- --help`

Reject named JSON format before running Bun, even when help is passed through.

**Exit code:** 1

```
bun does not support --format.
```

## `vp outdated --format json --long`

Collect declarative support errors together.

**Exit code:** 1

```
bun does not support --long.
bun does not support --format.
```

## `vp outdated --format table`

Reject explicit table format too; omit --format for Bun's default output.

**Exit code:** 1

```
bun does not support --format.
```

## `vp outdated --format list -- --help`

Reject named list format instead of silently using Bun's table output.

**Exit code:** 1

```
bun does not support --format.
```
