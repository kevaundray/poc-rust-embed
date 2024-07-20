## Proof of concept of Rust Embed docker edge case

### Overview

This repository demonstrates an edge case when using rust-embed with the default features alongside Docker in debug mode.

Debug mode is useful in this context for debugging programs, so this issue would never pop up in software released to the public.

### CI

CI is used to demonstrate the issue. Two jobs should be passing and one job should be failing.

### Issue

When a file is embedded using rust-embed and default features in debug mode, the binary works in Docker, given that you run the binary from the same image that created it. If you copy the binary to a fresh image, then rust-embed says that they cannot find the embedded file.

The first github actions job (Dockerfile.debug.full), shows this with a failing CI.

The second github actions job (Dockerfile.debug.fix) does not copy the binary and shows the CI passing.

The third github actions job (Dockerfile.release.full) does copy the binary but uses release mode, so it also passes.

We note that if `debug-embed` is enabled, then the issue also disappears. This can be confirmed by looking at the CI in this commit: b117604
