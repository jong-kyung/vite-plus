# pm_team_yarn_modern

## `vpt json-edit package.json packageManager yarn@4.18.0`


## `vp pm team ls @org -- --version`


## `node registry.mjs`

Yarn Modern warns and executes npm team with registry, authentication, and OTP intact

```
vp pm team create @org:developers --json --registry <registry> --otp <otp>
{
  "created": true,
  "team": "org:developers"
}
warn: yarn@2+ does not support team, falling back to npm team.
Verified PUT /-/org/org/team with authentication and OTP.
vp pm team add @org:developers alice --json --registry <registry> --otp <otp>
{
  "added": true,
  "team": "org:developers",
  "user": "alice"
}
warn: yarn@2+ does not support team, falling back to npm team.
Verified PUT /-/team/org/developers/user with authentication and OTP.
vp pm team ls @org --json --registry <registry> --otp <otp>
[
  "developers"
]
warn: yarn@2+ does not support team, falling back to npm team.
Verified GET /-/org/org/team with authentication.
vp pm team list @org:developers --json --registry <registry> --otp <otp>
[
  "alice"
]
warn: yarn@2+ does not support team, falling back to npm team.
Verified GET /-/team/org/developers/user with authentication.
vp pm team remove @org:developers alice --json --registry <registry> --otp <otp>
{
  "removed": true,
  "team": "org:developers",
  "user": "alice"
}
warn: yarn@2+ does not support team, falling back to npm team.
Verified DELETE /-/team/org/developers/user with authentication and OTP.
vp pm team destroy @org:developers --json --registry <registry> --otp <otp>
{
  "deleted": true,
  "team": "org:developers"
}
warn: yarn@2+ does not support team, falling back to npm team.
Verified DELETE /-/team/org/developers with authentication and OTP.
vp pm team create @org:developers --json --registry <registry> --otp <otp>
Registry denial returned a non-zero exit code.
Verified PUT /-/org/org/team with authentication and OTP.
```
