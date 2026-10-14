# vp pm cache

Inspect or clean the package manager's download cache or store. This is distinct from [`vp cache`](/guide/cache), which manages task results.

```bash
vp pm cache dir
vp pm cache clean
```

## Options

- `dir` and `path` print the cache or store location.
- `clean` requests native cache cleanup.

Other subcommands warn and exit successfully without running a command. There are no additional Vite+ options.

### `dir`

Print the cache or store location. Alias: `path`. There are no command-specific options.

### `clean`

Request native cache cleanup. There are no command-specific options.

## Compatibility

### `dir`

- **npm:** `npm config get cache`.
- **pnpm:** `pnpm store path`.
- **Yarn Classic:** `yarn cache dir`.
- **Yarn Modern:** `yarn config get cacheFolder`.
- **Bun:** `bun pm cache`.

### `clean`

- **npm:** `npm cache clean`.
- **pnpm:** `pnpm store prune`. Prunes the store rather than deleting all content.
- **Yarn Classic:** `yarn cache clean`.
- **Yarn Modern:** `yarn cache clean`.
- **Bun:** `bun pm cache rm`.

npm normally requires an explicit native `--force` for cache deletion:

```bash
vp pm cache clean -- --force
```

That example is npm-specific. Vite+ does not expose a separate `vp pm store` command.

## Native Options

See [Native Arguments](./#native-arguments). References: [npm cache](https://docs.npmjs.com/cli/v11/commands/npm-cache), [pnpm store](https://pnpm.io/cli/store), [Yarn cache clean](https://yarnpkg.com/cli/cache/clean), [Yarn Classic cache](https://classic.yarnpkg.com/en/docs/cli/cache), and [Bun pm](https://bun.com/docs/pm/cli/pm).
