/*!
# Dactyl — Bytes to Signed.
*/

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



/// # Bytes to Signed.
///
/// This is essentially the signed equivalent of [`BytesToUnsigned`](crate::traits::BytesToUnsigned).
/// It works exactly the same way and for the same reason, except the first
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
	/// # Bytes to Signed.
	fn btoi(src: &[u8]) -> Option<Self>;
}



macro_rules! signed {
	($ty:ty, $unsigned:ty, $min:literal, $max:literal) => (
		impl BytesToSigned for $ty {
			/// # Bytes to Signed.
			fn btoi(src: &[u8]) -> Option<Self> {
				match src.len() {
					1 => Some(<$unsigned>::btou(src)? as Self),
					0 => None,
					_ => {
						let (neg, val) = match src[0] {
							b'-' => (true, <$unsigned>::btou(&src[1..])?),
							b'+' => (false, <$unsigned>::btou(&src[1..])?),
							_ => (false, <$unsigned>::btou(src)?),
						};

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
			}
		}
	);
}

signed!(i8, u8, 128, 127);
signed!(i16, u16, 32768, 32767);
signed!(i32, u32, 2_147_483_648, 2_147_483_647);
signed!(i64, u64, 9_223_372_036_854_775_808, 9_223_372_036_854_775_807);
signed!(i128, u128, 170_141_183_460_469_231_731_687_303_715_884_105_728, 170_141_183_460_469_231_731_687_303_715_884_105_727);

#[cfg(target_pointer_width = "16")]
signed!(isize, u16, 32768, 32767);

#[cfg(target_pointer_width = "32")]
signed!(isize, u32, 2_147_483_648, 2_147_483_647);

#[cfg(target_pointer_width = "64")]
signed!(isize, u64, 9_223_372_036_854_775_808, 9_223_372_036_854_775_807);

#[cfg(target_pointer_width = "128")]
signed!(isize, u128, 170_141_183_460_469_231_731_687_303_715_884_105_728, 170_141_183_460_469_231_731_687_303_715_884_105_727);

/// # Helper: Non-Zero.
macro_rules! nonzero {
	($($outer:ty, $inner:ty),+ $(,)?) => ($(
		impl BytesToSigned for $outer {
			/// # Bytes to Unsigned.
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



#[cfg(test)]
mod tests {
	use super::*;

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
		for i in i16::MIN..=i16::MAX {
			let s = i.to_string();
			assert_eq!(i16::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroI16::btoi(s.as_bytes()), NonZeroI16::new(i));
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
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i32(..)).take(10_000_000) {
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
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i64(..)).take(10_000_000) {
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
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i128(..)).take(10_000_000) {
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
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.isize(..)).take(50_000) {
			let s = i.to_string();
			assert_eq!(isize::btoi(s.as_bytes()), Some(i));
			assert_eq!(NonZeroIsize::btoi(s.as_bytes()), NonZeroIsize::new(i));
		}
	}
}
