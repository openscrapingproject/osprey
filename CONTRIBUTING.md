# Contributing

Thanks for being here.

A few prerequisites:
- Rust Stable > 2018
- rustfmt nightly/unstable (we rely on an unstable comment formatting feature)

Then run 
```bash
git config --local core.hooksPath .githooks/
```

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