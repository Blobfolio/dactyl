# Changelog



## [0.2.4](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.4) - 2022-01-29

### New

* `TryFrom<(T, T)>` for `NicePercent`;

### Deprecated

* `GreaterThanZero`;
* `GtZero`;



## [0.2.3](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.3) - 2022-01-20

### New

* `NiceU*::with_separator` (except `NiceU8`, which can't reach 1000)
* `NiceU*::as_string`
* `NiceU*::as_vec`



## [0.2.2](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.2) - 2021-12-29

### Changed

* Implement `Ord`/`PartialOrd` for `NiceU*`;
* Only `Hash` filled bytes;
* Implement `Hash`, `Eq`, `PartialEq` for `NiceElapsed`;



## [0.2.1](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.1) - 2021-12-02

### Deprecated

* `dactyl::div_u128`
* `dactyl::div_u64`
* `dactyl::div_u32`
* `dactyl::div_u16`
* `dactyl::div_u8`
* `dactyl::div_usize`



## [0.2.0](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.0) - 2021-10-21

### Added

* This changelog! Haha.

### Changed

* Use Rust edition 2021.
