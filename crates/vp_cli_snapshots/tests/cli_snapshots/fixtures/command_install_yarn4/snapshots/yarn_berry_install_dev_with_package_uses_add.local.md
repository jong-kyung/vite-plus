# yarn_berry_install_dev_with_package_uses_add

## `vp install -D ./dev-dep --ignore-scripts`

with a package argument, -D still saves to devDependencies

```
➤ YN0000: · Yarn <version>
➤ YN0000: ┌ Resolution step
➤ YN0085: │ + dev-only-fixture@file:./dev-dep#./dev-dep::hash=6425ea&locator=command-install-yarn4%40workspace%3A., prod-only-fixture@file:prod-dep#prod-dep::hash=d14722&locator=command-install-yarn4%40workspace%3A.
➤ YN0000: └ Completed
➤ YN0000: ┌ Fetch step
➤ YN0013: │ 2 packages were added to the project (+ <size> KiB).
➤ YN0000: └ Completed
➤ YN0000: ┌ Link step
➤ YN0000: └ Completed
➤ YN0000: · Done in <duration>
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
  "packageManager": "yarn@4.16.0",
  "devDependencies": {
    "dev-only-fixture": "./dev-dep"
  }
}
```

## `vpt stat-file node_modules/dev-only-fixture/package.json --assert file`

```
node_modules/dev-only-fixture/package.json: file
```
