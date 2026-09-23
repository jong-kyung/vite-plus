# yarn_berry_install_rejects_unsupported_options

## `vp install --prefer-offline`

**Exit code:** 1

```
yarn >= 2 does not support --prefer-offline.
```

## `vp install --offline`

**Exit code:** 1

```
yarn >= 2 does not support --offline.
```

## `vp install --no-lockfile`

**Exit code:** 1

```
yarn >= 2 does not support --no-lockfile.
```

## `vp install --force`

**Exit code:** 1

```
yarn >= 2 does not support --force.
```

## `vp install --no-optional`

**Exit code:** 1

```
yarn >= 2 does not support --no-optional.
```

## `vp install --prefer-offline --offline --no-lockfile --force --no-optional --silent`

report all unsupported options, including Berry's existing silent restriction

**Exit code:** 1

```
yarn >= 2 does not support --no-optional.
yarn >= 2 does not support --prefer-offline.
yarn >= 2 does not support --offline.
yarn >= 2 does not support --force.
yarn >= 2 does not support --no-lockfile.
yarn >= 2 does not support --silent.
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt print-file package.json`

```
{
  "name": "command-install-yarn4",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "yarn@4.16.0"
}
```

## `vp install --ignore-scripts`

ordinary installation remains supported

```
➤ YN0000: · Yarn <version>
➤ YN0000: ┌ Resolution step
➤ YN0085: │ + prod-only-fixture@file:prod-dep#prod-dep::hash=<hash>&locator=command-install-yarn4%40workspace%3A.
➤ YN0000: └ Completed
➤ YN0000: ┌ Fetch step
➤ YN0013: │ A package was added to the project (+ <size> KiB).
➤ YN0000: └ Completed
➤ YN0000: ┌ Link step
➤ YN0000: └ Completed
➤ YN0000: · Done in <duration>
```

## `vpt stat-file node_modules/prod-only-fixture/package.json --assert file`

```
node_modules/prod-only-fixture/package.json: file
```
