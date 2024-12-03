/*!
# Dactyl

[![docs.rs](https://img.shields.io/docsrs/dactyl.svg?style=flat-square&label=docs.rs)](https://docs.rs/dactyl/)
[![changelog](https://img.shields.io/crates/v/dactyl.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/dactyl/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/dactyl.svg?style=flat-square&label=crates.io)](https://crates.io/crates/dactyl)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/dactyl/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/dactyl/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/dactyl/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/dactyl)<br>
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

The intended use case is to simply call the appropriate `from()` for the type, then use either the `as_str()` or `as_bytes()` struct methods to retrieve the output in the desired format. Each struct also implements traits like `Deref`, `Display`, `AsRef<str>`, `AsRef<[u8]>`, etc., if you prefer those.

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

#![expect(clippy::module_name_repetitions, reason = "Repetition is preferred.")]
#![expect(clippy::redundant_pub_crate, reason = "Unresolvable.")]



#[macro_use] mod macros;
mod hash;
mod nice_elapsed;
mod nice_int;
pub mod traits;

pub use hash::NoHash;
pub use nice_elapsed::{
	clock::NiceClock,
	NiceElapsed,
};
pub use nice_int::{
	nice_u8::NiceU8,
	nice_u16::NiceU16,
	nice_u32::NiceU32,
	nice_u64::NiceU64,
	nice_float::{
		FloatKind,
		NiceFloat,
	},
	nice_percent::NicePercent,
};

#[doc(hidden)]
pub use nice_int::NiceWrapper;



/// # Decimals, 00-99.
static DOUBLE: [[u8; 2]; 100] = [
	[48, 48], [48, 49], [48, 50], [48, 51], [48, 52], [48, 53], [48, 54], [48, 55], [48, 56], [48, 57],
	[49, 48], [49, 49], [49, 50], [49, 51], [49, 52], [49, 53], [49, 54], [49, 55], [49, 56], [49, 57],
	[50, 48], [50, 49], [50, 50], [50, 51], [50, 52], [50, 53], [50, 54], [50, 55], [50, 56], [50, 57],
	[51, 48], [51, 49], [51, 50], [51, 51], [51, 52], [51, 53], [51, 54], [51, 55], [51, 56], [51, 57],
	[52, 48], [52, 49], [52, 50], [52, 51], [52, 52], [52, 53], [52, 54], [52, 55], [52, 56], [52, 57],
	[53, 48], [53, 49], [53, 50], [53, 51], [53, 52], [53, 53], [53, 54], [53, 55], [53, 56], [53, 57],
	[54, 48], [54, 49], [54, 50], [54, 51], [54, 52], [54, 53], [54, 54], [54, 55], [54, 56], [54, 57],
	[55, 48], [55, 49], [55, 50], [55, 51], [55, 52], [55, 53], [55, 54], [55, 55], [55, 56], [55, 57],
	[56, 48], [56, 49], [56, 50], [56, 51], [56, 52], [56, 53], [56, 54], [56, 55], [56, 56], [56, 57],
	[57, 48], [57, 49], [57, 50], [57, 51], [57, 52], [57, 53], [57, 54], [57, 55], [57, 56], [57, 57]
];

#[inline]
/// # Double Digits.
///
/// Return both digits, ASCII-fied.
///
/// ## Panics
///
/// This will panic if the number is greater than 99.
pub(crate) const fn double(idx: usize) -> [u8; 2] { DOUBLE[idx] }

#[inline]
#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
#[expect(clippy::integer_division, reason = "We want this.")]
/// # Triple Digits.
///
/// Return both digits, ASCII-fied.
///
/// ## Panics
///
/// This will panic if the number is greater than 999.
pub(crate) const fn triple(idx: usize) -> [u8; 3] {
	assert!(idx < 1000, "Bug: Triple must be less than 1000.");
	let (div, rem) = (idx / 100, idx % 100);
	let a = div as u8 + b'0';
	let [b, c] = DOUBLE[rem];
	[a, b, c]
}



#[cfg(test)]
mod test {
	use super::*;
	use brunch as _;

	#[test]
	fn t_triple() {
		// Note this also tests double().
		for i in 0..=999 {
			assert_eq!(
				format!("{i:03}").as_bytes(),
				triple(i),
				"Invalid triple conversion for {i}"
			);
		}
	}
}
