# Local core2 compatibility shim

This crate is a local compatibility shim for `core2` 0.3.3.

Why it exists:

- `core2` 0.3.x was yanked from crates.io.
- CI jobs that intentionally resolve without `Cargo.lock` can no longer fetch `core2 = "^0.3"`.
- Several `librustzcash` crates and patched upstream dependencies still import `core2::io`.

Implementation:

- Package name remains `core2`, version `0.3.3`, so existing imports and dependency constraints keep working.
- Source is derived from `core3` 0.1.2, the maintained successor to `core2`.
- The exposed surface is the no_std I/O compatibility layer used by existing dependents.
- The crate is marked `publish = false`; it exists only to keep this workspace reproducible.

Licensing:

- Upstream licensing is `Apache-2.0 OR MIT`.
- Both license files are preserved in this directory.
