/*!
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

* [`NiceU8`]
* [`NiceU16`]
* [`NiceU32`]
* [`NiceU64`] (also covers `usize`)
* [`NiceFloat`]
* [`NiceClock`] (for durations)
* [`NiceElapsed`] (also for durations)
* [`NicePercent`] (for floats representing percentages)

The intended use case is to simply call the appropriate `from()` for the type, then use either the `as_str()` or `as_bytes()` struct methods to retrieve the output in the desired format. Each struct also implements traits like `Display`, `AsRef<str>`, `AsRef<[u8]>`, etc., if you prefer those.

```
use dactyl::NiceU16;

assert_eq!(NiceU16::from(11234_u16).as_str(), "11,234");
assert_eq!(NiceU16::from(11234_u16).as_bytes(), b"11,234");
```

But the niceness doesn't stop there. Dactyl provides several other structs, methods, and traits to performantly work with integers, such as:

* [`NoHash`]: a passthrough hasher for integer `HashSet`/`HashMap` collections
* [`traits::BytesToSigned`]: signed integer parsing from byte slices
* [`traits::BytesToUnsigned`]: unsigned integer parsing from byte slices
* [`traits::HexToSigned`]: signed integer parsing from hex
* [`traits::HexToUnsigned`]: unsigned integer parsing from hex

*/

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
	unsafe_code,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

// #![expect(clippy::redundant_pub_crate, reason = "Unresolvable.")]



#[macro_use] mod macros;
mod hash;
mod nice;
pub mod traits;

pub use hash::NoHash;
pub use nice::{
	NiceClock,
	NiceElapsed,
	NiceFloat,
	NicePercent,
	NiceSeparator,
	NiceU16,
	NiceU32,
	NiceU64,
	NiceU8,
};

#[cfg(test)] use brunch as _;
