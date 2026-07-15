# check_backpressure_nonblocking_stdout

vp check exposes the stdout EAGAIN failure when a large diagnostic replay meets a non-blocking, backpressured pipe (#2165).

## `vpt backpressure-run --digest 6,8 -- vp check`

**Exit code:** 1

```
backpressure-run detected truncated child output under stdio backpressure
```
