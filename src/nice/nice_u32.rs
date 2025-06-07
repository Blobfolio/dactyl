/*!
# Dactyl: Nice `u32`.
*/

use crate::NiceSeparator;
use std::num::NonZeroU32;
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
/// # `NiceU32` Indices.
enum NiceU32Idx {
	From00 =  0_u8, // 4
	From01 =  1_u8, // ,
	From02 =  2_u8, // 2
	From03 =  3_u8, // 9
	From04 =  4_u8, // 4
	From05 =  5_u8, // ,
	From06 =  6_u8, // 9
	From07 =  7_u8, // 6
	From08 =  8_u8, // 7
	From09 =  9_u8, // ,
	From10 = 10_u8, // 2
	From11 = 11_u8, // 9
	From12 = 12_u8, // 5
}

impl NiceU32Idx {
	/// # Digit Indices (Reverse Order).
	const DIGITS: [Self; 10] = [
		Self::From12, Self::From11, Self::From10, // ,
		Self::From08, Self::From07, Self::From06, // ,
		Self::From04, Self::From03, Self::From02, // ,
		Self::From00,
	];

	/// # Last.
	const LAST: Self = Self::From12;

	/// # Length.
	const LEN: usize = 13;
}



nice_uint! {
	@full
	NiceU32, NiceU32Idx,
	u32, NonZeroU32,
	[ 0 0 0 0 0 0 0 0 0 0 ],
	[ 4 2 9 4 9 6 7 2 9 5 ]
}



#[cfg(test)]
mod tests {
	use super::*;

	super::super::nice_test!(NiceU32, u32);
}
