/*!
# Dactyl: Hex Decode Trait
*/

#![expect(
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	trivial_numeric_casts,
	reason = "Macros made me do it.",
)]

use crate::int;



/// # Hex (ASCII Bytes) to Unsigned.
///
/// This trait exposes the method `htou` to decode an ASCII byte slice of
/// hex values into a proper unsigned integer.
///
/// This method returns `None` if the slice is empty or the resulting value is
/// too big for the type.
///
/// (Excessive leading zeroes are fine and will simply be stripped.)
///
/// For signed integers, see [`HexToSigned`].
///
/// ## Examples
///
/// ```
/// use dactyl::traits::HexToUnsigned;
///
/// assert_eq!(u8::htou(b"d"),   Some(13));
/// assert_eq!(u8::htou(b"D"),   Some(13));
/// assert_eq!(u8::htou(b"0d"),  Some(13));
/// assert_eq!(u8::htou(b"00D"), Some(13));
///
/// // These are no good.
/// assert!(u8::htou(&[]).is_none());      // Empty.
/// assert!(u8::htou(b"+13").is_none());   // "+"
/// assert!(u8::htou(b" 13  ").is_none()); // Whitespace.
/// assert!(u8::htou(b"1234").is_none());  // Too big.
/// ```
pub trait HexToUnsigned: Sized {
	/// # Hex (ASCII Bytes) to Unsigned.
	fn htou(hex: &[u8]) -> Option<Self>;
}

/// # Helper: Signed Impls.
///
/// Signed types use their unsigned counterpart's decoder, then are transmuted
/// after to handle any wrapping.
macro_rules! unsigned {
	($($ty:ty)+) => ($(
		impl HexToUnsigned for $ty {
			#[inline]
			/// # Hex (ASCII Bytes) to Unsigned.
			fn htou(mut src: &[u8]) -> Option<Self> {
				// Must not be empty, at least to start with.
				if src.is_empty() { return None; }

				// Trim leading zeroes.
				while let [ b'0', rest @ .. ] = src { src = rest; }

				// The result must be within twice the byte size of the
				// primitive.
				if size_of::<Self>() * 2 < src.len() { return None; }

				// Add up the rest!
				let mut out: Self = 0;
				while let [ v, rest @ .. ] = src {
					out *= 16;
					let v = (*v as char).to_digit(16)?;
					out += v as Self;
					src = rest;
				}

				Some(out)
			}
		}
	)+);
}

unsigned!(u8 u16 u32 u64 u128);

impl HexToUnsigned for usize {
	#[inline]
	/// # Hex (ASCII Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		<int!(@alias usize)>::htou(src).map(|n| n as Self)
	}
}



/// # Hex (ASCII Bytes) to Unsigned.
///
/// This trait exposes the method `htou` to decode an ASCII byte slice of
/// hex values into a proper unsigned integer.
///
/// This method returns `None` if the slice is empty or the resulting value is
/// too big for the type.
///
/// (Excessive leading zeroes are fine and will simply be stripped.)
///
/// For signed integers, see [`HexToSigned`].
///
/// ## Examples
///
/// ```
/// use dactyl::traits::HexToSigned;
///
/// assert_eq!(i8::htoi(b"fB"),  Some(-5));
/// assert_eq!(i8::htoi(b"0FB"), Some(-5));
///
/// // These are no good.
/// assert!(i8::htoi(&[]).is_none());      // Empty.
/// assert!(i8::htoi(b"-fb").is_none());   // "-"
/// assert!(i8::htoi(b" FB  ").is_none()); // Whitespace.
/// assert!(i8::htoi(b"FFB").is_none());   // Too big.
/// ```
pub trait HexToSigned: Sized {
	/// # Hex (ASCII Bytes) to Signed.
	fn htoi(hex: &[u8]) -> Option<Self>;
}

/// # Helper: Signed Impls.
///
/// Signed types use their unsigned counterpart's decoder, then are transmuted
/// after to handle any wrapping.
macro_rules! signed {
	($($ty:ident)+) => ($(
		impl HexToSigned for $ty {
			#[inline]
			/// # Hex (ASCII Bytes) to Signed.
			fn htoi(src: &[u8]) -> Option<Self> {
				<int!(@flip $ty)>::htou(src).map(<int!(@flip $ty)>::cast_signed)
			}
		}
	)+);
}

signed!{ i8 i16 i32 i64 i128 isize }



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 500; // Miri runs way too slow for a million tests.

	macro_rules! test_all {
		($tfn:ident, $hfn:ident, $ty:ty) => (
			#[test]
			fn $tfn() {
				for i in <$ty>::MIN..=<$ty>::MAX { hex!($hfn, i, $ty); }
			}
		);
	}
	macro_rules! test_rng {
		($tfn:ident, $hfn:ident, $ty:ident) => (
			#[test]
			fn $tfn() {
				// Test a reasonable random range of values.
				let mut rng = fastrand::Rng::new();
				for i in std::iter::repeat_with(|| rng.$ty(..)).take(SAMPLE_SIZE) {
					hex!($hfn, i, $ty);
				}

				// Explicitly test the min, max, and zero.
				for i in [<$ty>::MIN, 0, <$ty>::MAX] { hex!($hfn, i, $ty); }
			}
		);
	}

	macro_rules! hex {
		($fn:ident, $num:ident, $ty:ty) => (
			// Unpadded lower, upper.
			let mut s = format!("{:x}", $num);
			assert_eq!(<$ty>::$fn(s.as_bytes()), Some($num));
			s.make_ascii_uppercase();
			assert_eq!(<$ty>::$fn(s.as_bytes()), Some($num));

			// Padded upper, lower.
			let width = std::mem::size_of::<$ty>() * 2;
			if s.len() < width {
				while s.len() < width { s.insert(0, '0'); }
				assert_eq!(<$ty>::$fn(s.as_bytes()), Some($num));
				s.make_ascii_lowercase();
				assert_eq!(<$ty>::$fn(s.as_bytes()), Some($num));
			}
		);
	}

	// Test full set for small types.
	test_all!(t_u8, htou, u8);
	#[cfg(not(miri))] test_all!(t_u16, htou, u16);

	test_all!(t_i8, htoi, i8);
	#[cfg(not(miri))] test_all!(t_i16, htoi, i16);

	// Test random range for larger types.
	#[cfg(miri)] test_rng!(t_u16, htou, u16);
	test_rng!(t_u32, htou, u32);
	test_rng!(t_u64, htou, u64);
	test_rng!(t_u128, htou, u128);
	test_rng!(t_usize, htou, usize);

	#[cfg(miri)] test_rng!(t_i16, htoi, i16);
	test_rng!(t_i32, htoi, i32);
	test_rng!(t_i64, htoi, i64);
	test_rng!(t_i128, htoi, i128);
	test_rng!(t_isize, htoi, isize);
}
