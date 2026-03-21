# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.3.1](https://github.com/Rolv-Apneseth/rgd/compare/v1.3.0...v1.3.1) - 2026-03-21

### Fixed

- *(lib_game_detector)* avoid failing when Lutris entries have no game directory specified

## [1.3.0](https://github.com/Rolv-Apneseth/rgd/compare/v1.2.2...v1.3.0) - 2026-03-14

### Added

- log package version for additional context in log output

### Fixed

- *(lib_game_detector)* improve Steam support for different distros by following symlinks
- *(lib_game_detector)* differentiate between Lutris entries which share an executable
- *(lib_game_detector)* parse actual instance names from Prism ([#14](https://github.com/Rolv-Apneseth/rgd/pull/14))

## [1.2.2](https://github.com/Rolv-Apneseth/rgd/compare/v1.2.1...v1.2.2) - 2025-12-01

### Fixed

- update `lib_game_detector` to not fail on a single invalid steam library ([#8](https://github.com/Rolv-Apneseth/rgd/pull/8))

## [1.2.1](https://github.com/Rolv-Apneseth/rgd/compare/v1.2.0...v1.2.1) - 2025-11-14

### Fixed

- bump `lib_game_detector`, `bundled_sqlite` should not be a default feature

## [1.2.0](https://github.com/Rolv-Apneseth/rgd/compare/v1.1.0...v1.2.0) - 2025-11-14

### Added

- update `lib_game_detector` for `Itch` support ([#6](https://github.com/Rolv-Apneseth/rgd/pull/6))

## [1.1.0](https://github.com/Rolv-Apneseth/rgd/compare/v1.0.0...v1.1.0) - 2025-10-06

### Added

- include `source` field for games ([#2](https://github.com/Rolv-Apneseth/rgd/pull/2))
