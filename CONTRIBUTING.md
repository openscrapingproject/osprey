# Contributing

Thanks for being here.

A few prerequisites:
- Rust Stable > 2018
- rustfmt nightly/unstable (we rely on an unstable comment formatting feature)

Then run 
```bash
git config --local core.hooksPath .githooks/
```

## Project Structure

We recently moved to using Cargo workspaces to separate the libraries and CLI into separate crates.

`osplib` is the library. `osprey` is the CLI.

## Build docs
```sh
cargo +nightly doc --open --no-deps
```

## Format
```
cargo +nightly fmt
```

## Code stats/complexity

Install https://github.com/boyter/scc.

```bash
scc -i rs,json,md
```

## Useful cargo addons

- cargo-add
- cargo-geiger
- cargo-expand
- cargo-modules
- cargo-outdated