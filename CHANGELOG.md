# Changelog
All notable changes to this project will be documented in this file.

## [0.1.1] - 2023-03-16

### Added

- Add setup guide to `README.md`

### Changed

- Update checkbox goals on `README.md`

## [0.1.3] - 2023-03-23

### Changed

- Update order of arguments(the title) on the create pr action.

## [0.2.0] - 2023-03-27

### Added

- Add tests to common services(still missing a lot)
- Add build script for easy installlation

### Changed

- Fix label if statement.
- General refactors.

## [0.3.0] - 2023-03-28

### Added

- Add possibility to configure via JSON file instead of
  environment variables
- Add custom log helper function

## [0.3.1] - 2023-03-31

### Changed

- Improve `progress` command to only create a new
  branch if it doesn't already exists.
- Documentation is now sync with the new config module
  based on json.
- Refactor module `create_branch` to be more
  flexible(now it's change_to_branch with an bool to
  create or not).

## [0.3.2] - 2023-03-31

### Changed

- Stop asking for branch type prompt when branch already exists.

## [0.3.3] - 2023-04-08

### Added

- Configure `thiserror` crate.

### Changed

- Solve error with `progress` command

## [0.3.4] - 2023-04-14

### Changed

- Update `review` command to don't try to create a PR
  that already exists.

## [0.3.5] - 2023-04-14

### Changed

- Update `review` command to be run without any card code, if this happen it'll just create a pr.

## [0.3.6] - 2023-04-14

### Added

- Add new command `config-set` to interactively generate a new configuration file.
- Add flag `--generate` to create shell completion scripts.

## [0.3.7] - 2023-04-15

### Added

- Add new command `homol` to move card to Homol column.
- Add new command `done` to move card to Done column.

### Changed

- Now all the commands can be run without the code, if
  it's run this way it'll try to get the code from the
  current checkout branch.

## [0.3.8] - 2023-05-01

### Added

- New flag added `disable` that can disable the jira or the git part of
`review` and `progress` command

### Changed

- On `review` command if you run without the code we check the current
branch, if the current branch has a jira_code we proceed with the
default behavior, if not we just create a new PR without any title.

## [0.3.9] - 2023-05-22

### Changed

- Add new function to check if `develop` branch exists. It's should not change the user experience of the command, just a optimization of function.
