# unsupported_approval_all

## `vpt json-edit package.json packageManager yarn@4.18.0`


## `vp pm approve-builds --all -- --help`

Yarn Berry rejects all before invoking any native command.

**Exit code:** 1

```
yarn does not support --all.
```

## `vpt json-edit package.json packageManager yarn@1.22.22`


## `vp pm approve-builds --all -- --help`

**Exit code:** 1

```
yarn does not support --all.
```

## `vpt json-edit package.json packageManager npm@11.15.0`


## `vp pm approve-builds --all -- --help`

Old npm rejects the option instead of reporting a successful Noop.

**Exit code:** 1

```
npm < 11.16.0 does not support --all.
```

## `vpt json-edit package.json packageManager pnpm@10.31.0`


## `vp pm approve-builds --all -- --help`

Old pnpm uses the same pre-execution support diagnostic.

**Exit code:** 1

```
pnpm < 10.32.0 does not support --all.
```

## `vpt print-file package.json`

```
{
  "name": "command-pm-approve-builds-yarn",
  "packageManager": "pnpm@10.31.0",
  "private": true,
  "version": "1.0.0"
}
```
