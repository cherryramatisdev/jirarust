# Jira CLI in rust

A cli that group jira and github together in a beatiful workflow 🦄.

## Requirements

- This CLI interact direrctly with multiple remote providers (gitlab and github for now) and for that it's necessary to define your personal usage token with proper access to generate pull/merge request
  - For that please define either `GITHUB_TOKEN` or `gitlab_token` as environment variables in your system and make sure the token has the proper accesses

> Documentation about tokens on github: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens
>
> Documentation about tokens on gitlab: https://docs.gitlab.com/user/profile/personal_access_tokens/

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

## Goals

- [X] Fix build github action to release pre built binaries for different platforms
- [X] Add tests
- [X] Add documentation
- [X] Add commands to view description of a card
- [ ] Add commands to move through other columns like (homol, done, etc)
- [ ] Make transition object less personal(today the number only works for my jira board I think)
