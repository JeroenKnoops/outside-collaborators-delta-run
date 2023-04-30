# Contributing

## Development

This repository uses [NixOs](https://nixos.org) and [Devenv.sh](https://devenv.sh)
to align the development environments.

These tools will make sure the pre-commit hooks are setup, and the versions of 
the tools are locked in a lock file called: `devenv.lock`.

The CI build pipeline is using the same environment.

## Installation tools on local machine

### `Nix` and `Devenv`

See [installation guide](https://devenv.sh/getting-started/) to install Nix and Devenv.

### `Direnv`

You can automatically switch to this environment by using [Direnv](https://direnv.net/).
See instructions: https://devenv.sh/automatic-shell-activation/

:warning: Don't forget to do a `direnv allow` in the working directory.

## Development

You can use Cargo to build the project:

### build

```bash
cargo build
```

### test

```bash
cargo test
```

### run

```bash
cargo run -- help
```

### run ci pre-commit hooks

```bash
devenv ci
```

## Contribute

We will try to respond as soon as possible.

### Issues

You can contribute by creating an issue or discussion if something is not clear.

### PRs

If you want to fix something, please create a fork and create a PR.
