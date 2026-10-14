# command_publish_yarn4

## `vp pm publish --help`

should show help

```
VITE+ - The Unified Toolchain for the Web

Usage: vp pm publish [OPTIONS] [TARBALL|FOLDER] [-- <PASS_THROUGH_ARGS>...]

Publish package to registry

Arguments:
  [TARBALL|FOLDER]        Tarball or folder to publish
  [PASS_THROUGH_ARGS]...  Additional arguments

Options:
  --dry-run                  Preview without publishing
  --tag <TAG>                Publish tag
  --access <ACCESS>          Access level (public/restricted)
  --otp <OTP>                One-time password for authentication
  --no-git-checks            Skip git checks
  --publish-branch <BRANCH>  Set the branch name to publish from
  --report-summary           Save publish summary
  --provenance               Publish with provenance
  --force                    Force publish
  --json                     Output in JSON format
  -r, --recursive            Publish all workspace packages
  --filter <PATTERN>         Filter packages in monorepo
  -h, --help                 Print help

Documentation: https://viteplus.dev/guide/package-management
```

## `vp install`


## `vp pm publish --dry-run`

should preview using native yarn npm publish without publishing

```
➤ YN0000: package.json
➤ YN0000: [DRY RUN] Package would be published to https://registry.yarnpkg.com with tag latest
➤ YN0000: Done in <duration>
```

## `vp pm publish -- --dry-run`

should preserve supported raw dry-run

```
➤ YN0000: package.json
➤ YN0000: [DRY RUN] Package would be published to https://registry.yarnpkg.com with tag latest
➤ YN0000: Done in <duration>
```

## `vp pm publish pkg.tgz`

should reject an explicit target rather than use npm

**Exit code:** 1

```
yarn >= 2 does not support TARBALL|FOLDER.
```

## `vp pm publish --recursive`

should reject workspace publishing rather than use npm

**Exit code:** 1

```
yarn does not support --recursive.
```

## `vp pm publish --filter app`

should reject workspace selection rather than use npm

**Exit code:** 1

```
yarn does not support --filter.
```
