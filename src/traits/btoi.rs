/*!
# Dactyl — Bytes to Signed.
*/

#![expect(clippy::unreadable_literal, reason = "Macros made me do it.")]

use crate::{
	int,
	traits::BytesToUnsigned,
};
use std::num::{
	NonZeroI8,
	NonZeroI16,
	NonZeroI32,
	NonZeroI64,
	NonZeroI128,
	NonZeroIsize,
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



/// # Helper: Documentation.
macro_rules! docs {
	($ty:ident) => (concat!(
"# (ASCII) Bytes to Signed.

Parse a `", stringify!($ty), "` from an ASCII byte slice.

## Examples

```
use dactyl::traits::BytesToSigned;

assert_eq!(
    ", stringify!($ty), "::btoi(b\"", int!(@min $ty), "\"),
    Some(", stringify!($ty), "::MIN),
);
assert_eq!(
    ", stringify!($ty), "::btoi(b\"", int!(@max $ty), "\"),
    Some(", stringify!($ty), "::MAX),
);

// Leading zeroes are fine.
assert_eq!(
    ", stringify!($ty), "::btoi(b\"00000123\"),
    Some(123),
);

// These are all bad.
assert!(", stringify!($ty), "::btoi(&[]).is_none()); // Empty.
assert!(", stringify!($ty), "::btoi(b\" 2223231  \").is_none()); // Whitespace.
assert!(", stringify!($ty), "::btoi(b\"nope\").is_none()); // Not a number.
assert!(", stringify!($ty), "::btoi(
    b\"4402823669209384634633746074317682114550\").is_none()
); // Too big.
```
"
	));
}



/// # Helper: Generate Impls.
macro_rules! signed {
	($ty:ident $min:literal) => (
		impl BytesToSigned for $ty {
			#[expect(clippy::cast_possible_wrap, reason = "False positive.")]
			#[inline]
			#[doc = docs!($ty)]
			fn btoi(src: &[u8]) -> Option<Self> {
				let (neg, src) = strip_sign(src)?;
				let val = <int!(@flip $ty)>::btou(src)?;

				// Negative.
				if neg {
					if val == $min { Some(<$ty>::MIN) }
					else if val < $min { Some(0 - val as $ty) }
					else { None }
				}
				// Positive.
				else if val <= int!(@max $ty) { Some(val as $ty) }
				else { None }
			}
		}
	);
}

signed!(i8   128);
signed!(i16  32768);
signed!(i32  2147483648);
signed!(i64  9223372036854775808);
signed!(i128 170141183460469231731687303715884105728);

#[cfg(target_pointer_width = "16")]
signed!(isize 32768);

#[cfg(target_pointer_width = "32")]
signed!(isize 2147483648);

#[cfg(target_pointer_width = "64")]
signed!(isize 9223372036854775808);

/// # Helper: Non-Zero.
macro_rules! nz {
	($($ty:ident)+) => ($(
		impl BytesToSigned for $ty {
			#[inline]
			/// # (ASCII) Bytes to Signed.
			fn btoi(src: &[u8]) -> Option<Self> {
				<int!(@alias $ty)>::btoi(src).and_then(Self::new)
			}
		}
	)+);
}

nz! { NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize }



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

	macro_rules! test {
		(@eq $ty:ident, $raw:expr, $expected:expr $(,)?) => (
			assert_eq!(
				<$ty>::btoi($raw),
				$expected,
				concat!(stringify!($ty), "::btoi({:?})"),
				$raw,
			);
		);

		($($fn:ident $ty:ident),+ $(,)?) => ($(
			#[test]
			fn $fn() {
				use std::num::NonZero;

				// Common sanity checks.
				test!(@eq $ty, b"", None);
				test!(@eq $ty, b" 1", None);
				test!(@eq $ty, b"1.0", None);
				test!(@eq $ty, b"apples", None);

				// Plus is fine for signed types.
				test!(@eq $ty, b"+123", Some(123));

				// Zero is zero no matter how many.
				test!(@eq $ty, b"0", Some(0));
				test!(@eq $ty, b"00", Some(0));
				test!(@eq $ty, b"0000", Some(0));
				test!(@eq $ty, b"00000000", Some(0));
				test!(@eq $ty, b"0000000000000000", Some(0));
				test!(@eq $ty, b"000000000000000000000000000000000000000000000000", Some(0));

				// Maximum should work with or without a zero.
				test!(@eq $ty, concat!(int!(@max $ty)).as_bytes(), Some(<$ty>::MAX));
				test!(@eq $ty, concat!("0", int!(@max $ty)).as_bytes(), Some(<$ty>::MAX));

				// But not if bigger.
				test!(@eq $ty, concat!(int!(@max $ty), "0").as_bytes(), None);

				// i8 can go all the way.
				if size_of::<$ty>() == 1 {
					for i in <$ty>::MIN..<$ty>::MAX {
						let s = i.to_string();
						test!(@eq $ty, s.as_bytes(), Some(i));
						assert_eq!(
							<NonZero<$ty>>::btoi(s.as_bytes()),
							NonZero::<$ty>::new(i),
						);
					}
					return;
				}

				// Test a random sample.
				let mut rng = fastrand::Rng::new();
				for i in std::iter::repeat_with(|| rng.$ty(..)).take(SAMPLE_SIZE) {
					let s = i.to_string();
					test!(@eq $ty, s.as_bytes(), Some(i));
					assert_eq!(
						<NonZero<$ty>>::btoi(s.as_bytes()),
						NonZero::<$ty>::new(i),
					);
				}
			}
		)+);
	}

	test!(
		t_i8  i8,
		t_i16 i16,
		t_i32 i32,
		t_i64 i64,
		t_i128 i128,
		t_isize isize,
	);
}
