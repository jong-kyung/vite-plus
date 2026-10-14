# vp pm patch-commit

Save edits prepared with [patch](./patch) as a dependency patch.

```bash
vp pm patch-commit /path/to/edited-package
```

## Options

Supply the edit directory from the manager's patch workflow. There are no additional Vite+ options.

There are no command-specific options.

## Compatibility

- **npm:** 12 or later: `npm patch commit`. Earlier versions warn and skip.
- **pnpm:** `pnpm patch-commit`.
- **Yarn Classic:** No command. Warns and exits successfully without performing the operation.
- **Yarn Modern:** `yarn patch-commit --save`.
- **Bun:** `bun patch --commit`.

Review and commit the patch and any manifest or lockfile changes produced by the manager.

## Native Options

See [Native Arguments](./#native-arguments). References: [npm commands](https://docs.npmjs.com/cli/v12/commands), [pnpm patch-commit](https://pnpm.io/cli/patch-commit), [Yarn patch-commit](https://yarnpkg.com/cli/patch-commit), and [Bun patch](https://bun.com/docs/pm/cli/patch).
