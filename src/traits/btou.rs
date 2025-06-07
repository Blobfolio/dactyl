/*!
# Dactyl — Bytes to Unsigned.
*/

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



impl BytesToUnsigned for u8 {
	#[inline]
	/// # (ASCII) Bytes to Unsigned.
	///
	/// Parse a `u8` from an ASCII byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::BytesToUnsigned;
	///
	/// assert_eq!(
	///     u8::btou(b"0"),
	///     Some(u8::MIN),
	/// );
	/// assert_eq!(
	///     u8::btou(b"255"),
	///     Some(u8::MAX),
	/// );
	///
	/// // Leading zeroes are fine.
	/// assert_eq!(
	///     u8::btou(b"00000123"),
	///     Some(123_u8),
	/// );
	///
	/// // These are all bad.
	/// assert!(u8::btou(&[]).is_none());     // Empty.
	/// assert!(u8::btou(b"+255").is_none()); // "+"
	/// assert!(u8::btou(b" 222").is_none()); // Whitespace.
	/// assert!(u8::btou(b"1234").is_none()); // Too big.
	/// assert!(u8::btou(b"duh!").is_none()); // Not a number.
	/// ```
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
	/// # (ASCII) Bytes to Unsigned.
	///
	/// Parse a `u16` from an ASCII byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::BytesToUnsigned;
	///
	/// assert_eq!(
	///     u16::btou(b"0"),
	///     Some(u16::MIN),
	/// );
	/// assert_eq!(
	///     u16::btou(b"65535"),
	///     Some(u16::MAX),
	/// );
	///
	/// // Leading zeroes are fine.
	/// assert_eq!(
	///     u16::btou(b"00000123"),
	///     Some(123_u16),
	/// );
	///
	/// // These are all bad.
	/// assert!(u16::btou(&[]).is_none());       // Empty.
	/// assert!(u16::btou(b"+25500").is_none()); // "+"
	/// assert!(u16::btou(b" 222  ").is_none()); // Whitespace.
	/// assert!(u16::btou(b"123456").is_none()); // Too big.
	/// assert!(u16::btou(b"no way").is_none()); // Not a number.
	/// ```
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
	/// # (ASCII) Bytes to Unsigned.
	///
	/// Parse a `u32` from an ASCII byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::BytesToUnsigned;
	///
	/// assert_eq!(
	///     u32::btou(b"0"),
	///     Some(u32::MIN),
	/// );
	/// assert_eq!(
	///     u32::btou(b"4294967295"),
	///     Some(u32::MAX),
	/// );
	///
	/// // Leading zeroes are fine.
	/// assert_eq!(
	///     u32::btou(b"00000123"),
	///     Some(123_u32),
	/// );
	///
	/// // These are all bad.
	/// assert!(u32::btou(&[]).is_none());           // Empty.
	/// assert!(u32::btou(b"+255003320").is_none()); // "+"
	/// assert!(u32::btou(b" 2223231  ").is_none()); // Whitespace.
	/// assert!(u32::btou(b"4294967296").is_none()); // Too big.
	/// assert!(u32::btou(b"yeah, nope").is_none()); // Not a number.
	/// ```
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
	/// # (ASCII) Bytes to Unsigned.
	///
	/// Parse a `u64` from an ASCII byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::BytesToUnsigned;
	///
	/// assert_eq!(
	///     u64::btou(b"0"),
	///     Some(u64::MIN),
	/// );
	/// assert_eq!(
	///     u64::btou(b"18446744073709551615"),
	///     Some(u64::MAX),
	/// );
	///
	/// // Leading zeroes are fine.
	/// assert_eq!(
	///     u64::btou(b"00000123"),
	///     Some(123_u64),
	/// );
	///
	/// // These are all bad.
	/// assert!(u64::btou(&[]).is_none());           // Empty.
	/// assert!(u64::btou(b"+255003320").is_none()); // "+"
	/// assert!(u64::btou(b" 2223231  ").is_none()); // Whitespace.
	/// assert!(u64::btou(b"yeah, nope").is_none()); // Not a number.
	/// assert!(u64::btou(
	///     b"28446744073709551615").is_none()
	/// ); // Too big.
	/// ```
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
	/// # (ASCII) Bytes to Unsigned.
	///
	/// Parse a `u128` from an ASCII byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::BytesToUnsigned;
	///
	/// assert_eq!(
	///     u128::btou(b"0"),
	///     Some(u128::MIN),
	/// );
	/// assert_eq!(
	///     u128::btou(b"340282366920938463463374607431768211455"),
	///     Some(u128::MAX),
	/// );
	///
	/// // Leading zeroes are fine.
	/// assert_eq!(
	///     u128::btou(b"00000123"),
	///     Some(123_u128),
	/// );
	///
	/// // These are all bad.
	/// assert!(u128::btou(&[]).is_none());           // Empty.
	/// assert!(u128::btou(b"+255003320").is_none()); // "+"
	/// assert!(u128::btou(b" 2223231  ").is_none()); // Whitespace.
	/// assert!(u128::btou(b"yeah, nope").is_none()); // Not a number.
	/// assert!(u128::btou(
	///     b"4402823669209384634633746074317682114550").is_none()
	/// ); // Too big.
	/// ```
	fn btou(mut src: &[u8]) -> Option<Self> {
		if src.is_empty() { return None; }

		// The compiler doesn't seem to apply the same optimizations to u128
		// as the smaller types. Working in chunks of eight (while we can)
		// helps a lot.
		// TODO: use array_chunks when stable.
		let mut out: Self = 0;
		while let Some((chunk, rest)) = src.split_first_chunk::<8>() {
			let chunk = Self::from(parse8(*chunk)?);
			out = out.checked_mul(100_000_000)?.checked_add(chunk)?;
			src = rest;
		}

		// Parse the rest.
		src.iter().copied().try_fold(out, |acc, v| {
			let v = Self::from(v ^ b'0');
			if v < 10 {
				acc.checked_mul(10).and_then(|n| n.checked_add(v))
			}
			else { None }
		})
	}
}



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



#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
impl BytesToUnsigned for usize {
	#[cfg(target_pointer_width = "16")]
	#[inline]
	/// # (ASCII) Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { u16::btou(src).map(Self::from) }

	#[cfg(target_pointer_width = "32")]
	#[inline]
	/// # (ASCII) Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { Some(u32::btou(src)? as Self) }

	#[cfg(target_pointer_width = "64")]
	#[inline]
	/// # (ASCII) Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { Some(u64::btou(src)? as Self) }
}



/// # Helper: Non-Zero.
macro_rules! nonzero {
	($($outer:ty, $inner:ty),+ $(,)?) => ($(
		impl BytesToUnsigned for $outer {
			#[inline]
			/// # (ASCII) Bytes to Unsigned.
			fn btou(src: &[u8]) -> Option<Self> {
				<$inner>::btou(src).and_then(Self::new)
			}
		}
	)+);
}

nonzero!(
	NonZeroU8, u8,
	NonZeroU16, u16,
	NonZeroU32, u32,
	NonZeroU64, u64,
	NonZeroU128, u128,
	NonZeroUsize, usize,
);



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

	macro_rules! sanity_check {
		($ty:ty) => (
			assert_eq!(<$ty>::btou(b""), None);
			assert_eq!(<$ty>::btou(b" 1"), None);
			assert_eq!(<$ty>::btou(b"1.0"), None);
			assert_eq!(<$ty>::btou(b"+123"), None);
			assert_eq!(<$ty>::btou(b"apples"), None);

			assert_eq!(<$ty>::btou(b"0"), Some(0));
			assert_eq!(<$ty>::btou(b"00"), Some(0));
			assert_eq!(<$ty>::btou(b"0000"), Some(0));
			assert_eq!(<$ty>::btou(b"00000000"), Some(0));
			assert_eq!(<$ty>::btou(b"0000000000000000"), Some(0));
			assert_eq!(<$ty>::btou(b"000000000000000000000000000000000000000000000000"), Some(0));
		);
	}

	#[test]
	fn t_u8() {
		sanity_check!(u8);
		assert_eq!(u8::btou(b"0255"), Some(u8::MAX));
		assert_eq!(u8::btou(b"256"), None);

		// This is small enough we can check every value.
		for i in 0..=u8::MAX {
			let s = i.to_string();
			assert_eq!(u8::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroU8::btou(s.as_bytes()), NonZeroU8::new(i));
		}
	}

	#[test]
	fn t_u16() {
		sanity_check!(u16);
		assert_eq!(u16::btou(b"065535"), Some(u16::MAX));
		assert_eq!(u16::btou(b"65536"), None);

		// This is small enough we can check every value.
		#[cfg(not(miri))]
		for i in 0..=u16::MAX {
			let s = i.to_string();
			assert_eq!(u16::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroU16::btou(s.as_bytes()), NonZeroU16::new(i));
		}

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u16(..)).take(SAMPLE_SIZE) {
				let s = i.to_string();
				assert_eq!(u16::btou(s.as_bytes()), Some(i));
				assert_eq!(NonZeroU16::btou(s.as_bytes()), NonZeroU16::new(i));
			}
		}
	}

	#[test]
	fn t_u32() {
		sanity_check!(u32);
		assert_eq!(u32::btou(b"4294967295"), Some(u32::MAX));
		assert_eq!(u32::btou(b"04294967295"), Some(u32::MAX));
		assert_eq!(u32::btou(b"4294967296"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(u32::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroU32::btou(s.as_bytes()), NonZeroU32::new(i));
		}
	}

	#[test]
	fn t_u64() {
		sanity_check!(u64);
		assert_eq!(u64::btou(b"18446744073709551615"), Some(u64::MAX));
		assert_eq!(u64::btou(b"018446744073709551615"), Some(u64::MAX));
		assert_eq!(u64::btou(b"18446744073709551616"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(u64::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroU64::btou(s.as_bytes()), NonZeroU64::new(i));
		}
	}

	#[test]
	fn t_u128() {
		sanity_check!(u128);
		assert_eq!(u128::btou(b"340282366920938463463374607431768211455"), Some(u128::MAX));
		assert_eq!(u128::btou(b"0340282366920938463463374607431768211455"), Some(u128::MAX));
		assert_eq!(u128::btou(b"340282366920938463463374607431768211456"), None);

		// Now let's check ten million random values and hope they all hit.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u128(..)).take(SAMPLE_SIZE) {
			let s = i.to_string();
			assert_eq!(u128::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroU128::btou(s.as_bytes()), NonZeroU128::new(i));
		}
	}

	#[test]
	fn t_usize() {
		sanity_check!(usize);
		assert_eq!(usize::btou(usize::MAX.to_string().as_bytes()), Some(usize::MAX));

		// Usize just wraps the appropriate sized type, but let's check some
		// random values anyway.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.usize(..)).take(SAMPLE_SIZE.min(50_000)) {
			let s = i.to_string();
			assert_eq!(usize::btou(s.as_bytes()), Some(i));
			assert_eq!(NonZeroUsize::btou(s.as_bytes()), NonZeroUsize::new(i));
		}
	}
}
