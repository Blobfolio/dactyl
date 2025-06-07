/*!
# Dactyl: Nice `u16`.
*/

use crate::NiceSeparator;
use std::num::NonZeroU16;
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
/// # `NiceU16` Indices.
enum NiceU16Idx {
	From00 = 0_u8, // 6
	From01 = 1_u8, // 5
	From02 = 2_u8, // ,
	From03 = 3_u8, // 5
	From04 = 4_u8, // 3
	From05 = 5_u8, // 5
}

impl NiceU16Idx {
	/// # Digit Indices (Reverse Order).
	const DIGITS: [Self; 5] = [
		Self::From05, Self::From04, Self::From03, // ,
		Self::From01, Self::From00,
	];

	/// # Last.
	const LAST: Self = Self::From05;

	/// # Length.
	const LEN: usize = 6;
}



nice_uint! {
	@full
	NiceU16, NiceU16Idx,
	u16, NonZeroU16,
	[ 0 0 0 0 0 ],
	[ 6 5 5 3 5 ]
}



#[cfg(test)]
mod tests {
	use super::*;

	super::super::nice_test!(NiceU16, u16, NiceU16Idx);
}
