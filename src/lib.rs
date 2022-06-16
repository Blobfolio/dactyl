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

#![allow(clippy::module_name_repetitions)] // This is fine.



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
static DOUBLE: &[u8; 200] = b"\
	0001020304050607080910111213141516171819\
	2021222324252627282930313233343536373839\
	4041424344454647484950515253545556575859\
	6061626364656667686970717273747576777879\
	8081828384858687888990919293949596979899";

#[allow(unsafe_code)]
#[inline]
/// # Double Pointer.
///
/// This produces a pointer to a specific two-digit subslice of `DOUBLE`.
///
/// ## Safety
///
/// This method will panic if `num` is greater than 100, but as this is
/// private and all ranges are pre-checked, that should never actually happen.
pub(crate) fn double(num: usize) -> *const u8 {
	assert!(num < 100);
	unsafe { DOUBLE.as_ptr().add(num << 1) }
}



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
where T: Copy + std::ops::Div<Output=T> + std::ops::Rem<Output=T> {
	(e / d, e % d)
}

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

#[allow(unsafe_code)]
/// # Write u8.
///
/// This will quickly write a `u8` number as a UTF-8 byte slice to the provided
/// pointer.
///
/// ## Safety
///
/// The pointer must have enough space for the value, i.e. 1-3 digits, or
/// undefined things will happen.
pub unsafe fn write_u8(buf: *mut u8, num: u8) {
	if 99 < num {
		let (div, rem) = div_mod(num, 100);
		std::ptr::write(buf, div + b'0');
		std::ptr::copy_nonoverlapping(double(rem as usize), buf.add(1), 2);
	}
	else if 9 < num {
		std::ptr::copy_nonoverlapping(double(num as usize), buf, 2);
	}
	else {
		std::ptr::write(buf, num + b'0');
	}
}

#[allow(unsafe_code)]
/// # Write Time.
///
/// This writes HH:MM:SS to the provided pointer.
///
/// ## Panics
///
/// This method is only intended to cover values that fit in a day and will
/// panic if `h`, `m`, or `s` is outside the range of `0..60`.
///
/// ## Safety
///
/// The pointer must have 8 bytes free or undefined things will happen.
pub unsafe fn write_time(buf: *mut u8, h: u8, m: u8, s: u8) {
	assert!(h < 60 && m < 60 && s < 60);

	std::ptr::copy_nonoverlapping(double(h as usize), buf, 2);
	std::ptr::write(buf.add(2), b':');
	std::ptr::copy_nonoverlapping(double(m as usize), buf.add(3), 2);
	std::ptr::write(buf.add(5), b':');
	std::ptr::copy_nonoverlapping(double(s as usize), buf.add(6), 2);
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

	#[allow(unsafe_code)]
	#[test]
	fn t_write_u8() {
		for i in 0..10 {
			let mut buf = [0_u8];
			unsafe {
				write_u8(buf.as_mut_ptr(), i);
				assert_eq!(buf, format!("{}", i).as_bytes());
			}
		}

		for i in 10..100 {
			let mut buf = [0_u8, 0_u8];
			unsafe {
				write_u8(buf.as_mut_ptr(), i);
				assert_eq!(buf, format!("{}", i).as_bytes());
			}
		}

		for i in 100..u8::MAX {
			let mut buf = [0_u8, 0_u8, 0_u8];
			unsafe {
				write_u8(buf.as_mut_ptr(), i);
				assert_eq!(buf, format!("{}", i).as_bytes());
			}
		}
	}

	#[allow(unsafe_code)]
	#[test]
	fn t_write_time() {
		let mut buf = [0_u8; 8];
		unsafe {
			write_time(buf.as_mut_ptr(), 1, 2, 3);
			assert_eq!(buf, *b"01:02:03");

			write_time(buf.as_mut_ptr(), 10, 26, 37);
			assert_eq!(buf, *b"10:26:37");
		}
	}
}
