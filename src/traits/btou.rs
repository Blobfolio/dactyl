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



/// # Bytes to Unsigned.
///
/// This trait exposes the method `btou` which converts (UTF-8) byte slices to
/// proper, unsigned integers. It works just like [`str::parse`] and
/// `u*::from_str_radix`, but faster, particularly for `u64` and `u128`.
///
/// Leading zeroes are fine, but the method will return `None` if the slice is
/// empty, contains non-numeric characters (including `+` and `-`), or is too
/// large for the type.
///
/// Only little endian architectures are optimized; for big endian machines,
/// this trait just passes through the results of [`str::parse`].
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
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self>;
}



/// # Helper: Generate Fallbacks.
macro_rules! big {
	() => (
		#[cfg(target_endian = "big")]
		#[must_use]
		/// # Bytes to Unsigned.
		fn btou(src: &[u8]) -> Option<Self> {
			if src.is_empty() || src[0] == b'+' { None }
			else {
				std::str::from_utf8(src).ok().and_then(|s| s.parse::<Self>().ok())
			}
		}
	);
}



impl BytesToUnsigned for u8 {
	#[cfg(target_endian = "little")]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1 => parse1(src[0]),
			2 => Some(parse1(src[0])? * 10 + parse1(src[1])?),
			3 => match src[0] {
				b'0' => Some(parse1(src[1])? * 10 + parse1(src[2])?),
				b'1' => Some(100_u8 + parse1(src[1])? * 10 + parse1(src[2])?),
				// This requires overflow checking, but a fairly simple variety.
				b'2' => {
					let end = parse1(src[1])? * 10 + parse1(src[2])?;
					if end < 56 { Some(200 + end) }
					else { None }
				},
				_ => None,
			},
			0 => None,
			// We have to check anything larger for overflow.
			_ => src.iter()
				.try_fold(0_u8, |a, &b| a.checked_mul(10)?.checked_add(parse1(b)?)),
		}
	}

	big!();
}



impl BytesToUnsigned for u16 {
	#[cfg(target_endian = "little")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1 => parse1(src[0]).map(Self::from),
			2 => parse2(src),
			3 => Some(parse1(src[0]).map(Self::from)? * 100 + parse2(&src[1..])?),
			4 => parse4(src),
			0 => None,
			// We have to check anything larger for overflow.
			_ => {
				// Take a shortcut.
				let (a, b) = src.split_at(4);
				let start = parse4(a)?;
				b.iter()
					.try_fold(start, |a, &b| a.checked_mul(10)?.checked_add(parse1(b).map(Self::from)?))
			}
		}
	}

	big!();
}



impl BytesToUnsigned for u32 {
	#[cfg(target_endian = "little")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1 => parse1(src[0]).map(Self::from),
			2 => parse2(src).map(Self::from),
			3 => Some(parse2(&src[..2]).map(Self::from)? * 10 + parse1(src[2]).map(Self::from)?),
			4 => parse4(src).map(Self::from),
			5 => Some(parse4(&src[..4]).map(Self::from)? * 10 + parse1(src[4]).map(Self::from)?),
			6 => {
				let (a, b) = src.split_at(4);
				Some(parse4(a).map(Self::from)? * 100 + parse2(b).map(Self::from)?)
			},
			7 => Some(
				parse4(&src[..4]).map(Self::from)? * 1000 +
				parse1(src[4]).map(Self::from)? * 100 +
				parse2(&src[5..]).map(Self::from)?
			),
			8 => parse8(src),
			9 => Some(parse8(&src[..8])? * 10 + parse1(src[8]).map(Self::from)?),
			10 => {
				let (a, b) = src.split_at(8);
				parse8(a)?.checked_mul(100)?.checked_add(parse2(b).map(Self::from)?)
			},
			0 => None,
			// This probably won't work, but let's give it a shot.
			_ => {
				let (a, b) = src.split_at(10);
				let start = Self::btou(a)?;
				b.iter()
					.try_fold(start, |a, &b| a.checked_mul(10)?.checked_add(parse1(b).map(Self::from)?))
			},
		}
	}

	big!();
}



impl BytesToUnsigned for u64 {
	#[cfg(target_endian = "little")]
	#[allow(clippy::cognitive_complexity)] // Manual indexing wins.
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1..=9 => u32::btou(src).map(Self::from),
			10 => {
				let (a, b) = src.split_at(8);
				Some(parse8(a).map(Self::from)? * 100 + parse2(b).map(Self::from)?)
			},
			11 => Some(
				parse8(&src[..8]).map(Self::from)? * 1000 +
				parse1(src[8]).map(Self::from)? * 100 +
				parse2(&src[9..]).map(Self::from)?
			),
			12 => {
				let (a, b) = src.split_at(8);
				Some(parse8(a).map(Self::from)? * 10_000 + parse4(b).map(Self::from)?)
			},
			13 => Some(
				parse8(&src[..8]).map(Self::from)? * 100_000 +
				parse4(&src[8..12]).map(Self::from)? * 10 +
				parse1(src[12]).map(Self::from)?
			),
			14 => Some(
				parse8(&src[..8]).map(Self::from)? * 1_000_000 +
				parse4(&src[8..12]).map(Self::from)? * 100 +
				parse2(&src[12..]).map(Self::from)?
			),
			15 => Some(
				parse8(&src[..8]).map(Self::from)? * 10_000_000 +
				parse4(&src[8..12]).map(Self::from)? * 1000 +
				parse1(src[12]).map(Self::from)? * 100 +
				parse2(&src[13..]).map(Self::from)?
			),
			16 => parse16(src),
			17 => Some(parse16(&src[..16])? * 10 + parse1(src[16]).map(Self::from)?),
			18 => {
				let (a, b) = src.split_at(16);
				Some(parse16(a)? * 100 + parse2(b).map(Self::from)?)
			},
			19 => Some(
				parse16(&src[..16])? * 1000 +
				parse1(src[16]).map(Self::from)? * 100 +
				parse2(&src[17..]).map(Self::from)?
			),
			20 => {
				let (a, b) = src.split_at(16);
				parse16(a)?.checked_mul(10_000)?.checked_add(parse4(b).map(Self::from)?)
			},
			0 => None,
			// This probably won't work, but let's give it a shot.
			_ => {
				let (a, b) = src.split_at(20);
				let start = Self::btou(a)?;
				b.iter()
					.try_fold(start, |a, &b| a.checked_mul(10)?.checked_add(parse1(b).map(Self::from)?))
			},
		}
	}

	big!();
}



impl BytesToUnsigned for u128 {
	#[cfg(target_endian = "little")]
	#[allow(clippy::cognitive_complexity)] // Manual indexing wins.
	#[allow(clippy::too_many_lines)] // Agreed! These numbers are fucking huge!
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1..=19 => u64::btou(src).map(Self::from),
			20 => {
				let (a, b) = src.split_at(16);
				Some(parse16(a).map(Self::from)? * 10_000 + parse4(b).map(Self::from)?)
			},
			21 => Some(
				parse16(&src[..16]).map(Self::from)? * 100_000 +
				parse4(&src[16..20]).map(Self::from)? * 10 +
				parse1(src[20]).map(Self::from)?
			),
			22 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000 +
				parse4(&src[16..20]).map(Self::from)? * 100 +
				parse2(&src[20..]).map(Self::from)?
			),
			23 => Some(
				parse16(&src[..16]).map(Self::from)? * 10_000_000 +
				parse4(&src[16..20]).map(Self::from)? * 1000 +
				parse1(src[20]).map(Self::from)? * 100 +
				parse2(&src[21..]).map(Self::from)?
			),
			24 => {
				let (a, b) = src.split_at(16);
				Some(parse16(a).map(Self::from)? * 100_000_000 + parse8(b).map(Self::from)?)
			},
			25 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 10 +
				parse1(src[24]).map(Self::from)?
			),
			26 => Some(
				parse16(&src[..16]).map(Self::from)? * 10_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 100 +
				parse2(&src[24..]).map(Self::from)?
			),
			27 => Some(
				parse16(&src[..16]).map(Self::from)? * 100_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 1000 +
				parse1(src[24]).map(Self::from)? * 100 +
				parse2(&src[25..]).map(Self::from)?
			),
			28 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 10_000 +
				parse4(&src[24..]).map(Self::from)?
			),
			29 => Some(
				parse16(&src[..16]).map(Self::from)? * 10_000_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 100_000 +
				parse4(&src[24..28]).map(Self::from)? * 10 +
				parse1(src[28]).map(Self::from)?
			),
			30 => Some(
				parse16(&src[..16]).map(Self::from)? * 100_000_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 1_000_000 +
				parse4(&src[24..28]).map(Self::from)? * 100 +
				parse2(&src[28..]).map(Self::from)?
			),
			31 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000_000_000_000 +
				parse8(&src[16..24]).map(Self::from)? * 10_000_000 +
				parse4(&src[24..28]).map(Self::from)? * 1000 +
				parse1(src[28]).map(Self::from)? * 100 +
				parse2(&src[29..]).map(Self::from)?
			),
			32 => {
				let (a, b) = src.split_at(16);
				Some(parse16(a).map(Self::from)? * 10_000_000_000_000_000 + parse16(b).map(Self::from)?)
			},
			33 => Some(
				parse16(&src[..16]).map(Self::from)? * 100_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 10 +
				parse1(src[32]).map(Self::from)?
			),
			34 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 100 +
				parse2(&src[32..]).map(Self::from)?
			),
			35 => Some(
				parse16(&src[..16]).map(Self::from)? * 10_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 1000 +
				parse1(src[32]).map(Self::from)? * 100 +
				parse2(&src[33..]).map(Self::from)?
			),
			36 => Some(
				parse16(&src[..16]).map(Self::from)? * 100_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 10_000 +
				parse4(&src[32..]).map(Self::from)?
			),
			37 => Some(
				parse16(&src[..16]).map(Self::from)? * 1_000_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 100_000 +
				parse4(&src[32..36]).map(Self::from)? * 10 +
				parse1(src[36]).map(Self::from)?
			),
			38 => Some(
				parse16(&src[..16]).map(Self::from)? * 10_000_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 1_000_000 +
				parse4(&src[32..36]).map(Self::from)? * 100 +
				parse2(&src[36..]).map(Self::from)?
			),
			39 => (
				parse16(&src[..16]).map(Self::from)? * 10_000_000_000_000_000_000_000 +
				parse16(&src[16..32]).map(Self::from)? * 1_000_000 +
				parse4(&src[32..36]).map(Self::from)? * 100 +
				parse2(&src[36..38]).map(Self::from)?
			)
				.checked_mul(10)?.checked_add(parse1(src[38]).map(Self::from)?),
			0 => None,
			// This probably won't work, but let's give it a shot.
			_ => {
				let (a, b) = src.split_at(39);
				let start = Self::btou(a)?;
				b.iter()
					.try_fold(start, |a, &b| a.checked_mul(10)?.checked_add(parse1(b).map(Self::from)?))
			},
		}
	}

	big!();
}


#[allow(clippy::cast_possible_truncation)] // We're matching with pointer widths.
impl BytesToUnsigned for usize {
	#[cfg(target_pointer_width = "16")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { u16::btou(src).map(Self::from) }

	#[cfg(target_pointer_width = "32")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { Some(u32::btou(src)? as Self) }

	#[cfg(target_pointer_width = "64")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { Some(u64::btou(src)? as Self) }

	#[cfg(target_pointer_width = "128")]
	#[must_use]
	/// # Bytes to Unsigned.
	fn btou(src: &[u8]) -> Option<Self> { Some(u128::btou(src)? as Self) }
}


/// # Helper: Non-Zero.
macro_rules! nonzero {
	($($outer:ty, $inner:ty),+ $(,)?) => ($(
		impl BytesToUnsigned for $outer {
			/// # Bytes to Unsigned.
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



#[cfg(target_endian = "little")]
/// # Parse One.
///
/// This converts a single byte into a digit, or dies trying.
const fn parse1(byte: u8) -> Option<u8> {
	let byte = byte ^ b'0';
	if byte < 10 { Some(byte) }
	else { None }
}

#[cfg(target_endian = "little")]
/// # Parse Two.
///
/// This parses two digits as a single `u16`, reducing the number of
/// operations that would otherwise be required.
const fn parse2(src: &[u8]) -> Option<u16> {
	assert!(src.len() == 2, "Bug: parse2 requires 2 bytes.");
	let chunk = u16::from_le_bytes([src[0], src[1]]) ^ 0x3030_u16;

	// Make sure the slice contains only ASCII digits.
	if (chunk & 0xf0f0_u16) | (chunk.wrapping_add(0x7676_u16) & 0x8080_u16) == 0 {
		Some(
			((chunk & 0x000f) << 1) +
			((chunk & 0x000f) << 3) +
			((chunk & 0x0f00) >> 8)
		)
	}
	else { None }
}

#[cfg(target_endian = "little")]
#[allow(clippy::cast_possible_truncation)] // Four digits always fit `u16`.
/// # Parse Four.
///
/// This parses four digits as a single `u32`, reducing the number of
/// operations that would otherwise be required. The return value is downcast
/// to `u16` because four digits will always fit the type.
const fn parse4(src: &[u8]) -> Option<u16> {
	assert!(src.len() == 4, "Bug: parse4 requires 4 bytes.");
	let chunk = u32::from_le_bytes([
		src[0], src[1], src[2],  src[3],
	]) ^ 0x3030_3030;

	// Make sure the slice contains only ASCII digits.
	if (chunk & 0xf0f0_f0f0_u32) | (chunk.wrapping_add(0x7676_7676_u32) & 0x8080_8080_u32) == 0 {
		// 1-byte mask trick (works on 4 pairs of single digits)
		let lower_digits = (chunk & 0x0f00_0f00) >> 8;
		let chunk = lower_digits + (chunk & 0x000f_000f) * 10;
		let masked = chunk as u16;

		// Multiply by 100 via shifts
		let m1 = masked << 6;
		let m2 = masked << 5;
		let m3 = masked << 2;

		let r = ((chunk & 0x00ff_0000) >> 16) as u16;

		// 2-byte mask trick (works on 2 pairs of two digits)
		Some(r + m1 + m2 + m3)
	}
	else { None }
}

#[cfg(target_endian = "little")]
#[allow(clippy::cast_possible_truncation)] // Eight digits always fit `u32`.
/// # Parse Eight.
///
/// This parses eight digits as a single `u64`, reducing the number of
/// operations that would otherwise be required. The return value is downcast
/// to `u32` because eight digits will always fit the type.
const fn parse8(src: &[u8]) -> Option<u32> {
	assert!(src.len() == 8, "Bug: parse8 requires 8 bytes.");
	let chunk = u64::from_le_bytes([
		src[0], src[1], src[2],  src[3],  src[4],  src[5],  src[6],  src[7],
	]) ^ 0x3030_3030_3030_3030_u64;

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

#[cfg(target_endian = "little")]
#[allow(clippy::cast_possible_truncation)] // Sixteen digits always fit `u16`.
/// # Parse Sixteen.
///
/// This parses sixteen digits as a single `u128`, reducing the number of
/// operations that would otherwise be required. The return value is downcast
/// to `u64` because sixteen digits will always fit the type.
const fn parse16(src: &[u8]) -> Option<u64> {
	assert!(src.len() == 16, "Bug: parse16 requires 16 bytes.");
	let chunk = u128::from_le_bytes([
		src[0], src[1], src[2],  src[3],  src[4],  src[5],  src[6],  src[7],
		src[8], src[9], src[10], src[11], src[12], src[13], src[14], src[15],
	]) ^
		0x3030_3030_3030_3030_3030_3030_3030_3030_u128;

	// Make sure the slice contains only ASCII digits.
	let chk = chunk.wrapping_add(0x7676_7676_7676_7676_7676_7676_7676_7676_u128);
	if (chunk & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_u128) | (chk & 0x8080_8080_8080_8080_8080_8080_8080_8080_u128) == 0 {
		// 1-byte mask trick (works on 8 pairs of single digits)
		let lower_digits = (chunk & 0x0f00_0f00_0f00_0f00_0f00_0f00_0f00_0f00) >> 8;
		let upper_digits = (chunk & 0x000f_000f_000f_000f_000f_000f_000f_000f) * 10;
		let chunk = lower_digits + upper_digits;

		// 2-byte mask trick (works on 4 pairs of two digits)
		let lower_digits = (chunk & 0x00ff_0000_00ff_0000_00ff_0000_00ff_0000) >> 16;
		let upper_digits = (chunk & 0x0000_00ff_0000_00ff_0000_00ff_0000_00ff) * 100;
		let chunk = lower_digits + upper_digits;

		// 4-byte mask trick (works on 2 pair of four digits)
		let lower_digits = (chunk & 0x0000_ffff_0000_0000_0000_ffff_0000_0000) >> 32;
		let upper_digits = (chunk & 0x0000_0000_0000_ffff_0000_0000_0000_ffff) * 10_000;
		let chunk = lower_digits + upper_digits;

		// 8-byte mask trick (works on a pair of eight digits)
		let lower_digits = ((chunk & 0x0000_0000_ffff_ffff_0000_0000_0000_0000) >> 64) as u64;
		let upper_digits = (chunk as u64) * 100_000_000;
		Some(lower_digits + upper_digits)
	}
	else { None }
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
