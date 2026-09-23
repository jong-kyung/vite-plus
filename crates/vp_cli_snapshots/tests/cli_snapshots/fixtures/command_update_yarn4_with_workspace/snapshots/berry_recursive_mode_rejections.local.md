# berry_recursive_mode_rejections

## `vpt json-edit package.json packageManager yarn@2.4.2`


## `vp update --recursive testnpm2`

range-preserving recursive updates require Yarn 3

**Exit code:** 1

```
Unknown Syntax Error: Unsupported option name ("--recursive").

$ yarn up [-i,--interactive] [-E,--exact] [-T,--tilde] [-C,--caret] ...
```

## `vpt json-edit package.json packageManager yarn@4.10.3`


## `vp update --recursive --interactive testnpm2`

interactive mode cannot be combined with range-preserving re-resolution

**Exit code:** 1

```
Usage Error: Invalid option schema: property "recursive" forbids using property "interactive"

$ yarn up [-i,--interactive] [-F,--fixed] [-E,--exact] [-T,--tilde] [-C,--caret] [-R,--recursive] [--mode #0] ...
```

## `vp update --recursive --latest --no-save testnpm2`

latest updates cannot promise to preserve manifest ranges

**Exit code:** 1

```
yarn does not support --no-save.
```

## `vpt print-file package.json packages/app/package.json packages/utils/package.json`

```
{
  "dependencies": {
    "testnpm2": "*"
  },
  "name": "command-update-yarn4-with-workspace",
  "packageManager": "yarn@4.10.3",
  "version": "1.0.0",
  "workspaces": [
    "packages/*"
  ]
}
{
  "name": "app",
  "dependencies": {
    "@vite-plus-test/utils": "workspace:*",
    "test-vite-plus-install": "*",
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  }
}
{
  "name": "@vite-plus-test/utils",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  }
}
```

## `vpt stat-file yarn.lock --assert missing`

```
yarn.lock: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
