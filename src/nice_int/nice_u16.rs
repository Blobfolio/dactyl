/*!
# Dactyl: Nice u16.
*/

use std::num::NonZeroU16;



/// # Total Buffer Size.
const SIZE: usize = 6;



#[derive(Debug, Clone, Copy)]
/// `NiceU16` provides a quick way to convert a `u16` into a formatted byte
/// string for e.g. printing. Commas are added for every thousand.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU16;
/// assert_eq!(
///     NiceU16::from(33231).as_str(),
///     "33,231"
/// );
/// ```
pub struct NiceU16 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU16 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE,
		}
	}
}

impl From<u16> for NiceU16 {
	fn from(mut num: u16) -> Self {
		let mut out = Self::default();
		let ptr = out.inner.as_mut_ptr();

		// For `u16` this can only trigger once.
		if num >= 1000 {
			let (div, rem) = crate::div_mod_u16(num, 1000);
			unsafe { super::write_u8_3(ptr.add(out.from - 3), usize::from(rem)); }
			num = div;
			out.from -= 4;
		}

		if num >= 100 {
			out.from -= 3;
			unsafe { super::write_u8_3(ptr.add(out.from), usize::from(num)); }
		}
		else if num >= 10 {
			out.from -= 2;
			unsafe { super::write_u8_2(ptr.add(out.from), usize::from(num)); }
		}
		else {
			out.from -= 1;
			unsafe { super::write_u8_1(ptr.add(out.from), usize::from(num)); }
		}

		out
	}
}

impl NiceU16 {
	#[must_use]
	#[inline]
	/// # Min.
	///
	/// This is equivalent to zero.
	pub const fn min() -> Self {
		Self {
			inner: [b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE - 1,
		}
	}
}

// A few Macro traits.
super::impl_nice_nonzero_int!(NiceU16: NonZeroU16);
super::impl_nice_int!(NiceU16);



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u16() {
		assert_eq!(NiceU16::min(), NiceU16::from(0));

		for i in 0..=u16::MAX {
			assert_eq!(
				NiceU16::from(i).as_str(),
				i.to_formatted_string(&Locale::en),
			);
		}

		// Test the defaults too.
		assert_eq!(NiceU16::default().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU16::default().as_str(), "");

		// Check ordering too.
		let one = NiceU16::from(10);
		let two = NiceU16::from(90);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);
	}

	#[test]
	fn t_nice_nonzero_u16() {
		assert_eq!(NiceU16::min(), NiceU16::from(NonZeroU16::new(0)));
		assert_eq!(NiceU16::from(50_u16), NiceU16::from(NonZeroU16::new(50)));
		assert_eq!(NiceU16::from(50_u16), NiceU16::from(NonZeroU16::new(50).unwrap()));
	}
}
