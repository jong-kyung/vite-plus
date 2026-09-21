# unsupported_pack_options

## `vpt json-edit package.json scripts.prepack 'node -e "require('\''node:fs'\'').writeFileSync('\''prepack-ran'\'', '\''unexpected'\'')"'`


## `vp pm pack --out package.tgz --pack-destination output`

Do not prepare an output directory or run prepack when an option is unsupported.

**Exit code:** 1

```
npm does not support --out.
```

## `vpt stat-file output --assert missing`

```
output: missing
```

## `vpt stat-file prepack-ran --assert missing`

```
prepack-ran: missing
```
