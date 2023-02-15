# Changelog



## [0.4.8](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.8) - TBD

### New

* `traits::HexToUnsigned`



## [0.4.7](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.7) - 2023-01-26

### Changed

* Bump brunch `0.4`
* Fix ci badge (docs)



## [0.4.6](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.6) - 2022-11-03

### Changed

* Improved documentation.



## [0.4.5](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.5) - 2022-09-09

### Changed

* Optimize `NiceFloat::compact_bytes`/`NiceFloat::compact_str`

### Fixed

* `NiceFloat::precise_bytes`/`NiceFloat::precise_str` incorrectly truncated `NiceFloat::overflow`



## [0.4.4](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.4) - 2022-09-06

### New

* `dactyl::total_cmp!` (total float comparison)
* `FloatKind`
* `NiceFloat`

### Changed

* `NicePercent` output is now closer to `format!("{:0.02}%", num * 100.0)`, but will occasionally vary ±0.01% due to differences in rounding (`NicePercent` rounds up on `x.xxxx5`).



## [0.4.3](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.3) - 2022-09-02

### New

* `NiceElapsed::dhms`

### Changed

* `NiceElapsed` now supports "days"

### Deprecated

* `NiceElapsed::max` (moot now that days are supported)



## [0.4.2](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.2) - 2022-08-13

### New

* `traits::Inflection`
* `traits::NiceInflection`



## [0.4.1](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.1) - 2022-08-11

### Changed

* Bump MSRV `1.63`



## [0.4.0](https://github.com/Blobfolio/dactyl/releases/tag/v0.4.0) - 2022-06-18

### Changed

* Bump MSRV `1.61`
* The `NiceU*` and `NicePercent` structs are now type aliases
* `NicePercent::default` is now equivalent to `From<0.0>`
* `NiceU*::default` is now equivalent to `From<0>`
* Slightly faster `NiceU8`, `NiceU16` instantiation

### Added

* `dactyl::div_mod` (generic)
* impl `From<NicePercent>` for `String`
* impl `From<NicePercent>` for `Vec<u8>`
* impl `From<NiceU*>` for `String`
* impl `From<NiceU*>` for `Vec<u8>`
* impl `From<Option<T>>` (where `From<T>`) for `NicePercent`
* impl `From<Option<T>>` (where `From<T>`) for `NiceU*`

### Removed

* `dactyl::div_mod_u8` (use the new generic version instead)
* `dactyl::div_mod_u16`
* `dactyl::div_mod_u32`
* `dactyl::div_mod_u64`
* `dactyl::div_mod_u128`
* `dactyl::div_mod_usize`
* `dactyl::write_time`
* `dactyl::write_u8`
* `NicePercent::as_string` (use `From<NicePercent>` instead)
* `NicePercent::as_vec` (use `From<NicePercent>` instead)
* `NiceU*::as_string` (use `From<NiceU*>` instead)
* `NiceU*::as_vec` (use `From<NiceU*>` instead)



## [0.3.4](https://github.com/Blobfolio/dactyl/releases/tag/v0.3.4) - 2022-04-14

### Changed

* Make unit tests less glacial for `miri`

### Fixed

* Enable `num-traits` crate feature `i128` (needed for some targets)



## [0.3.3](https://github.com/Blobfolio/dactyl/releases/tag/v0.3.3) - 2022-03-28

### Added

* `dactyl::NoHash` (for `HashMap`, `HashSet`)
* `dactyl::traits::BytesToSigned` (slice to signed integer parsing)



## [0.3.2](https://github.com/Blobfolio/dactyl/releases/tag/v0.3.2) - 2022-03-27

### Added

* impl `BytesToUnsigned` for `NonZeroU*`



## [0.3.1](https://github.com/Blobfolio/dactyl/releases/tag/v0.3.1) - 2022-03-23

### Changed

* Faster `NiceU*` parsing, particularly for `NiceU8` and `NiceU16`
* Faster `NiceElapsed` parsing
* `NiceElapsed::from(Duration)` and `NiceElapsed::from(Instant)` now render fractional seconds (hundredths), e.g. `5 minutes and 0.02 seconds`



## [0.3.0](https://github.com/Blobfolio/dactyl/releases/tag/v0.3.0) - 2022-03-15

### New

* `dactyl::traits::BytesToUnsigned` (slice to unsigned integer parsing)
* `From<Instant>` for `NiceElapsed`

### Removed

* `dactyl::div_u128`
* `dactyl::div_u16`
* `dactyl::div_u32`
* `dactyl::div_u64`
* `dactyl::div_u8`
* `dactyl::div_usize`
* `GreaterThanZero`
* `GtZero`



## [0.2.4](https://github.com/Blobfolio/dactyl/releases/tag/v0.2.4) - 2022-01-29

### New

* `TryFrom<(T, T)>` for `NicePercent`;

### Deprecated

* `GreaterThanZero`
* `GtZero`



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
