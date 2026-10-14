# command_publish_yarn1

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

## `vp pm publish --dry-run`

should reject dry-run before Yarn Classic can publish

**Exit code:** 1

```
yarn < 4.9.3 does not support --dry-run.
```

## `vp pm publish --provenance`

should reject provenance before Yarn Classic publishes without an attestation

**Exit code:** 1

```
yarn < 4.9.0 does not support --provenance.
```

## `vp pm publish --recursive`

should reject workspace publishing rather than use npm

**Exit code:** 1

```
yarn does not support --recursive.
```

## `vp pm publish --filter app`

should reject workspace selection rather than publish the current package

**Exit code:** 1

```
yarn does not support --filter.
```

## `vp pm publish -- --help`

should route to native Yarn Classic publish without publishing

```
warning package.json: No license field

  Usage: yarn publish [<tarball>|<folder>] [--tag <tag>] [--access <public|restricted>]

  Publishes a package to the npm registry.

  Options:

    -v, --version                       output the version number
    --no-default-rc                     prevent Yarn from automatically detecting yarnrc and npmrc files
    --use-yarnrc <path>                 specifies a yarnrc file that Yarn should use (.yarnrc only, not .npmrc) (default: )
    --verbose                           output verbose messages on internal operations
    --offline                           trigger an error if any required dependencies are not available in local cache
    --prefer-offline                    use network only if dependencies are not available in local cache
    --enable-pnp, --pnp                 enable the Plug'n'Play installation
    --disable-pnp                       disable the Plug'n'Play installation
    --strict-semver
    --json                              format Yarn log messages as lines of JSON (see jsonlines.org)
    --ignore-scripts                    don't run lifecycle scripts
    --har                               save HAR output of network traffic
    --ignore-platform                   ignore platform checks
    --ignore-engines                    ignore engines check
    --ignore-optional                   ignore optional dependencies
    --force                             install and build packages even if they were built before, overwrite lockfile
    --skip-integrity-check              run install without checking if node_modules is installed
    --check-files                       install will verify file tree of packages for consistency
    --no-bin-links                      don't generate bin links when setting up packages
    --flat                              only allow one version of a package
    --prod, --production [prod]
    --no-lockfile                       don't read or generate a lockfile
    --pure-lockfile                     don't generate a lockfile
    --frozen-lockfile                   don't generate a lockfile and fail if an update is needed
    --update-checksums                  update package checksums from current repository
    --link-duplicates                   create hardlinks to the repeated modules in node_modules
    --link-folder <path>                specify a custom folder to store global links
    --global-folder <path>              specify a custom folder to store global packages
    --modules-folder <path>             rather than installing modules into the node_modules folder relative to the cwd, output them here
    --preferred-cache-folder <path>     specify a custom folder to store the yarn cache if possible
    --cache-folder <path>               specify a custom folder that must be used to store the yarn cache
    --mutex <type>[:specifier]          use a mutex to ensure only one yarn instance is executing
    --emoji [bool]                      enable emoji in output (default: true)
    -s, --silent                        skip Yarn console logs, other types of logs (script output) will be printed
    --cwd <cwd>                         working directory to use (default: <workspace>)
    --proxy <host>
    --https-proxy <host>
    --registry <url>                    override configuration registry
    --no-progress                       disable progress bar
    --network-concurrency <number>      maximum number of concurrent network requests
    --network-timeout <milliseconds>    TCP timeout for network requests
    --non-interactive                   do not show interactive prompts
    --scripts-prepend-node-path [bool]  prepend the node executable dir to the PATH in scripts
    --no-node-version-check             do not warn when using a potentially unsupported Node version
    --focus                             Focus on a single workspace by installing remote copies of its sibling workspaces.
    --otp <otpcode>                     one-time password for two factor authentication
    --new-version [version]             new version
    --major                             auto-increment major version number
    --minor                             auto-increment minor version number
    --patch                             auto-increment patch version number
    --premajor                          auto-increment premajor version number
    --preminor                          auto-increment preminor version number
    --prepatch                          auto-increment prepatch version number
    --prerelease                        auto-increment prerelease version number
    --preid [preid]                     add a custom identifier to the prerelease
    --message [message]                 message
    --no-git-tag-version                no git tag version
    --no-commit-hooks                   bypass git hooks when committing new version
    --access [access]                   access
    --tag [tag]                         tag
    -h, --help                          output usage information
  Visit https://yarnpkg.com/en/docs/cli/publish for documentation about this command.
```
