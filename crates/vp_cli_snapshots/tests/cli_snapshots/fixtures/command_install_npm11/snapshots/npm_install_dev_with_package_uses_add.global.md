# npm_install_dev_with_package_uses_add

## `vp install -D ./dev-dep --ignore-scripts`

with a package argument, -D still saves to devDependencies

```
VITE+ - The Unified Toolchain for the Web

added 1 package in <duration>
```

## `vpt print-file package.json`

```
{
  "name": "command-install-npm11",
  "version": "1.0.0",
  "private": true,
  "packageManager": "npm@11.16.0",
  "devDependencies": {
    "dev-only-fixture": "file:dev-dep"
  }
}
```

## `vpt stat-file node_modules/dev-only-fixture/package.json --assert file`

```
node_modules/dev-only-fixture/package.json: file
```
