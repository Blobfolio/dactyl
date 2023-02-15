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



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri runs way too slow for a million tests.

	macro_rules! hex {
		($num:ident, $ty:ty, $len:literal) => (
			let mut s = format!("{:x}", $num);
			assert_eq!(<$ty>::htou(s.as_bytes()), Some($num));
			s.make_ascii_uppercase();
			assert_eq!(<$ty>::htou(s.as_bytes()), Some($num));

			s = format!(concat!("{:0", $len, "x}"), $num);
			assert_eq!(<$ty>::htou(s.as_bytes()), Some($num));
			s.make_ascii_uppercase();
			assert_eq!(<$ty>::htou(s.as_bytes()), Some($num));
		);
	}

	#[test]
	fn t_u8() {
		for i in 0..=u8::MAX { hex!(i, u8, "2"); }
	}

	#[cfg(not(miri))]
	#[test]
	fn t_u16() {
		for i in 0..=u16::MAX { hex!(i, u16, "4"); }
	}

	#[cfg(miri)]
	#[test]
	fn t_u16() {
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u16(..)).take(SAMPLE_SIZE) {
			hex!(i, u16, "4");
		}

		for i in [u16::MIN, u16::MAX] { hex!(i, u16, "4"); }
	}

	#[test]
	fn t_u32() {
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(..)).take(SAMPLE_SIZE) {
			hex!(i, u32, "8");
		}

		for i in [u32::MIN, u32::MAX] { hex!(i, u32, "8"); }
	}

	#[test]
	fn t_u64() {
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(..)).take(SAMPLE_SIZE) {
			hex!(i, u64, "16");
		}

		for i in [u64::MIN, u64::MAX] { hex!(i, u64, "16"); }
	}

	#[test]
	fn t_u128() {
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u128(..)).take(SAMPLE_SIZE) {
			hex!(i, u128, "32");
		}

		for i in [u128::MIN, u128::MAX] { hex!(i, u128, "32"); }
	}
}
