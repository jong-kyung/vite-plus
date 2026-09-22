# global_remove_bypasses_local_scope_validation

## `vp remove --global --dry-run --recursive --filter app typescript`

global commands take the managed-global path before local package-manager validation

**Exit code:** 1

```
Failed to uninstall typescript: Package typescript is not installed
```

## `vpt print-file package.json`

```
{
  "name": "command-remove-yarn1",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "testnpm2": "1.0.0"
  },
  "packageManager": "yarn@1.22.22"
}
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
