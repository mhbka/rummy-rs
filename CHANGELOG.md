# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/mhbka/rummy-rs/compare/v0.1.4...v0.2.0) - 2025-09-26

### Fixed

- fixed some clippy warnings + disabled in CI cuz fk it lol
- fix test stuff
- fixed some bugs in replay + wrote basic tests
- fix borken tests n stuff
- fixing + writing tests
- fix quit logic
- fixing more logic bugs and stuff

### Other

- final small changes
- debuging workflow
- add a release workflow + check for its updates
- add changelog
- update ver
- disabled doctests
- a ton of docs
- start writing docs (starting with cards)
- fmt
- fmt
- ran clippy
- add more steps to GH Actions workflow + fmt
- cargo fix
- made GameState fields private + fixed that for everything else
- add serialization test + finally fixed it (why trait bound like that? who knows)
- refactor example program + fix a bug and modify layoff API (and other small things)
- oopsie on mod tests
- readme
- deriving more gated serde la la la
- added a serde crate feature + serialization module for serializing BasicRummyGame
- start feature gating serde
- rearrange the cargo file + fix up errors
- small change
- wrote a new README amongst other thigns
- adding wrapper tests (history first)
- finish up replay wrapper
- small API change + writing replay wrapper
- added wrappers + wrote most of the history wrapper
- more tests + fixed an edge case here n there
- add more tests
- write tests + fix a discovered bug (im bad at writing code
- a ton of refactoring to make stuff more sensible i guess
- replaced old game module
- example seems to be working
- refactor example into separate files
- finished example
- few small API changes + work on example
- rewrote game
- added multiple fn to Meld API
- progress
- start working on basic variant
- starting rewrite

### Removed

- removed possible_plays wrapper (will add it as a separate feature later)
- removed ActionOutcome as it was unnecessary

## [2.0.0] - 26-09-2025

### Added

- Complete rewrite of the entire crate
- Started maintaining changelog

### Removed

- Old implementations of most things within the crate