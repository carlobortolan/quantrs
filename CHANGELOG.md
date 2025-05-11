# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.7] - 2025-05-11

### Added

- Add Black-76 pricing model
- Implement lookback options
- Implement bermudan options

### Changed

- Updated dependencies to latest versions

### Fixed

- Fix time spread and flaky strategies

## [0.1.6] - 2025-03-22

### Added

- Added option pricing models and support for vanilla and exotic options
- Added support for non-directional option strategies
- Option strategy payoff and price curve visualization
- Support for option strategy visualization with `plot_strategy_breakdown`

### Changed

- Upgraded dependency versions
- Changed MSRV from 1.65.0 to 1.77.0
- Added new tests (90.7% project coverage)

### Fixed

- Fixed incorrect calculations in condor strategy implementation
- Resolved issues with fontconfig in CI pipeline
- Fixed margin spacing in plot visualization

[Unreleased]: https://github.com/carlobortolan/quantrs/compare/v0.1.7...HEAD
[0.1.7]: https://github.com/carlobortolan/quantrs/releases/tag/v0.1.7
[0.1.6]: https://github.com/carlobortolan/quantrs/releases/tag/v0.1.6
