# Check

`vp check` runs format, lint, and type checks together.

## Overview

`vp check` is the default command for fast static checks in Vite+. It brings together formatting through [Oxfmt](https://oxc.rs/docs/guide/usage/formatter.html), linting through [Oxlint](https://oxc.rs/docs/guide/usage/linter.html), and TypeScript type checks through [tsgolint](https://github.com/oxc-project/tsgolint). By merging all of these tasks into a single command, `vp check` is faster than running formatting, linting, and type checking as separate tools in separate commands.

When `typeCheck` is enabled in the `lint.options` block in `vite.config.ts`, `vp check` also runs TypeScript type checks through the Oxlint type-aware path powered by the TypeScript Go toolchain and [tsgolint](https://github.com/oxc-project/tsgolint). `vp create` and `vp migrate` enable both `typeAware` and `typeCheck` by default.

We recommend turning `typeCheck` on so `vp check` becomes the single command for static checks during development.

## Usage

```bash
vp check
vp check --fix # Format and run autofixers.
```

### Phase toggles

`vp check` exposes its three core phases — format, lint, and type check — as
independent `--no-*` flags. All three run by default. Disable any subset by
combining the flags:

| Command | fmt | lint | type check |
|---|:-:|:-:|:-:|
| `vp check` | ✓ | ✓ | ✓ |
| `vp check --no-fmt` | ✗ | ✓ | ✓ |
| `vp check --no-lint` | ✓ | ✗ | ✓ |
| `vp check --no-type-check` | ✓ | ✓ | ✗ |
| `vp check --no-lint --no-type-check` | ✓ | ✗ | ✗ |

Notes:

- `--no-lint` skips lint rules only. Type checking still runs when
  `lint.options.typeCheck` is enabled in `vite.config.ts` — useful when you want
  to focus on type errors without lint-rule noise.
- `--no-type-check` overrides `lint.options.typeCheck` at runtime. Lint rules —
  including type-aware rules governed by `typeAware` — continue to run.
- Disabling every phase (`--no-fmt --no-lint --no-type-check`) exits with an
  error.
- `--fix` cannot be combined with `--no-lint` while type checking is enabled,
  because lint rule fixes would be skipped. Add `--no-type-check` to request a
  fmt-only fix.

> **Breaking change:** Prior releases treated `--no-lint` as "skip the entire
> lint phase, including type checks". `--no-lint` now skips lint rules only. To
> restore the old behavior (skip both), use `--no-lint --no-type-check`.

## Configuration

`vp check` uses the same configuration you already define for linting and formatting:

- [`lint`](/guide/lint#configuration) block in `vite.config.ts`
- [`fmt`](/guide/fmt#configuration) block in `vite.config.ts`
- TypeScript project structure and tsconfig files for type-aware linting

Recommended base `lint` config:

```ts
import { defineConfig } from 'vite-plus';

export default defineConfig({
  lint: {
    options: {
      typeAware: true,
      typeCheck: true,
    },
  },
});
```
