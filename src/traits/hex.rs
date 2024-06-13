/*!
# Dactyl: Hex Decode Trait
*/

/// # Not Hex Placeholder Value.
const NIL: u8 = u8::MAX;

/// # Hex Decoding Table.
///
/// This is the same lookup table used by the `faster-hex` crate, or at least,
/// one of them.
static UNHEX: [u8; 256] = [
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, 10, 11, 12, 13, 14, 15, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, 10, 11, 12, 13,
	14, 15, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL,
	NIL, NIL, NIL,
];

/// # Hex (Bytes) to Unsigned.
///
/// This trait exposes the method `htou` which converts Hex byte slices to
/// proper, unsigned integers. It works just like `u*::from_str_radix`, but
/// faster, particularly for `u64` and `u128`.
///
/// Decoding is case-insensitive and padding-agnostic; slice lengths may be
/// anything between `1..=std::mem::size_of::<Self>()*2`.
///
/// Invalid slices, overflows, etc., will result in `None` being returned.
///
/// For signed integers, see [`HexToSigned`].
///
/// ## Examples
///
/// ```
/// use dactyl::traits::HexToUnsigned;
///
/// assert_eq!(u8::htou(b"d"), Some(13));
/// assert_eq!(u8::htou(b"D"), Some(13));
/// assert_eq!(u8::htou(b"0d"), Some(13));
/// assert_eq!(u8::htou(b"0D"), Some(13));
/// ```
pub trait HexToUnsigned: Sized {
	/// # Hex (Bytes) to Unsigned.
	fn htou(hex: &[u8]) -> Option<Self>;
}

impl HexToUnsigned for u8 {
	#[inline]
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		match src.len() {
			1 => {
				let a = UNHEX[src[0] as usize];
				if a == NIL { None }
				else { Some(a) }
			},
			2 => {
				let a = UNHEX[src[0] as usize];
				let b = UNHEX[src[1] as usize];
				if a == NIL || b == NIL { None }
				else { Some(16 * a + b) }
			},
			_ => None,
		}
	}
}

impl HexToUnsigned for u16 {
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		let len = src.len();
		if 0 != len && len <= 4 {
			[4096, 256, 16, 1][4 - len..].iter()
				.zip(src)
				.try_fold(0, |out, (base, byte)| {
					let digit = UNHEX[*byte as usize];
					if digit == NIL { None }
					else { Some(out + Self::from(digit) * base) }
				})
		}
		else { None }
	}
}

impl HexToUnsigned for u32 {
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		let len = src.len();
		if 0 != len && len <= 8 {
			[268_435_456, 16_777_216, 1_048_576, 65_536, 4096, 256, 16, 1][8 - len..].iter()
				.zip(src)
				.try_fold(0, |out, (base, byte)| {
					let digit = UNHEX[*byte as usize];
					if digit == NIL { None }
					else { Some(out + Self::from(digit) * base) }
				})
		}
		else { None }
	}
}

#[cfg(target_pointer_width = "16")]
impl HexToUnsigned for usize {
	#[inline]
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> { u16::htou(src).map(Self::from) }
}

#[cfg(target_pointer_width = "32")]
#[allow(clippy::cast_possible_truncation)]
impl HexToUnsigned for usize {
	#[inline]
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> { u32::htou(src).map(|n| n as Self) }
}

#[cfg(target_pointer_width = "64")]
#[allow(clippy::cast_possible_truncation)]
impl HexToUnsigned for usize {
	#[inline]
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> { u64::htou(src).map(|n| n as Self) }
}

impl HexToUnsigned for u64 {
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		let len = src.len();
		match len {
			1..=8 => u32::htou(src).map(Self::from),
			9..=16 => {
				let (src, rhs) = src.split_at(len - 8);
				let rhs = u32::htou(rhs).map(Self::from)?;
				[
					1_152_921_504_606_846_976,
					72_057_594_037_927_936,
					4_503_599_627_370_496,
					281_474_976_710_656,
					17_592_186_044_416,
					1_099_511_627_776,
					68_719_476_736,
					4_294_967_296,
				][8 - src.len()..].iter()
					.zip(src)
					.try_fold(rhs, |out, (base, byte)| {
						let digit = UNHEX[*byte as usize];
						if digit == NIL { None }
						else { Some(out + Self::from(digit) * base) }
					})
			},
			_ => None,
		}
	}
}

impl HexToUnsigned for u128 {
	/// # Hex (Bytes) to Unsigned.
	fn htou(src: &[u8]) -> Option<Self> {
		let len = src.len();
		match len {
			1..=16 => u64::htou(src).map(Self::from),
			17..=32 => {
				let (src, rhs) = src.split_at(len - 16);
				let rhs = u64::htou(rhs).map(Self::from)?;
				[
					21_267_647_932_558_653_966_460_912_964_485_513_216,
					1_329_227_995_784_915_872_903_807_060_280_344_576,
					83_076_749_736_557_242_056_487_941_267_521_536,
					5_192_296_858_534_827_628_530_496_329_220_096,
					324_518_553_658_426_726_783_156_020_576_256,
					20_282_409_603_651_670_423_947_251_286_016,
					1_267_650_600_228_229_401_496_703_205_376,
					79_228_162_514_264_337_593_543_950_336,
					4_951_760_157_141_521_099_596_496_896,
					309_485_009_821_345_068_724_781_056,
					19_342_813_113_834_066_795_298_816,
					1_208_925_819_614_629_174_706_176,
					75_557_863_725_914_323_419_136,
					4_722_366_482_869_645_213_696,
					295_147_905_179_352_825_856,
					18_446_744_073_709_551_616,
				][16 - src.len()..].iter()
					.zip(src)
					.try_fold(rhs, |out, (base, byte)| {
						let digit = UNHEX[*byte as usize];
						if digit == NIL { None }
						else { Some(out + Self::from(digit) * base) }
					})
			},
			_ => None,
		}
	}
}



/// # Hex (Bytes) to Signed.
///
/// This trait exposes the method `htoi` which converts Hex byte slices to
/// proper, signed integers. It works just like `i*::from_str_radix`, but
/// faster, particularly for `i64` and `i128`.
///
/// Decoding is case-insensitive and padding-agnostic; slice lengths may be
/// anything between `1..=std::mem::size_of::<Self>()*2`.
///
/// Invalid slices, overflows, etc., will result in `None` being returned.
///
/// For unsigned integers, see [`HexToUnsigned`].
///
/// ## Examples
///
/// ```
/// use dactyl::traits::HexToSigned;
///
/// assert_eq!(i8::htoi(b"fb"), Some(-5));
/// assert_eq!(i8::htoi(b"FB"), Some(-5));
/// ```
pub trait HexToSigned: Sized {
	/// # Hex (Bytes) to Signed.
	fn htoi(hex: &[u8]) -> Option<Self>;
}

/// # Helper: Signed Impls.
///
/// Signed types use their unsigned counterpart's decoder, then are transmuted
/// after to handle any wrapping.
macro_rules! signed {
	($signed:ty, $unsigned:ty) => (
		impl HexToSigned for $signed {
			#[inline]
			#[allow(clippy::cast_possible_wrap)]
			/// # Hex (Bytes) to Signed.
			fn htoi(src: &[u8]) -> Option<Self> {
				<$unsigned>::htou(src).map(|n| n as Self)
			}
		}
	);
}

signed!(i8, u8);
signed!(i16, u16);
signed!(i32, u32);
signed!(i64, u64);
signed!(i128, u128);
signed!(isize, usize);



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
