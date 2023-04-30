# outside-collaborators-delta-run
Outside Collaborators CLI to be able to do delta runs

## Purpose of this repository

### Context

GitHub has a way of adding outside collaborators to a repository.
This process is not very smooth, because there is no way to group
people together and assign these groups to repositories.

This is fine when dealing with small organizations, but when you
have a big organization with tens of thousands of repositories, this
becomes hard to manage.

Luckily [some nice people from icub-tech-itt](https://www.iit.it/web/icub-tech)
created [an Open Source project](https://github.com/icub-tech-iit/outside-collaborators)
to help with this problem.

### Scaling problem

The solution provided works really nice, but also has it's problems when 
dealing with lots of repositories and lots of users.

Everytime something has changed it will re-run the invitations for all
repositories again. And this can take up to hours to be finished.

One typo will make you have to wait again for several hours until it's finished.

In a [discussion on GitHub](https://github.com/icub-tech-iit/outside-collaborators/discussions/175)
I've suggested a way to allow for Delta Runs. Only apply changes to the groups
and repositories which have changes.

## Usage

### Command Line Interface

#### Download release

TODO

#### Execute tool

```
Usage: analyse-changed-groups-repos [OPTIONS] <COMMAND>

Commands:
  completion  Generate completion script
  search      Read a file and execute the correct search on it
  help        Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
  -V, --version     Print version
```

## Example how to use in GitHub Workflow

TODO

## Contributions

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Technology

- [Rust-lang](https://www.rust-lang.org/)
- Bash
- GitHub Actions
- [NixOs](https://nixos.org/)
- [Devenv.sh](https://devenv.sh/)

## License

[BSD 3-Clause License](./LICENSE)






