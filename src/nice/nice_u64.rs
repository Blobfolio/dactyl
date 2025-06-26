/*!
# Dactyl: Nice `u64`.
*/

use crate::NiceSeparator;
use std::num::{
	NonZeroUsize,
	NonZeroU64,
};
use super::{
	Digiter,
	nice_arr,
	nice_str,
	nice_uint,
	NiceChar,
};



#[expect(dead_code, reason = "For readability.")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
/// # `NiceU64` Indices.
enum NiceU64Idx {
	From00 =  0_u8, // 1
	From01 =  1_u8, // 8
	From02 =  2_u8, // ,
	From03 =  3_u8, // 4
	From04 =  4_u8, // 4
	From05 =  5_u8, // 6
	From06 =  6_u8, // ,
	From07 =  7_u8, // 7
	From08 =  8_u8, // 4
	From09 =  9_u8, // 4
	From10 = 10_u8, // ,
	From11 = 11_u8, // 0
	From12 = 12_u8, // 7
	From13 = 13_u8, // 3
	From14 = 14_u8, // ,
	From15 = 15_u8, // 7
	From16 = 16_u8, // 0
	From17 = 17_u8, // 9
	From18 = 18_u8, // ,
	From19 = 19_u8, // 5
	From20 = 20_u8, // 5
	From21 = 21_u8, // 1
	From22 = 22_u8, // ,
	From23 = 23_u8, // 6
	From24 = 24_u8, // 1
	From25 = 25_u8, // 5
}

impl NiceU64Idx {
	/// # Digit Indices (Reverse Order).
	const DIGITS: [Self; 20] = [
		Self::From25, Self::From24, Self::From23, // ,
		Self::From21, Self::From20, Self::From19, // ,
		Self::From17, Self::From16, Self::From15, // ,
		Self::From13, Self::From12, Self::From11, // ,
		Self::From09, Self::From08, Self::From07, // ,
		Self::From05, Self::From04, Self::From03, // ,
		Self::From01, Self::From00,
	];

	/// # Last.
	const LAST: Self = Self::From25;

	/// # Length.
	const LEN: usize = 26;
}



nice_uint! {
	@full
	NiceU64, NiceU64Idx,
	u64, NonZeroU64,
	[ 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ],
	[ 1 8 4 4 6 7 4 4 0 7 3 7 0 9 5 5 1 6 1 5 ]
}



// Usize is handled by this type to keep things easy.
impl From<usize> for NiceU64 {
	#[inline]
	fn from(src: usize) -> Self { Self::from(src as u64) }
}

impl From<Option<usize>> for NiceU64 {
	#[inline]
	fn from(src: Option<usize>) -> Self {
		src.map_or(Self::MIN, |src| Self::from(src as u64))
	}
}

impl From<NonZeroUsize> for NiceU64 {
	#[inline]
	fn from(src: NonZeroUsize) -> Self { Self::from(src.get() as u64) }
}

impl From<Option<NonZeroUsize>> for NiceU64 {
	#[inline]
	fn from(src: Option<NonZeroUsize>) -> Self {
		src.map_or(Self::MIN, |src| Self::from(src.get() as u64))
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	super::super::nice_test!(NiceU64, u64);

	#[test]
	fn t_nice_usize() {
		use num_format::{ToFormattedString, Locale};

		// This just forwards to the u64 impl, so all we really need to do
		// is check the min and max come out as expected.
		assert_eq!(
			NiceU64::from(usize::MIN).as_str(),
			usize::MIN.to_formatted_string(&Locale::en),
		);
		assert_eq!(
			NiceU64::from(usize::MAX).as_str(),
			usize::MAX.to_formatted_string(&Locale::en),
		);
	}
}
