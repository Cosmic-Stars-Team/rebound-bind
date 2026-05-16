# rebound-bind

Low-level Rust FFI bindings for the [REBOUND](https://github.com/hannorein/rebound) N-body simulation C library.

## Requirements

Building this crate requires:

- A C compiler
- `libclang` and Clang headers for `bindgen`
- The `rebound` submodule checked out

If you cloned this repository without submodules, initialize them first:

```sh
git submodule update --init --recursive
```

Some optional features require additional system libraries.

## Features

No features are enabled by default.

Available features:

- `server`: enable REBOUND server support
- `opengl`: enable OpenGL display support; requires GLFW
- `openmp`: enable OpenMP support
- `openmp-clang`: enable OpenMP support for Clang/libomp setups, commonly needed on macOS with Homebrew `libomp`
- `mpi`: enable MPI support; requires an MPI compiler and headers
- `fftw`: enable FFTW support; requires FFTW3
- `avx512`: enable AVX512 support
- `quadrupole`: enable quadrupole support
- `profiling`: enable profiling support

Example:

```sh
cargo add rebound-bind --features server
```

## Build notes

The build script compiles the C sources from `rebound/src` directly with the `cc` crate and generates Rust bindings from `rebound/src/rebound.h` with `bindgen`.

The source file list and compile flags are aligned with REBOUND `5.0.0`'s `src/Makefile` and `src/Makefile.defs`.

## License

This crate is licensed under GPL-3.0 and includes vendored REBOUND sources under the same license family. See `rebound/LICENSE` for the upstream license text.
