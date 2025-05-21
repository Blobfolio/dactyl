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



#[derive(Debug, Clone, Eq, PartialEq)]
/// # Popping Digiterator.
///
/// This struct is used internally by the library's various `Nice*` structs to
/// help stringify numbers.
///
/// It employs a naive divide-by-ten strategy to "pop" digits off the end one
/// at a time, and an equally naive `n + b'0'` to convert them to ASCII for
/// return.
///
/// (For our purposes, on-the-fly calculations are usually more performant
/// than static lookup tables, and not too much worse the rest of the time.)
///
/// Depending on the situation, this either returns all digits last-to-first
/// (via `Iterator`), or only the last 1-2 digits as a fixed, zero-padded
/// `[u8;2]` (via `double`).
pub(crate) struct Digiter<T>(pub(crate) T);

/// # Helper: Primitive Implementations.
macro_rules! digiter {
	($($ty:ty),+) => ($(
		#[allow(
			dead_code,
			clippy::allow_attributes,
			trivial_numeric_casts,
			reason = "Macro made me do it.",
		)]
		impl Digiter<$ty> {
			/// # New Instance.
			///
			/// Return a new [`Digiter`] for a given value, unless zero.
			///
			/// This is only necessary for iteration purposes; for one-off
			/// crunching it can instantiated directly to service any number,
			/// including zero.
			pub(crate) const fn new(num: $ty) -> Option<Self> {
				if num == 0 { None }
				else { Some(Self(num)) }
			}

			#[must_use]
			/// # Double.
			///
			/// Return the last two digits as ASCII bytes, zero-padded as
			/// necessary.
			///
			/// Note this is independent of iteration and will always return
			/// the same result for a given wrapped value.
			pub(crate) const fn double(self) -> [u8; 2] {
				let Self(mut num) = self;
				let b = (num % 10) as u8 + b'0';
				num /= 10;
				let a = (num % 10) as u8 + b'0';
				[a, b]
			}
		}

		impl Iterator for Digiter<$ty> {
			type Item = u8;

			#[allow(
				clippy::allow_attributes,
				trivial_numeric_casts,
				reason = "Macro made me do it.",
			)]
			#[inline]
			/// # Digit Iteration.
			///
			/// Read and return each digit, right to left.
			///
			/// This will not work if the starting value is zero; `Digiter::new`
			/// should be used for initialization to rule out that possibility.
			fn next(&mut self) -> Option<Self::Item> {
				if self.0 == 0 { None }
				else {
					let next = (self.0 % 10) as u8 + b'0';
					self.0 = self.0.wrapping_div(10);
					Some(next)
				}
			}

			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				let len = self.len();
				(len, Some(len))
			}
		}

		impl ExactSizeIterator for Digiter<$ty> {
			#[inline]
			fn len(&self) -> usize {
				// Zero marks the end for the iterator.
				if self.0 == 0 { 0 }
				else { self.0.ilog10() as usize + 1 }
			}
		}
	)+);
}

digiter!(u8, u16, u32, u64);



#[cfg(test)]
mod test {
	use super::*;
	use brunch as _;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri is way too slow for a million tests!

	/// # Helper: Digiter test for one specific value.
	macro_rules! t_digits {
		($num:ident, $ty:ty) => (
			// The expected number.
			let expected = $num.to_string();

			// Make sure we can digitize it.
			let Some(iter) = Digiter::<$ty>::new($num) else {
				panic!(
					concat!("Digiter::new failed with {num}_", stringify!($ty)),
					num=expected,
				);
			};

			// Verify the iter's reported length matches.
			assert_eq!(
				iter.len(),
				expected.len(),
				concat!("Digiter::len invalid for {num}_", stringify!($ty)),
				num=expected,
			);

			// Collect the results and reverse, then verify we got it right!
			let mut digits = iter.collect::<Vec<u8>>();
			digits.reverse();
			assert_eq!(
				String::from_utf8(digits).ok().as_deref(),
				Some(expected.as_str()),
			);
		);
	}

	#[test]
	fn t_digiter_u8() {
		// Zero is a no.
		assert!(Digiter::<u8>::new(0).is_none());

		// Everything else should be happy!
		for i in 1..=u8::MAX { t_digits!(i, u8); }
	}

	#[test]
	fn t_digiter_u16() {
		// Zero is a no.
		assert!(Digiter::<u16>::new(0).is_none());

		#[cfg(not(miri))]
		for i in 1..=u16::MAX { t_digits!(i, u16); }

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u16(..)).take(SAMPLE_SIZE) {
				t_digits!(i, u16);
			}

			// Explicitly check the max works.
			let i = u16::MAX;
			t_digits!(i, u16);
		}
	}

	#[test]
	fn t_digiter_u32() {
		// Zero is a no.
		assert!(Digiter::<u32>::new(0).is_none());

		// Testing the full range takes too long.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(..)).take(SAMPLE_SIZE) {
			t_digits!(i, u32);
		}

		// Explicitly check the max works.
		let i = u32::MAX;
		t_digits!(i, u32);
	}

	#[test]
	fn t_digiter_u64() {
		// Zero is a no.
		assert!(Digiter::<u64>::new(0).is_none());

		// Testing the full range takes too long.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(..)).take(SAMPLE_SIZE) {
			t_digits!(i, u64);
		}

		// Explicitly check the max works.
		let i = u64::MAX;
		t_digits!(i, u64);
	}
}
