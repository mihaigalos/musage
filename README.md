# musage

[![CI](https://github.com/mihaigalos/musage/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/musage/actions/workflows/ci.yaml)
[![CD](https://github.com/mihaigalos/musage/actions/workflows/cd.yaml/badge.svg)](https://github.com/mihaigalos/musage/actions/workflows/cd.yaml)
[![crates.io](https://img.shields.io/crates/d/musage.svg)](https://crates.io/crates/musage)
[![LoC](https://tokei.rs/b1/github/mihaigalos/musage)](https://github.com/mihaigalos/musage)

A command line memory usage information tool.

![musage_example](screenshots/musage_example.png)


### Why?

A better interface for `free`.

### BTW

You might also like [`dusage`](https://github.com/mihaigalos/dusage).
Both can be i.e. automatically executed upon login via `ssh` do a remote machine by invoking them in the remote's `.bashrc` or `.zshrc`.

### Features

* bargraph with RAM and cache memory usage.
    * background: cache, foreground: RAM.

### Installation

##### Building from source

```bash
cargo install musage
```

##### Using precompiled binaries

Precompiled binaries are available for multiple architectures in [Releases](https://github.com/mihaigalos/musage/releases).

