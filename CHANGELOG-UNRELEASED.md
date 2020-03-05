# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

{{ version-heading }}

### Added

- JsonString::from_json_unchecked added to skip validity checks
- TryFrom<&str> added for JsonString

### Changed

- JsonString::from_bytes and JsonString::from_json enforce valid json data

### Deprecated

### Removed

- From<&str> removed for JsonString (replaced with TryFrom)

### Fixed

### Security
