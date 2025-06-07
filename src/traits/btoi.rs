/*!
# Dactyl — Bytes to Signed.
*/

#![expect(clippy::cast_possible_truncation, reason = "False positive.")]
#![expect(clippy::unreadable_literal, reason = "Macros made me do it.")]

use crate::traits::BytesToUnsigned;
use std::{
	cmp::Ordering,
	num::{
		NonZeroI8,
		NonZeroI16,
		NonZeroI32,
		NonZeroI64,
		NonZeroI128,
		NonZeroIsize,
	},
};



/// # (ASCII) Bytes to Signed.
///
/// This is the signed equivalent of [`BytesToUnsigned`](crate::traits::BytesToUnsigned).
/// It works exactly the same way and for the same reasons, except the first
/// byte can optionally be a `+` or `-`.
///
/// ## Examples
///
/// ```
/// use dactyl::traits::BytesToSigned;
///
/// assert_eq!(
///     i8::btoi(b"-120"),
///     Some(-120_i8),
/// );
/// ```
pub trait BytesToSigned: Sized {
	/// # (ASCII) Bytes to Signed.
	fn btoi(src: &[u8]) -> Option<Self>;
}



/// # Helper: Generate Impls.
macro_rules! signed {
	($ty:ty, $unsigned:ty, $min:literal, $max:literal) => (
		impl BytesToSigned for $ty {
			#[expect(clippy::cast_possible_wrap, reason = "False positive.")]
			#[inline]
			/// # (ASCII) Bytes to Signed.
			///
			#[doc = concat!("Parse a `", stringify!($ty), "` from an ASCII byte slice.")]
			///
			/// ## Examples
			///
			/// ```
			/// use dactyl::traits::BytesToSigned;
			///
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::btoi(b\"-", stringify!($min), "\"),")]
			#[doc = concat!("    Some(", stringify!($ty), "::MIN),")]
			/// );
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::btoi(b\"0\"),")]
			///     Some(0),
			/// );
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::btoi(b\"", stringify!($max), "\"),")]
			#[doc = concat!("    Some(", stringify!($ty), "::MAX),")]
			/// );
			///
			/// // Leading zeroes are fine.
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::btoi(b\"00000123\"),")]
			///     Some(123),
			/// );
			///
			/// // These are all bad.
			#[doc = concat!("assert!(", stringify!($ty), "::btoi(&[]).is_none());     // Empty.")]
			#[doc = concat!("assert!(", stringify!($ty), "::btoi(b\" 22 \").is_none()); // Whitespace.")]
			#[doc = concat!("assert!(", stringify!($ty), "::btoi(b\"duh!\").is_none()); // Not a number.")]
			/// ```
			fn btoi(src: &[u8]) -> Option<Self> {
				// Find/strip the sign, then crunch as if it were unsigned.
				let (neg, src) = strip_sign(src)?;
				let val = <$unsigned>::btou(src)?;

				// Deal with negation…
				if neg {
					match val.cmp(&$min) {
						Ordering::Equal => Some(Self::MIN),
						Ordering::Less => Some(0 - val as Self),
						Ordering::Greater => None,
					}
				}
				// Positive values.
				else if val <= $max { Some(val as Self) }
				// Bunk.
				else { None }
			}
		}
	);
}

signed!(i8, u8, 128, 127);
signed!(i16, u16, 32768, 32767);
signed!(i32, u32, 2147483648, 2147483647);
signed!(i64, u64, 9223372036854775808, 9223372036854775807);
signed!(i128, u128, 170141183460469231731687303715884105728, 170141183460469231731687303715884105727);

#[cfg(target_pointer_width = "16")]
signed!(isize, u16, 32768, 32767);

#[cfg(target_pointer_width = "32")]
signed!(isize, u32, 2147483648, 2147483647);

#[cfg(target_pointer_width = "64")]
signed!(isize, u64, 9223372036854775808, 9223372036854775807);

/// # Helper: Non-Zero.
macro_rules! nonzero {
	($($outer:ty, $inner:ty),+ $(,)?) => ($(
		impl BytesToSigned for $outer {
			#[inline]
			/// # (ASCII) Bytes to Signed.
			fn btoi(src: &[u8]) -> Option<Self> {
				<$inner>::btoi(src).and_then(Self::new)
			}
		}
	)+);
}

nonzero!(
	NonZeroI8, i8,
	NonZeroI16, i16,
	NonZeroI32, i32,
	NonZeroI64, i64,
	NonZeroI128, i128,
	NonZeroIsize, isize,
);



/// # Strip Sign.
///
/// If the slice starts with a plus or minus, strip it off, returning the
/// remainder and a bool indicating negativity.
///
/// If empty, `None` is returned. Everything else is passed through as-is and
/// assumed to be positive.
const fn strip_sign(src: &[u8]) -> Option<(bool, &[u8])> {
	match src {
		[] => None,
		[b'-', rest @ ..] => Some((true, rest)),
		[b'+', rest @ ..] => Some((false, rest)),
		_ => Some((false, src)),
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 500; // Miri runs way too slow for a million tests.

	macro_rules! sanity_check {
		($ty:ty) => (
			assert_eq!(<$ty>::btoi(b""), None);
			assert_eq!(<$ty>::btoi(b" 1"), None);
			assert_eq!(<$ty>::btoi(b"1.0"), None);
			assert_eq!(<$ty>::btoi(b"+123"), Some(123));
			assert_eq!(<$ty>::btoi(b"+0123"), Some(123));
			assert_eq!(<$ty>::btoi(b"-123"), Some(-123));
			assert_eq!(<$ty>::btoi(b"-0123"), Some(-123));
			assert_eq!(<$ty>::btoi(b"apples"), None);

			assert_eq!(<$ty>::btoi(b"-0"), Some(0));
			assert_eq!(<$ty>::btoi(b"+0"), Some(0));
			assert_eq!(<$ty>::btoi(b"0"), Some(0));
			assert_eq!(<$ty>::btoi(b"00"), Some(0));
			assert_eq!(<$ty>::btoi(b"0000"), Some(0));
			assert_eq!(<$ty>::btoi(b"00000000"), Some(0));
			assert_eq!(<$ty>::btoi(b"0000000000000000"), Some(0));
			assert_eq!(<$ty>::btoi(b"000000000000000000000000000000000000000000000000"), Some(0));
		);
	}

	#[test]
	fn t_i8() {
		sanity_check!(i8);
		assert_eq!(i8::btoi(b"0127"), Some(i8::MAX));
		assert_eq!(i8::btoi(b"128"), None);

		// This is small enough we can check every value.
		for i in i8::MIN..=i8::MAX {
			let s = i.to_string();
			assert_eq!(i8::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI8::btoi(s.as_bytes()), NonZeroI8::new(i));
		}
	}

	#[test]
	fn t_i16() {
		sanity_check!(i16);
		assert_eq!(i16::btoi(b"032767"), Some(i16::MAX));
		assert_eq!(i16::btoi(b"32768"), None);

		// This is small enough we can check every value.
		#[cfg(not(miri))]
		for i in i16::MIN..=i16::MAX {
			let s = i.to_string();
			assert_eq!(i16::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI16::btoi(s.as_bytes()), NonZeroI16::new(i));
		}

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.i16(..)).take(SAMPLE_SIZE) {
				let s = i.to_string();
				assert_eq!(i16::btoi(s.as_bytes()), Some(i));
				assert_eq!(NonZeroI16::btoi(s.as_bytes()), NonZeroI16::new(i));
			}
		}
	}

	#[test]
	fn t_i32() {
		sanity_check!(i32);
		assert_eq!(i32::btoi(b"-2147483648"), Some(i32::MIN));
		assert_eq!(i32::btoi(b"2147483647"), Some(i32::MAX));
		assert_eq!(i32::btoi(b"02147483647"), Some(i32::MAX));
		assert_eq!(i32::btoi(b"2147483648"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i32(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(i32::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI32::btoi(s.as_bytes()), NonZeroI32::new(i));
		}
	}

	#[test]
	fn t_i64() {
		sanity_check!(i64);
		assert_eq!(i64::btoi(b"-9223372036854775808"), Some(i64::MIN));
		assert_eq!(i64::btoi(b"9223372036854775807"), Some(i64::MAX));
		assert_eq!(i64::btoi(b"09223372036854775807"), Some(i64::MAX));
		assert_eq!(i64::btoi(b"9223372036854775808"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i64(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(i64::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI64::btoi(s.as_bytes()), NonZeroI64::new(i));
		}
	}

	#[test]
	fn t_i128() {
		sanity_check!(i128);
		assert_eq!(i128::btoi(b"-170141183460469231731687303715884105728"), Some(i128::MIN));
		assert_eq!(i128::btoi(b"170141183460469231731687303715884105727"), Some(i128::MAX));
		assert_eq!(i128::btoi(b"0170141183460469231731687303715884105727"), Some(i128::MAX));
		assert_eq!(i128::btoi(b"170141183460469231731687303715884105728"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i128(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(i128::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI128::btoi(s.as_bytes()), NonZeroI128::new(i));
		}
	}

	#[test]
	fn t_isize() {
		sanity_check!(isize);
		assert_eq!(isize::btoi(isize::MIN.to_string().as_bytes()), Some(isize::MIN));
		assert_eq!(isize::btoi(isize::MAX.to_string().as_bytes()), Some(isize::MAX));

		// Usize just wraps the appropriate sized type, but let's check some
		// random values anyway.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.isize(..)).take(SAMPLE_SIZE.min(50_000)) {
			let s = i.to_string();
			assert_eq!(isize::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroIsize::btoi(s.as_bytes()), NonZeroIsize::new(i));
		}
	}
}
