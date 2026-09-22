# classic_update_interactive

## `vp install --ignore-scripts`


## `NODE_OPTIONS=--require="${workspace}/interactive-milestone.cjs" vp update --interactive --latest is-number -- --ignore-scripts`

select the offered updates in Yarn's native prompt and leave the unrelated dev dependency unchanged

**→ expect-milestone:** `checkbox:yarn-upgrade:ready`

```
yarn upgrade-interactive <version>
info Color legend :
 "<red>"    : Major Update backward-incompatible updates
 "<yellow>" : Minor Update backward-compatible features
 "<green>"  : Patch Update backward-compatible bug fixes
? Choose which packages to update. (Press <space> to select, <a> to toggle all, <i> to invert selection)
 dependencies
   name       range   from      to     workspace                            url
❯◯ is-number  latest  6.0.0  ❯  7.0.0  command-update-yarn1-with-workspace  https://github.com/jonschlinkert/is-number
 ◯ is-number  latest  6.0.0  ❯  7.0.0  app                                  https://github.com/jonschlinkert/is-number
 ◯ is-number  latest  6.0.0  ❯  7.0.0  web                                  https://github.com/jonschlinkert/is-number
```

**← write:** `a`

**← write-key:** `enter`

```
yarn upgrade-interactive <version>
info Color legend :
 "<red>"    : Major Update backward-incompatible updates
 "<yellow>" : Minor Update backward-compatible features
 "<green>"  : Patch Update backward-compatible bug fixes
? Choose which packages to update.
? Choose which packages to update. is-number@7.0.0, is-number@7.0.0, is-number@7.0.0
info Installing "dependencies"...
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
warning Ignored scripts due to flag.
success Saved lockfile.
success Saved 1 new dependency.
info Direct dependencies
└─ is-number@7.0.0
info All dependencies
└─ is-number@7.0.0
info Installing "dependencies"...
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
warning Ignored scripts due to flag.
success Saved lockfile.
success Saved 0 new dependencies.
info Installing "dependencies"...
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
warning Ignored scripts due to flag.
success Saved lockfile.
success Saved 0 new dependencies.

Done in <duration>.
```

## `vpt print-file package.json packages/app/package.json packages/web/package.json`

```
{
  "name": "command-update-yarn1-with-workspace",
  "version": "1.0.0",
  "private": true,
  "packageManager": "yarn@1.22.22",
  "workspaces": [
    "packages/*"
  ],
  "dependencies": {
    "is-number": "7.0.0"
  },
  "devDependencies": {
    "yocto-queue": "0.1.0"
  }
}
{
  "name": "app",
  "version": "1.0.0",
  "dependencies": {
    "is-number": "7.0.0"
  }
}
{
  "name": "web",
  "version": "1.0.0",
  "dependencies": {
    "is-number": "7.0.0"
  }
}
```

## `node assert-installed.cjs 7.0.0 7.0.0 7.0.0`

```
```
