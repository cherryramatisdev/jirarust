# Jira CLI in rust

A cli that group jira and github together in a beatiful workflow 🦄.

## Usage

1. To install (In the future we'll have releases with pre built binaries)

```sh
cargo install jirarust
```

2. Run `jirarust config-set` and ask all the prompts.

3. Just be happy 🎉

```sh
jirarust --help
```

> Tip: Alias `jirarust` to `jira` for easier usage in your config.

## Shell completion

Replace `zsh` with your shell of choice and the write path to where you keep the completions.

```sh
jirarust --generate=zsh > ~/.zsh/functions/_jirarust
```

## Requirements

- [Github CLI](https://cli.github.com/)

## Goals

- [ ] Fix build github action to release pre built binaries for different platforms
- [X] Add tests
- [X] Add documentation
- [X] Add commands to view description of a card
- [ ] Add commands to move through other columns like (homol, done, etc)
- [ ] Make transition object less personal(today the number only works for my jira board I think)
