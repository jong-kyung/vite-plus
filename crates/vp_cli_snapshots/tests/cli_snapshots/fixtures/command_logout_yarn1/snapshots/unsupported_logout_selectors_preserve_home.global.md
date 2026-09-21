# unsupported_logout_selectors_preserve_home

## `vp pm config set username local-test-user --location user`


## `vp pm config set email local-test@example.invalid --location user`


## `vp pm logout --registry http://127.0.0.1:9 --scope company`

Rejected selectors must not clear the home login information.

**Exit code:** 1

```
yarn does not support --registry.
yarn < 2 does not support --scope.
```

## `vp pm config get username --location user`

```
warning package.json: No license field
local-test-user
```

## `vp pm config get email --location user`

```
warning package.json: No license field
local-test@example.invalid
```

## `vp pm logout`

Optionless logout still clears the credentials.

```
yarn logout <version>
warning package.json: No license field
success Cleared login credentials.

Done in <duration>.
```

## `vp pm config get username --location user`

```
warning package.json: No license field
undefined
```
