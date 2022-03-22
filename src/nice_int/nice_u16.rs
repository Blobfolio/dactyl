/*!
# Dactyl: Nice u16.
*/

use std::num::NonZeroU16;



/// # Total Buffer Size.
const SIZE: usize = 6;



/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b'0', b'0', $sep, b'0', b'0', b'0']);
}



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
			inner: inner!(b','),
			from: SIZE,
		}
	}
}

impl From<u16> for NiceU16 {
	fn from(num: u16) -> Self {
		let mut out = Self::default();
		out.parse(num);
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
			inner: inner!(b','),
			from: SIZE - 1,
		}
	}

	#[must_use]
	/// # New Instance w/ Custom Separator.
	///
	/// Create a new instance, defining any arbitrary ASCII byte as the
	/// thousands separator.
	///
	/// If you're good with commas, just use [`NiceU16::from`] instead.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceU16;
	///
	/// let num = NiceU16::from(31415_u16);
	/// assert_eq!(num.as_str(), "31,415");
	///
	/// let num = NiceU16::with_separator(31415_u16, b'_');
	/// assert_eq!(num.as_str(), "31_415");
	/// ```
	///
	/// ## Panics
	///
	/// This method will panic if the separator is invalid ASCII.
	pub fn with_separator(num: u16, sep: u8) -> Self {
		assert!(sep.is_ascii(), "Invalid separator.");
		let mut out = Self {
			inner: inner!(sep),
			from: SIZE,
		};
		out.parse(num);
		out
	}

	#[allow(clippy::cast_possible_truncation)] // One digit always fits u8.
	/// # Parse.
	///
	/// This handles the actual crunching.
	fn parse(&mut self, mut num: u16) {
		let ptr = self.inner.as_mut_ptr();

		// For `u16` this can only trigger once.
		if num >= 1000 {
			let (div, rem) = crate::div_mod_u16(num, 1000);
			self.from -= 4;
			unsafe { super::write_u8_3(ptr.add(self.from + 1), usize::from(rem)); }
			num = div;
		}

		if num >= 100 {
			self.from -= 3;
			unsafe { super::write_u8_3(ptr.add(self.from), usize::from(num)); }
		}
		else if num >= 10 {
			self.from -= 2;
			unsafe { super::write_u8_2(ptr.add(self.from), usize::from(num)); }
		}
		else {
			self.from -= 1;
			unsafe { std::ptr::write(ptr.add(self.from), num as u8 + b'0'); }
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

	#[test]
	fn t_as() {
		let num = NiceU16::from(1234_u16);
		assert_eq!(num.as_str(), num.as_string());
		assert_eq!(num.as_bytes(), num.as_vec());
	}
}
