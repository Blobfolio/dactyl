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
* [`NicePercent`] (for percentagelike floats)

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
	clippy::implicit_clone,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
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



#[cfg(target_pointer_width = "16")]
/// # Helper: `isize`/`usize` Properties.
///
/// TODO: merge with `minmax` if/when `#[cfg]` starts working _inside_ a macro.
macro_rules! int_sized {
	(@min isize) => ( -32768 );
	(@min usize) => ( 0 );

	(@max isize) => ( 32767 );
	(@max usize) => ( 65535 );

	(@alias isize) => ( i16 );
	(@alias usize) => ( u16 );
}

#[cfg(target_pointer_width = "32")]
/// # Helper: `isize`/`usize` Properties.
macro_rules! int_sized {
	(@min isize) => ( -2147483648 );
	(@min usize) => ( 0 );

	(@max isize) => ( 2147483647 );
	(@max usize) => ( 4294967295 );

	(@alias isize) => ( i32 );
	(@alias usize) => ( u32 );
}

#[cfg(target_pointer_width = "64")]
/// # Helper: `isize`/`usize` Properties.
macro_rules! int_sized {
	(@min isize) => ( -9223372036854775808 );
	(@min usize) => ( 0 );

	(@max isize) => ( 9223372036854775807 );
	(@max usize) => ( 18446744073709551615 );

	(@alias isize) => ( i64 );
	(@alias usize) => ( u64 );
}

/// # Helper: Type Min and Max.
macro_rules! int {
	// Minimums.
	(@min i8) =>   ( -128 );
	(@min i16) =>  ( -32768 );
	(@min i32) =>  ( -2147483648 );
	(@min i64) =>  ( -9223372036854775808 );
	(@min i128) => ( -170141183460469231731687303715884105728 );
	(@min isize) =>( $crate::int_sized!(@min isize) );

	(@min u8) =>   ( 0 );
	(@min u16) =>  ( 0 );
	(@min u32) =>  ( 0 );
	(@min u64) =>  ( 0 );
	(@min u128) => ( 0 );
	(@min usize) =>( 0 );

	// Maximums.
	(@max i8) =>   ( 127 );
	(@max i16) =>  ( 32767 );
	(@max i32) =>  ( 2147483647 );
	(@max i64) =>  ( 9223372036854775807 );
	(@max i128) => ( 170141183460469231731687303715884105727 );
	(@max isize) =>( $crate::int_sized!(@max isize) );

	(@max u8) =>   ( 255 );
	(@max u16) =>  ( 65535 );
	(@max u32) =>  ( 4294967295 );
	(@max u64) =>  ( 18446744073709551615 );
	(@max u128) => ( 340282366920938463463374607431768211455 );
	(@max usize) =>( $crate::int_sized!(@max usize) );

	// Sign swap.
	(@flip u8) => ( i8 );
	(@flip u16) => ( i16 );
	(@flip u32) => ( i32 );
	(@flip u64) => ( i64 );
	(@flip u128) => ( i128 );
	(@flip usize) => ( isize );

	(@flip i8) => ( u8 );
	(@flip i16) => ( u16 );
	(@flip i32) => ( u32 );
	(@flip i64) => ( u64 );
	(@flip i128) => ( u128 );
	(@flip isize) => ( usize );

	// Aliases.
	(@alias NonZeroU8) => ( u8 );
	(@alias NonZeroU16) => ( u16 );
	(@alias NonZeroU32) => ( u32 );
	(@alias NonZeroU64) => ( u64 );
	(@alias NonZeroU128) => ( u128 );
	(@alias NonZeroUsize) => ( usize );
	(@alias NonZeroI8) => ( i8 );
	(@alias NonZeroI16) => ( i16 );
	(@alias NonZeroI32) => ( i32 );
	(@alias NonZeroI64) => ( i64 );
	(@alias NonZeroI128) => ( i128 );
	(@alias NonZeroIsize) => ( isize );
	(@alias isize) => ( $crate::int_sized!(@alias isize) );
	(@alias usize) => ( $crate::int_sized!(@alias usize) );
}

// Keep these private to the crate.
pub(crate) use int;
pub(crate) use int_sized;



#[cfg(test)]
mod tests {
	#[test]
	/// # Test `int!` macro.
	///
	/// Make sure the `MIN`/`MAX` constants agree with our hard-coded literals!
	fn t_int() {
		assert_eq!(i8::MIN,    int!(@min i8));
		assert_eq!(i8::MAX,    int!(@max i8));
		assert_eq!(i16::MIN,   int!(@min i16));
		assert_eq!(i16::MAX,   int!(@max i16));
		assert_eq!(i32::MIN,   int!(@min i32));
		assert_eq!(i32::MAX,   int!(@max i32));
		assert_eq!(i64::MIN,   int!(@min i64));
		assert_eq!(i64::MAX,   int!(@max i64));
		assert_eq!(i128::MIN,  int!(@min i128));
		assert_eq!(i128::MAX,  int!(@max i128));
		assert_eq!(isize::MIN, int!(@min isize));
		assert_eq!(isize::MAX, int!(@max isize));

		assert_eq!(u8::MIN,    int!(@min u8));
		assert_eq!(u8::MAX,    int!(@max u8));
		assert_eq!(u16::MIN,   int!(@min u16));
		assert_eq!(u16::MAX,   int!(@max u16));
		assert_eq!(u32::MIN,   int!(@min u32));
		assert_eq!(u32::MAX,   int!(@max u32));
		assert_eq!(u64::MIN,   int!(@min u64));
		assert_eq!(u64::MAX,   int!(@max u64));
		assert_eq!(u128::MIN,  int!(@min u128));
		assert_eq!(u128::MAX,  int!(@max u128));
		assert_eq!(usize::MIN, int!(@min usize));
		assert_eq!(usize::MAX, int!(@max usize));
	}
}
