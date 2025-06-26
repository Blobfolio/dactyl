/*!
# Dactyl: Nice `u8`.
*/

use super::{
	nice_arr,
	nice_str,
	nice_uint,
	NiceChar,
};
use std::num::NonZeroU8;



#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
/// # `NiceU8` Indices.
enum NiceU8Idx {
	From00 = 0_u8, // 2
	From01 = 1_u8, // 5
	From02 = 2_u8, // 5
}

#[cfg_attr(
	not(test),
	expect(dead_code, reason = "For consistency with bigger index types.")
)]
impl NiceU8Idx {
	/// # Digit Indices (Reverse Order).
	const DIGITS: [Self; 3] = [
		Self::From02, Self::From01, Self::From00,
	];

	/// # Last.
	const LAST: Self = Self::From02;

	/// # Length.
	const LEN: usize = 3;
}



nice_uint! {
	NiceU8, NiceU8Idx,
	u8, NonZeroU8,
	[ 0 0 0 ],
	[ 2 5 5 ]
}

impl From<u8> for NiceU8 {
	#[inline]
	fn from(src: u8) -> Self {
		let (data, from) = data_from(src);
		Self { data, from }
	}
}

impl NiceU8 {
	#[must_use]
	#[inline]
	/// # Double Digit Bytes.
	///
	/// This method returns a zero-padded two-byte representation of the
	/// nice value, _except_ for values requiring three digits (i.e. `100+`).
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_bytes2(), b"03");
	/// assert_eq!(dactyl::NiceU8::from(50).as_bytes2(), b"50");
	/// assert_eq!(dactyl::NiceU8::from(113).as_bytes2(), b"113");
	/// ```
	pub const fn as_bytes2(&self) -> &[u8] {
		NiceChar::as_bytes(
			if matches!(self.from, NiceU8Idx::From00) { self.data.as_slice() }
			else {
				let (_, rest) = self.data.split_at(1);
				rest
			}
		)
	}

	#[must_use]
	#[inline]
	/// # Triple Digit Bytes.
	///
	/// This method returns a zero-padded three-byte representation of the
	/// nice value.
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_bytes3(), b"003");
	/// assert_eq!(dactyl::NiceU8::from(50).as_bytes3(), b"050");
	/// assert_eq!(dactyl::NiceU8::from(113).as_bytes3(), b"113");
	/// ```
	pub const fn as_bytes3(&self) -> &[u8] {
		NiceChar::as_bytes(self.data.as_slice())
	}

	#[must_use]
	#[inline]
	/// # Double Digit Str.
	///
	/// This method returns a zero-padded two-byte representation of the
	/// nice value, _except_ for values requiring three digits (i.e. `100+`).
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_str2(), "03");
	/// assert_eq!(dactyl::NiceU8::from(50).as_str2(), "50");
	/// assert_eq!(dactyl::NiceU8::from(113).as_str2(), "113");
	/// ```
	pub const fn as_str2(&self) -> &str {
		NiceChar::as_str(
			if matches!(self.from, NiceU8Idx::From00) { self.data.as_slice() }
			else {
				let (_, rest) = self.data.split_at(1);
				rest
			}
		)
	}

	#[must_use]
	#[inline]
	/// # Triple Digit Str.
	///
	/// This method returns a zero-padded three-byte representation of the
	/// nice value.
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_str3(), "003");
	/// assert_eq!(dactyl::NiceU8::from(50).as_str3(), "050");
	/// assert_eq!(dactyl::NiceU8::from(113).as_str3(), "113");
	/// ```
	pub const fn as_str3(&self) -> &str {
		NiceChar::as_str(self.data.as_slice())
	}
}

impl NiceU8 {
	/// # Replace.
	///
	/// Reuse the backing storage behind `self` to hold a new nice number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceU8;
	///
	/// let mut num = NiceU8::from(123_u8);
	/// assert_eq!(num.as_str(), "123");
	///
	/// num.replace(1);
	/// assert_eq!(num.as_str(), "1");
	/// ```
	pub const fn replace(&mut self, src: u8) {
		(self.data, self.from) = data_from(src);
	}
}



#[inline]
/// # Data and From.
///
/// The `u8` range is too small to benefit from our `Digiter` helper; this
/// method handles the conversion all in one go, and is constant.
const fn data_from(num: u8) -> ([NiceChar; 3], NiceU8Idx) {
	// Three digits.
	if 99 < num {
		let a = if 199 < num { NiceChar::Digit2 } else { NiceChar::Digit1 };
		let b = NiceChar::from_digit_u8(num / 10);
		let c = NiceChar::from_digit_u8(num);
		([a, b, c], NiceU8Idx::From00)
	}
	// Two digits.
	else if 9 < num {
		let b = NiceChar::from_digit_u8(num / 10);
		let c = NiceChar::from_digit_u8(num);
		([NiceChar::Digit0, b, c], NiceU8Idx::From01)
	}
	// One digit.
	else {
		(
			[NiceChar::Digit0, NiceChar::Digit0, NiceChar::from_digit_u8(num)],
			NiceU8Idx::From02
		)
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice() {
		// Explicitly check default, min, and max.
		assert_eq!(NiceU8::default(), NiceU8::from(0));
		assert_eq!(NiceU8::MIN, NiceU8::from(0));
		assert_eq!(NiceU8::MAX, NiceU8::from(u8::MAX));

		let mut last = NiceU8::MAX;
		for i in u8::MIN..=u8::MAX {
			let istr = i.to_string();
			let nice = NiceU8::from(i);

			assert_eq!(istr, nice.as_str());
			assert_eq!(istr.as_bytes(), nice.as_bytes());
			assert_eq!(istr.len(), nice.len());

			// This should not equal the last value!
			assert_ne!(nice, last);

			// Now it should!
			last.replace(i);
			assert_eq!(nice, last);
		}
	}

	super::super::nice_test!(NiceU8Idx);
}
