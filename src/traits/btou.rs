/*!
# Dactyl — Bytes to Unsigned.
*/

use crate::int;
use std::num::{
	NonZeroU8,
	NonZeroU16,
	NonZeroU32,
	NonZeroU64,
	NonZeroU128,
	NonZeroUsize,
};



/// # (ASCII) Bytes to Unsigned.
///
/// This trait exposes the method `btou` for converting byte slices to proper
/// unsigned primitives, similar to what [`str::parse`] does for strings.
///
/// Leading zeroes — e.g. "001" — are fine, but the method will return `None`
/// if the slice is empty, contains non-numeric characters — including signs,
/// punctuation, etc. —  or is out of range for the type.
///
/// For signed integer parsing, see [`BytesToSigned`](crate::traits::BytesToSigned);
///
/// ## Examples
///
/// ```
/// use dactyl::traits::BytesToUnsigned;
///
/// assert_eq!(
///     u8::btou(b"255"),
///     Some(u8::MAX),
/// );
/// ```
pub trait BytesToUnsigned: Sized {
	/// # (ASCII) Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self>;
}



/// # Helper: Documentation.
macro_rules! docs {
	($ty:ident) => (concat!(
"# (ASCII) Bytes to Unsigned.

Parse a `", stringify!($ty), "` from an ASCII byte slice.

## Examples

```
use dactyl::traits::BytesToUnsigned;

assert_eq!(
    ", stringify!($ty), "::btou(b\"", int!(@min $ty), "\"),
    Some(", stringify!($ty), "::MIN),
);
assert_eq!(
    ", stringify!($ty), "::btou(b\"", int!(@max $ty), "\"),
    Some(", stringify!($ty), "::MAX),
);

// Leading zeroes are fine.
assert_eq!(
    ", stringify!($ty), "::btou(b\"00000123\"),
    Some(123),
);

// These are all bad.
assert!(", stringify!($ty), "::btou(&[]).is_none()); // Empty.
assert!(", stringify!($ty), "::btou(b\"+13\").is_none()); // Plus.
assert!(", stringify!($ty), "::btou(b\" 2223231  \").is_none()); // Whitespace.
assert!(", stringify!($ty), "::btou(b\"nope\").is_none()); // Not a number.
assert!(", stringify!($ty), "::btou(
    b\"4402823669209384634633746074317682114550\").is_none()
); // Too big.
```
"
	));
}



impl BytesToUnsigned for u8 {
	#[inline]
	#[doc = docs!(u8)]
	fn btou(mut src: &[u8]) -> Option<Self> {
		loop {
			return match src {
				[] => None,
				[                        c @ b'0'..=b'9' ] => Some(                          *c ^ b'0'),
				[       b @ b'1'..=b'9', c @ b'0'..=b'9' ] => Some(      (*b ^ b'0') * 10 + (*c ^ b'0')),
				[ b'1', b @ b'0'..=b'9', c @ b'0'..=b'9' ] => Some(100 + (*b ^ b'0') * 10 + (*c ^ b'0')),
				[ b'2', b @ b'0'..=b'5', c @ b'0'..=b'9' ] => 200_u8.checked_add((*b ^ b'0') * 10 + (*c ^ b'0')),
				_ => {
					// Strip leading zeroes and try again?
					src = strip0(src)?;
					if src.is_empty() { Some(0) }
					else { continue }
				}
			};
		}
	}
}



impl BytesToUnsigned for u16 {
	#[expect(clippy::many_single_char_names, reason = "For readability.")]
	#[inline]
	#[doc = docs!(u16)]
	fn btou(mut src: &[u8]) -> Option<Self> {
		loop {
			return match src {
				[] => None,
				[                                                                     e @ b'0'..=b'9' ] => Some(Self::from(*e ^ b'0')),
				[                                                    d @ b'1'..=b'9', e @ b'0'..=b'9' ] => Some(Self::from(*d ^ b'0') * 10 + Self::from(*e ^ b'0')),
				[                                   c @ b'1'..=b'9', d @ b'0'..=b'9', e @ b'0'..=b'9' ] => Some(Self::from(*c ^ b'0') * 100 + Self::from(*d ^ b'0') * 10 + Self::from(*e ^ b'0')),
				[                  b @ b'1'..=b'9', c @ b'0'..=b'9', d @ b'0'..=b'9', e @ b'0'..=b'9' ] => Some(Self::from(*b ^ b'0') * 1000 + Self::from(*c ^ b'0') * 100 + Self::from(*d ^ b'0') * 10 + Self::from(*e ^ b'0')),
				[ a @ b'1'..=b'6', b @ b'0'..=b'9', c @ b'0'..=b'9', d @ b'0'..=b'9', e @ b'0'..=b'9' ] => (Self::from(a ^ b'0') * 10_000).checked_add(
						Self::from(*b ^ b'0') * 1000 + Self::from(*c ^ b'0') * 100 + Self::from(*d ^ b'0') * 10 + Self::from(*e ^ b'0')
				),
				_ => {
					// Strip leading zeroes and try again?
					src = strip0(src)?;
					if src.is_empty() { Some(0) }
					else { continue }
				}
			};
		}
	}
}



impl BytesToUnsigned for u32 {
	#[inline]
	#[doc = docs!(u32)]
	fn btou(mut src: &[u8]) -> Option<Self> {
		// The number of digits that can be safely multiplied by 10 without
		// risking overflow.
		const SAFE: usize = u32::MAX.ilog10() as usize;

		if src.is_empty() { return None; }

		// Split src into safe and unsafe halves.
		let overflowable: &[u8];
		if SAFE < src.len() { (src, overflowable) = src.split_at(SAFE); }
		else { overflowable = &[]; }

		// The "safe" chunk can't overflow.
		let mut out: Self = 0;
		for v in src {
			let v = Self::from(v ^ b'0');
			if 9 < v { return None; }
			out = out * 10 + v;
		}

		// The rest requires more explicit checking.
		for v in overflowable {
			let v = Self::from(v ^ b'0');
			if 9 < v { return None; }
			out = out.checked_mul(10).and_then(|n| n.checked_add(v))?;
		}

		Some(out)
	}
}



impl BytesToUnsigned for u64 {
	#[inline]
	#[doc = docs!(u64)]
	fn btou(mut src: &[u8]) -> Option<Self> {
		// The number of digits that can be safely multiplied by 10 without
		// risking overflow.
		const SAFE: usize = u64::MAX.ilog10() as usize;

		if src.is_empty() { return None; }

		// Split src into safe and unsafe halves.
		let overflowable: &[u8];
		if SAFE < src.len() { (src, overflowable) = src.split_at(SAFE); }
		else { overflowable = &[]; }

		// The "safe" chunk can't overflow.
		let mut out: Self = 0;
		for v in src {
			let v = Self::from(v ^ b'0');
			if 9 < v { return None; }
			out = out * 10 + v;
		}

		// The rest requires more explicit checking.
		for v in overflowable {
			let v = Self::from(v ^ b'0');
			if 9 < v { return None; }
			out = out.checked_mul(10).and_then(|n| n.checked_add(v))?;
		}

		Some(out)
	}
}



impl BytesToUnsigned for u128 {
	#[inline]
	#[doc = docs!(u128)]
	fn btou(src: &[u8]) -> Option<Self> {
		if src.is_empty() { return None; }

		// The compiler doesn't seem to apply the same optimizations to u128
		// as the smaller types. Working in chunks of eight (while we can)
		// helps a lot.
		let mut out: Self = 0;
		let (chunks, rest) = src.as_chunks::<8>();
		for chunk in chunks {
			let chunk = Self::from(parse8(*chunk)?);
			out = out.checked_mul(100_000_000)?.checked_add(chunk)?;
		}

		// Parse the rest.
		rest.iter().copied().try_fold(out, |acc, v| {
			let v = Self::from(v ^ b'0');
			if v < 10 {
				acc.checked_mul(10).and_then(|n| n.checked_add(v))
			}
			else { None }
		})
	}
}



impl BytesToUnsigned for usize {
	#[allow(
		clippy::allow_attributes,
		clippy::cast_possible_truncation,
		reason = "False positive.",
	)]
	#[inline]
	#[doc = docs!(usize)]
	fn btou(src: &[u8]) -> Option<Self> {
		<int!(@alias usize)>::btou(src).map(|n| n as Self)
	}
}



/// # Helper: Non-Zero.
macro_rules! nz {
	($($ty:ident)+) => ($(
		impl BytesToUnsigned for $ty {
			#[inline]
			/// # (ASCII) Bytes to Unsigned.
			fn btou(src: &[u8]) -> Option<Self> {
				<int!(@alias $ty)>::btou(src).and_then(Self::new)
			}
		}
	)+);
}

nz! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize }



#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
/// # Parse Eight.
///
/// This parses eight digits as a single `u64`, reducing the number of
/// operations that would otherwise be required.
const fn parse8(src: [u8; 8]) -> Option<u32> {
	let chunk = u64::from_le_bytes(src) ^ 0x3030_3030_3030_3030_u64;

	// Make sure the slice contains only ASCII digits.
	let chk = chunk.wrapping_add(0x7676_7676_7676_7676_u64);
	if (chunk & 0xf0f0_f0f0_f0f0_f0f0_u64) | (chk & 0x8080_8080_8080_8080_u64) == 0 {
		// 1-byte mask trick (works on 4 pairs of single digits)
		let lower_digits = (chunk & 0x0f00_0f00_0f00_0f00) >> 8;
		let upper_digits = (chunk & 0x000f_000f_000f_000f) * 10;
		let chunk = lower_digits + upper_digits;

		// 2-byte mask trick (works on 2 pairs of two digits)
		let lower_digits = (chunk & 0x00ff_0000_00ff_0000) >> 16;
		let upper_digits = (chunk & 0x0000_00ff_0000_00ff) * 100;
		let chunk = lower_digits + upper_digits;

		// 4-byte mask trick (works on a pair of four digits)
		let lower_digits = ((chunk & 0x0000_ffff_0000_0000) >> 32) as u32;
		let upper_digits = (chunk as u32) * 10000;

		Some(lower_digits + upper_digits)
	}
	else { None }
}

#[cold]
/// # Strip Leading Zeroes.
const fn strip0(mut src: &[u8]) -> Option<&[u8]> {
	let before = src.len();
	while let [ b'0', rest @ .. ] = src { src = rest; }

	if src.len() == before { None }
	else { Some(src) }
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
				<$ty>::btou($raw),
				$expected,
				concat!(stringify!($ty), "::btou({:?})"),
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
				test!(@eq $ty, b"+123", None);

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
							<NonZero<$ty>>::btou(s.as_bytes()),
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
						<NonZero<$ty>>::btou(s.as_bytes()),
						NonZero::<$ty>::new(i),
					);
				}
			}
		)+);
	}

	test!(
		t_u8  u8,
		t_u16 u16,
		t_u32 u32,
		t_u64 u64,
		t_u128 u128,
		t_usize usize,
	);
}
