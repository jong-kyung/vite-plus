# vp pm ping

Check connectivity to a package registry.

```bash
vp pm ping
vp pm ping --registry https://registry.npmjs.org
```

## Options

| Option             | Description          | Notes |
| ------------------ | -------------------- | ----- |
| `--registry <url>` | Select the registry. |       |

## Compatibility

- **npm:** `npm ping`.
- **pnpm:** 11 or later: `pnpm ping`. Earlier versions use `npm ping` as a fallback.
- **Yarn Classic:** Fallback: `npm ping`.
- **Yarn Modern:** Fallback: `npm ping`.
- **Bun:** Fallback: `npm ping`.

| Option             | npm | pnpm | Yarn Classic | Yarn Modern | Bun | Notes |
| ------------------ | --- | ---- | ------------ | ----------- | --- | ----- |
| `--registry <url>` | Yes | Yes  | Yes          | Yes         | Yes |       |

Registry configuration, output, and exit codes follow the command that runs. Use [whoami](./whoami) when you need to check the authenticated account rather than connectivity.

## Native Options

See [Native Arguments](./#native-arguments) and [npm ping](https://docs.npmjs.com/cli/v11/commands/npm-ping).
