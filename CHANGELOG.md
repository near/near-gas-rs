# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2](https://github.com/near/near-gas-rs/compare/v0.3.1...v0.3.2) - 2025-08-23

### Added

- Improvements to OpenAPI and added schemars v1 support ([#21](https://github.com/near/near-gas-rs/pull/21))
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/near/near-gas-rs/compare/v0.3.2...v0.3.3) - 2025-12-02

### Added

- Added i64 deserialization support to fix `bson` compatibility ([#25](https://github.com/near/near-gas-rs/pull/25))
- Support both schemars versions simultaneously ([#23](https://github.com/near/near-gas-rs/pull/23))

## [0.3.1](https://github.com/near/near-gas-rs/compare/v0.3.0...v0.3.1) - 2025-07-18

### Added

- allowed to deserialize from u64 ([#19](https://github.com/near/near-gas-rs/pull/19))

### Other

- Use ubuntu-latest for CI runners instead of the deprecated ubuntu-20.04

## [0.3.0](https://github.com/near/near-gas-rs/compare/v0.2.5...v0.3.0) - 2024-08-12

### Other
- Extended the range of supported interactive-clap versions to be >=0.2,<0.4 ([#15](https://github.com/near/near-gas-rs/pull/15))

## [0.2.7](https://github.com/near/near-gas-rs/compare/v0.2.5...v0.2.7) - 2024-08-12 (yanked due to [invalid interactive-clap version](https://users.rust-lang.org/t/cargo-duplicating-dependency-when-it-seems-like-it-shouldnt-be/87883/6) gets pulled in new projects)

### Other
- Extended the range of supported interactive-clap versions to be >=0.2,<0.4 ([#15](https://github.com/near/near-gas-rs/pull/15))

## [0.2.6](https://github.com/near/near-gas-rs/compare/v0.2.5...v0.2.6) - 2024-08-12 (yanked due to invalid interactive-clap version range)

### Other
- Extended the range of supported interactive-clap versions ([#15](https://github.com/near/near-gas-rs/pull/15))

## [0.2.5](https://github.com/near/near-gas/compare/v0.2.4...v0.2.5) - 2023-10-23

### Other
- Update README.md
- Fixed typo and added badges in README.md ([#12](https://github.com/near/near-gas/pull/12))

## [0.2.4](https://github.com/near/near-gas/compare/v0.2.3...v0.2.4) - 2023-10-22

### Other
- Small cleanups in various places ([#10](https://github.com/near/near-gas/pull/10))
- Update `borsh`, separate `borsh/unstable__schema` and `schemars` under `abi` feature ([#8](https://github.com/near/near-gas/pull/8))
- Add github actions jobs from rust template ([#9](https://github.com/near/near-gas/pull/9))
- Split lib.rs into smaller files to ease testing, extension, and review ([#5](https://github.com/near/near-gas/pull/5))
