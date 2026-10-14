# command_registry_pnpm11

## `vp pm dist-tag list`

should preserve the native missing-package error

**Exit code:** 1

```
[ERR_PNPM_DIST_TAG_LS_PACKAGE_REQUIRED] Package name is required
```

## `vp pm deprecate command-registry-pnpm11@1.0.0 `

should preserve the native empty-message error without undeprecating

**Exit code:** 1

```
[ERR_PNPM_DEPRECATE_MESSAGE_REQUIRED] Deprecation message is required. To un-deprecate, use the undeprecate command.
```

## `vp pm search react --long`

should let pnpm reject unsupported options

**Exit code:** 1

```
[ERROR] Unknown option: 'long'
For help, run: pnpm help search
```
