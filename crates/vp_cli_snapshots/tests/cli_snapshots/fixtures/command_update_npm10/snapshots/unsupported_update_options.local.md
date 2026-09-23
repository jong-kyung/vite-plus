# unsupported_update_options

## `vp update --latest --interactive --dev --prod --no-optional --workspace`

Report every unsupported option without changing dependencies.

**Exit code:** 1

```
npm does not support --latest.
npm does not support --interactive.
npm does not support --workspace.
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
