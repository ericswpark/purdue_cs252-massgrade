# massgrade

Grade lots of directories at once

```
$ massgrade ./dirA ./dirB
Checking ./dirA...
Initial commit was made at 2025-02-24 15:44:50 (4 minutes ago)
=============
| Eric Park |
=============
Commits: 3 commits
Time spent: 16 minutes
LOC: +0/-0 (NaN%)
Checking ./dirB...
(... snip ...)
```

## Building

- Install `rustup`
- Install the Rust compiler and toolchain with `rustup`
- Build a release executable using the following command:

```
cargo build --release
```

Or, you can use the `Makefile`.


### Troubleshooting

This project depends on the [cs252chkr](https://github.com/ericswpark/purdue_cs252chkr) project. Make sure Git can
access this project over HTTPS or SSH. Otherwise, `cargo` won't be able to clone the subproject as a dependency.
