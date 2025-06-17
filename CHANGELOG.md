# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.56] - 2025-06-17

### Changed

- Add release support (#52) by @ThetaSinner in [#52](https://github.com/holochain/holochain-serialization/pull/52)
- Add ci_pass check (#51) by @ThetaSinner in [#51](https://github.com/holochain/holochain-serialization/pull/51)
- Maintenance 03-25 (#50) by @ThetaSinner in [#50](https://github.com/holochain/holochain-serialization/pull/50)
- Merge pull request #49 from holochain/bump-deps by @ThetaSinner in [#49](https://github.com/holochain/holochain-serialization/pull/49)

## [0.0.54] - 2024-04-17

## Changed
- **BREAKING**: Updated dependencies `rmp-serde` and `serde`. Serialization format for Rust enums changed.
An enum
```rust
enum ConductorApi {
    Request { param: i32 },
}
let request = ConductorApi::Request { param: 100 };
```
previously serialized to
```json
{
    "type": {
        "request": null
    },
    "data": {
        "param": 100
    }
}
```
and now serializes to
```json
{
    "type": "request",
    "data": {
        "param": 100
    }
}
```

## [0.0.53] - 2023-08-29

## Added

- Added proptest derives

## [0.0.52] - 2023-01-17

## Added

- Basic fuzz testing

## [0.0.51] - 2021-07-20

### Added

- `impl Arbitrary for SerializedBytes`

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.50] - 2021-02-21

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.49] - 2021-02-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.47] - 2021-02-02

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.46] - 2021-02-02

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.47] - 2020-12-22

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.46] - 2020-12-17

### Added

### Changed
- `SerializedBytesError` can from `Infallible`.
- `SerializedBytes` can `TryFrom<&SerializedBytes>` by cloning.

### Deprecated

### Removed

### Fixed

### Security

## [0.0.45] - 2020-12-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.45] - 2020-09-18

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.44] - 2020-09-18

### Added

- PartialOrd and Ord impls for `SerializedBytes`

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.43] - 2020-08-28

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.42] - 2020-08-17

### Added

### Changed

- Uses `with_struct_map` and `with_string_variants` rmp_serde config for more robust serialization

### Deprecated

### Removed

### Fixed

### Security

## [0.0.41] - 2020-08-05

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.40] - 2020-06-15

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.39] - 2020-05-02

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.38] - 2020-05-02

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.38] - 2020-04-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.37] - 2020-04-14

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.36] - 2020-03-29

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.35] - 2020-03-29

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.34] - 2020-03-29

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.33] - 2020-03-29

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.32] - 2020-03-27

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.31] - 2020-03-27

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.30] - 2020-03-15

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.29] - 2020-03-15

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.28] - 2020-03-14

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.26] - 2020-03-14

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.25] - 2020-03-14

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.24] - 2020-03-14

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.23] - 2020-02-13

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.22] - 2020-02-11

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.21] - 2020-02-10

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.20] - 2020-02-10

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.19] - 2020-02-10

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.17] - 2020-02-10

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.17] - 2019-08-05

### Added

### Changed

- revert to holonix 0.0.20

### Deprecated

### Removed

### Fixed

### Security

## [0.0.16] - 2019-08-05

### Added

### Changed

- updated holonix to 0.0.22
- updated futures crates to 0.3.0-alpha17

### Deprecated

### Removed

### Fixed

### Security

## [0.0.15] - 2019-07-15

### Added

### Changed

- using holonix `0.0.20`

### Deprecated

### Removed

### Fixed

### Security

## [0.0.14] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.13] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.12] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.11] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.10] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.9] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.8] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

- fixing publishing bugs

### Security

## [0.0.7] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.6] - 2019-07-09

### Added

- added publish release hook

### Changed

- remove readme version hook

### Deprecated

### Removed

### Fixed

### Security

## [0.0.5] - 2019-07-09

## [0.0.4] - 2019-07-09

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security
