# `lucet-wasi-sdk` &nbsp; [![docs-badge]][docs-rs]

[docs-badge]: https://docs.rs/lucet-wasi-sdk/badge.svg
[docs-rs]: https://docs.rs/lucet-wasi-sdk

[`wasi-sdk`](https://github.com/WebAssembly/wasi-sdk) is a Cranelift project that packages a build
of the Clang toolchain, the WASI reference sysroot, and a libc based on WASI syscalls.

`lucet-wasi-sdk` is a Rust crate that provides wrappers around these tools for building C programs
into Lucet modules. We use this crate to build test cases in `lucet-runtime-tests` and `lucet-wasi`.
