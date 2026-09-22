# bun_install_dev_with_package_uses_add

## `vp install -D ./dev-dep --ignore-scripts`

with a package argument, -D still saves to devDependencies

```
VITE+ - The Unified Toolchain for the Web

bun add <version> (<hash>)

 prod-only-fixture@prod-dep

installed dev-only-fixture@dev-dep

2 packages installed [<duration>]
```

## `vpt print-file package.json`

```
{
  "name": "command-install-bun",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "bun@1.4.0",
  "devDependencies": {
    "dev-only-fixture": "./dev-dep"
  }
}
```

## `vpt stat-file node_modules/dev-only-fixture/package.json --assert file`

```
node_modules/dev-only-fixture/package.json: file
```
