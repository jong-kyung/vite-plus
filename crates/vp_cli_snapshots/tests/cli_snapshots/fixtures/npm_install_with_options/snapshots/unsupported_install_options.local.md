# unsupported_install_options

## `vp install --fix-lockfile --silent`

Silent mode must not hide an unsupported-option error.

**Exit code:** 1

```
npm does not support --fix-lockfile.
```

## `vpt stat-file package-lock.json --assert missing`

```
package-lock.json: missing
```

## `vpt stat-file node_modules --assert missing`

```
node_modules: missing
```
