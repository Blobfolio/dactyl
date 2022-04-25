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
	#[allow(unsafe_code)]
	/// # Parse.
	///
	/// This handles the actual crunching.
	fn parse(&mut self, num: u16) {
		let ptr = self.inner.as_mut_ptr();

		if 999 < num {
			let (num, rem) = crate::div_mod_u16(num, 1000);
			unsafe { super::write_u8_3(ptr.add(3), rem); }

			if 9 < num {
				self.from = 0;
				unsafe {
					std::ptr::copy_nonoverlapping(
						crate::double(num as usize),
						ptr,
						2
					);
				}
			}
			else {
				self.from = 1;
				unsafe { std::ptr::write(ptr.add(1), num as u8 + b'0'); }
			}
		}
		else if 99 < num {
			self.from = 3;
			unsafe { super::write_u8_3(ptr.add(3), num); }
		}
		else if 9 < num {
			self.from = 4;
			unsafe {
				std::ptr::copy_nonoverlapping(
					crate::double(num as usize),
					ptr.add(4),
					2
				);
			}
		}
		else {
			self.from = 5;
			unsafe { std::ptr::write(ptr.add(5), num as u8 + b'0'); }
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

		#[cfg(not(miri))]
		for i in 0..=u16::MAX {
			assert_eq!(
				NiceU16::from(i).as_str(),
				i.to_formatted_string(&Locale::en),
			);
		}

		#[cfg(miri)]
		{
			let rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u16(..)).take(1000) {
				assert_eq!(
					NiceU16::from(i).as_str(),
					i.to_formatted_string(&Locale::en),
				);
			}
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
