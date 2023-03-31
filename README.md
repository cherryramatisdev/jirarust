# Jira CLI in rust

A cli that group jira and github together in a beatiful workflow 🦄.

## Usage

1. To install (In the future we'll have releases with pre built binaries)

```sh
cargo install jirarust
```

2. Add the required configuration under `~/.jira_config.json`
> DISCLAIMER: You can use the env var `JIRA_CONFIG_LOG` to change the config file location.

The json file should look like this:
```json
{
  "auth": {
    "user_mail": "<the email you use on your account>",
    "user_token": "<your personal access token>",
    "profile_id": "<the id for your profile>"
  },
  "prefixes": {
    "card_prefix": "<the prefix of your jira cards (normally depend on your board like TEC-xxx)>",
    "url_prefix": "<the url of your jira instance>"
  },
  "git": {
    "feature_tag": "<the github label you use for features>",
    "fix_tag": "<the github label you use for bugfixes>",
  }
}
```

3. Just run and be happy 🎉

```sh
jirarust
```

> Tip: Change the binary name to `jira` or something better for your use.

## Requirements

- [Github CLI](https://cli.github.com/)

## Goals

- [ ] Fix build github action to release pre built binaries for different platforms
- [ ] Add tests
- [X] Add documentation
- [X] Add commands to view description of a card
- [ ] Add commands to move through other columns like (homol, done, etc)
- [ ] Make transition object less personal(today the number only works for my jira board I think)
