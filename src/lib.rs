/*!
# Dactyl

This crate provides a fast interface to "stringify" unsigned integers, formatted with commas at each thousand. It prioritizes speed and simplicity over configurability.

If your application just wants to turn `1010` into `"1,010"`, `Dactyl` is a great choice. If your application requires locale awareness or other options, something like [`num-format`](https://crates.io/crates/num-format) would probably make more sense.

Similar to [`itoa`](https://crates.io/crates/itoa), Dactyl writes ASCII conversions to a temporary buffer, but does so using fixed arrays sized for each type's maximum value, minimizing the allocation overhead for, say, tiny little `u8`s.

Each type has its own struct, each of which works exactly the same way:

* [`NiceU8`]
* [`NiceU16`]
* [`NiceU32`]
* [`NiceU64`]

(Note: support for `usize` values is folded into [`NiceU64`].)

The intended use case is to simply call the appropriate `from()` for the type, then use either the `as_str()` or `as_bytes()` struct methods to retrieve the output in the desired format. Each struct also implements traits like `Deref`, `Display`, `AsRef<str>`, `AsRef<[u8]>`, etc., if you prefer those.

```
use dactyl::NiceU16;

assert_eq!(NiceU16::from(11234_u16).as_str(), "11,234");
assert_eq!(NiceU16::from(11234_u16).as_bytes(), b"11,234");
```

This crate also contains two "in development" structs — [`NicePercent`] and [`NiceElapsed`] — that can be useful for formatting percentages and durations, however their implementations are subject to change and they might eventually be split off into their own crates.
*/

#![deny(unsafe_code)]

#![warn(
	clippy::filetype_is_file,
	clippy::integer_division,
	clippy::needless_borrow,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suboptimal_flops,
	clippy::unneeded_field_pattern,
	macro_use_extern_crate,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![allow(
	clippy::module_name_repetitions,
	clippy::redundant_pub_crate
)] // This is fine.



mod hash;
mod nice_elapsed;
mod nice_int;
pub mod traits;

pub use hash::NoHash;
pub use nice_elapsed::NiceElapsed;
pub use nice_int::{
	nice_u8::NiceU8,
	nice_u16::NiceU16,
	nice_u32::NiceU32,
	nice_u64::NiceU64,
	nice_percent::NicePercent,
};

#[doc(hidden)]
pub use nice_int::NiceWrapper;

use num_traits::cast::AsPrimitive;



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

#[allow(unsafe_code)]
#[inline]
/// # Double Pointer.
///
/// This produces a pointer to a specific two-digit subslice of `DOUBLE`.
///
/// ## Panics
///
/// This will panic if the number is greater than 99.
pub(crate) fn double_ptr(idx: usize) -> *const u8 {
	debug_assert!(idx < 100, "BUG: Invalid index passed to double_ptr.");
	unsafe { DOUBLE.get_unchecked(idx).as_ptr() }
}

/// # Double Digits.
///
/// Return both digits, ASCII-fied.
///
/// ## Panics
///
/// This will panic if the number is greater than 99.
pub(crate) fn double(idx: usize) -> [u8; 2] { DOUBLE[idx] }



#[must_use]
/// # Combined Division/Remainder.
///
/// Perform division and remainder operations in one go, returning both results
/// as a tuple.
///
/// Nothing fancy happens here. This is just more convenient than performing
/// each operation individually.
///
/// ## Examples
///
/// ```
/// // Using the div_mod one-liner.
/// assert_eq!(
///     dactyl::div_mod(10_u32, 3_u32),
///     (3_u32, 1_u32),
/// );
///
/// // Or the same thing, done manually.
/// assert_eq!(
///     (10_u32 / 3_u32, 10_u32 % 3_u32),
///     (3_u32, 1_u32),
/// );
/// ```
///
/// ## Panics
///
/// This will panic if the denominator is set to zero or if the result of
/// either operation would overflow, like `i8::MIN / -1_i8`.
pub fn div_mod<T>(e: T, d: T) -> (T, T)
where T: Copy + std::ops::Div<Output=T> + std::ops::Rem<Output=T> { (e / d, e % d) }

#[must_use]
/// # Integer to Float Division.
///
/// This uses [`num_traits::cast`](https://docs.rs/num-traits/latest/num_traits/cast/index.html) to convert primitives to `f64` as accurately
/// as possible, then performs the division. For very large numbers, some
/// rounding may occur.
///
/// If the result is invalid, NaN, or infinite, `None` is returned.
pub fn int_div_float<T>(e: T, d: T) -> Option<f64>
where T: AsPrimitive<f64> {
	let d: f64 = d.as_();

	// The denominator can't be zero.
	if d == 0.0 { None }
	else {
		Some(e.as_() / d).filter(|x| x.is_finite())
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;

	#[test]
	fn t_int_div_float() {
		assert_eq!(int_div_float(4_000_000_000_u64, 8_000_000_000_u64), Some(0.5));
		assert_eq!(int_div_float(400_000_000_000_u64, 800_000_000_000_u64), Some(0.5));
		assert_eq!(int_div_float(400_000_000_000_u64, 0_u64), None);
		assert_eq!(int_div_float(4_u8, 8_u8), Some(0.5));
	}
}
