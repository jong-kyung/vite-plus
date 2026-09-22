# command_update_npm10

## `vp update testnpm2 -- --no-audit`

should update package within semver range

```

added 3 packages in <duration>
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*"
  },
  "packageManager": "npm@10.9.2"
}
```

## `vp up testnpm2 --latest -- --no-audit`

reject unsupported --latest without updating dependencies

**Exit code:** 1

```
npm does not support --latest.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*"
  },
  "packageManager": "npm@10.9.2"
}
```

## `vp update -D -- --no-audit`

reject unsupported dev-only updates

**Exit code:** 1

```
npm does not support --dev.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*"
  },
  "packageManager": "npm@10.9.2"
}
```

## `vp update -P --no-save -- --no-audit`

reject unsupported production-only updates even with no-save

**Exit code:** 1

```
npm does not support --prod.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "dependencies": {
    "testnpm2": "*"
  },
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*"
  },
  "packageManager": "npm@10.9.2"
}
```

## `vp rm testnpm2`

should skip optional dependencies

```

removed 1 package, and audited 3 packages in <duration>

found 0 vulnerabilities
```

## `vp add testnpm2@1.0.0 -O -- --no-audit`

```

added 1 package in <duration>
```

## `vp update --no-optional -- --no-audit`

reject unsupported optional dependency exclusion

**Exit code:** 1

```
npm does not support --no-optional.
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*",
    "testnpm2": "^1.0.0"
  },
  "packageManager": "npm@10.9.2"
}
```

## `vp update -- --no-audit`

should update all packages but won't change the package.json

```

changed 1 package in <duration>
```

## `vpt print-file package.json`

```
{
  "name": "command-update-npm10",
  "version": "1.0.0",
  "devDependencies": {
    "test-vite-plus-package": "*"
  },
  "optionalDependencies": {
    "test-vite-plus-package-optional": "*",
    "testnpm2": "^1.0.0"
  },
  "packageManager": "npm@10.9.2"
}
```
