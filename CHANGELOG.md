# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### httpbin

- Use `clap` and `toml` to parse config file
- Add `base64::encode` and `base64::decode` functions

#### implementations

- poem-openapi, poem, axum, actix, salvo, rocket
  - `HTTP Methods` support
  - `Request inspection` support
  - `Anything` support
  - `Data` support: base64

#### chore

- CI to check fmt, lint, build and test
- CI to check CHANGELOG.md
- CI to build and publish docker image
- Add renovate to manage dependencies
