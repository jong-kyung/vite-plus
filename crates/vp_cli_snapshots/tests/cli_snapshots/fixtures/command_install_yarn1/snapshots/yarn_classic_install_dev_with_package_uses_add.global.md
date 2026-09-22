# yarn_classic_install_dev_with_package_uses_add

## `vp install -D ./dev-dep --ignore-scripts`

with a package argument, -D still saves to devDependencies

```
VITE+ - The Unified Toolchain for the Web

yarn add <version>
info No lockfile found.
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
warning Ignored scripts due to flag.
success Saved lockfile.
success Saved 2 new dependencies.
info Direct dependencies
├─ dev-only-fixture@1.0.0
└─ prod-only-fixture@1.0.0
info All dependencies
├─ dev-only-fixture@1.0.0
└─ prod-only-fixture@1.0.0

Done in <duration>.
```

## `vpt print-file package.json`

```
{
  "name": "command-install-yarn1",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "prod-only-fixture": "file:prod-dep"
  },
  "packageManager": "yarn@1.22.22",
  "devDependencies": {
    "dev-only-fixture": "./dev-dep"
  }
}
```

## `vpt stat-file node_modules/dev-only-fixture/package.json --assert file`

```
node_modules/dev-only-fixture/package.json: file
```
