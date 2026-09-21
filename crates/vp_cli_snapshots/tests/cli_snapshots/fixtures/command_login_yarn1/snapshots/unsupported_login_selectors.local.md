# unsupported_login_selectors

## `vp pm config set username local-test-user --location user`


## `vp pm config set email local-test@example.invalid --location user`


## `vp pm login --registry http://127.0.0.1:9 --scope company`

Classic must reject both selectors rather than ignore them.

**Exit code:** 1

```
yarn does not support --registry.
yarn < 2 does not support --scope.
```

## `vp pm login`

Without selectors, retain Classic's existing login behavior.

```
yarn login <version>
warning package.json: No license field
info npm username: local-test-user
info npm email: local-test@example.invalid

Done in <duration>.
```
