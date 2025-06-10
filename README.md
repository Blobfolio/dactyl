# Dactyl

[![docs.rs](https://img.shields.io/docsrs/dactyl.svg?style=flat-square&label=docs.rs)](https://docs.rs/dactyl/)
[![changelog](https://img.shields.io/crates/v/dactyl.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/dactyl/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/dactyl.svg?style=flat-square&label=crates.io)](https://crates.io/crates/dactyl)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/dactyl/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/dactyl/actions)
[![deps.rs](https://deps.rs/crate/dactyl/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/dactyl/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/dactyl/issues)

This crate provides a fast interface to "stringify" unsigned integers, formatted with commas at each thousand. It prioritizes speed and simplicity over configurability.

If your application just wants to quickly turn `1010` into `"1,010"`, Dactyl is a great choice. If your application requires locale awareness or other options, something like [`num-format`](https://crates.io/crates/num-format) would probably make more sense.

Similar to [`itoa`](https://crates.io/crates/itoa), Dactyl writes ASCII conversions to a temporary buffer, but does so using fixed arrays sized for each type's maximum value, minimizing the allocation overhead for, say, tiny little `u8`s.

Each type has its own struct, each of which works exactly the same way:

* `NiceU8`
* `NiceU16`
* `NiceU32`
* `NiceU64` (also covers `usize`)
* `NiceFloat`
* `NiceClock` (for durations)
* `NiceElapsed` (also for durations)
* `NicePercent` (for percentage-like floats)

The intended use case is to simply call the appropriate `from()` for the type, then use either the `as_str()` or `as_bytes()` struct methods to retrieve the output in the desired format. Each struct also implements traits like `Display`, `AsRef<str>`, `AsRef<[u8]>`, etc., if you prefer those.

```rust
use dactyl::NiceU16;

assert_eq!(NiceU16::from(11234_u16).as_str(), "11,234");
assert_eq!(NiceU16::from(11234_u16).as_bytes(), b"11,234");
```

But the niceness doesn't stop there. Dactyl provides several other structs, methods, and traits to performantly work with integers, such as:

* `NoHash`: a passthrough hasher for integer `HashSet`/`HashMap` collections
* `traits::BytesToSigned`: signed integer parsing from byte slices
* `traits::BytesToUnsigned`: unsigned integer parsing from byte slices
* `traits::HexToSigned`: signed integer parsing from hex
* `traits::HexToUnsigned`: unsigned integer parsing from hex



## Installation

Add `dactyl` to your `dependencies` in `Cargo.toml`, like:

```toml
[dependencies]
dactyl = "0.12.*"
```
