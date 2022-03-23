/*!
# Dactyl: Nice u32.
*/

use std::num::NonZeroU32;



/// # Total Buffer Size.
const SIZE: usize = 13;



/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0']);
}



#[derive(Debug, Clone, Copy)]
/// `NiceU32` provides a quick way to convert a `u32` into a formatted byte
/// string for e.g. printing. Commas are added for every thousand.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU32;
/// assert_eq!(
///     NiceU32::from(33231).as_str(),
///     "33,231"
/// );
/// ```
pub struct NiceU32 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU32 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: inner!(b','),
			from: SIZE,
		}
	}
}

impl From<u32> for NiceU32 {
	fn from(num: u32) -> Self {
		let mut out = Self::default();
		out.parse(num);
		out
	}
}

impl NiceU32 {
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
	/// If you're good with commas, just use [`NiceU32::from`] instead.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceU32;
	///
	/// let num = NiceU32::from(3141592653_u32);
	/// assert_eq!(num.as_str(), "3,141,592,653");
	///
	/// let num = NiceU32::with_separator(3141592653_u32, b'_');
	/// assert_eq!(num.as_str(), "3_141_592_653");
	/// ```
	///
	/// ## Panics
	///
	/// This method will panic if the separator is invalid ASCII.
	pub fn with_separator(num: u32, sep: u8) -> Self {
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
	fn parse(&mut self, mut num: u32) {
		let ptr = self.inner.as_mut_ptr();

		while 999 < num {
			let (div, rem) = crate::div_mod_u32(num, 1000);
			self.from -= 4;
			unsafe { super::write_u8_3(ptr.add(self.from + 1), rem as u16); }
			num = div;
		}

		if 99 < num {
			self.from -= 3;
			unsafe { super::write_u8_3(ptr.add(self.from), num as u16); }
		}
		else if 9 < num {
			self.from -= 2;
			unsafe {
				std::ptr::copy_nonoverlapping(
					crate::double(num as usize),
					ptr.add(self.from),
					2
				);
			}
		}
		else {
			self.from -= 1;
			unsafe { std::ptr::write(ptr.add(self.from), num as u8 + b'0'); }
		}
	}
}

// A few Macro traits.
super::impl_nice_nonzero_int!(NiceU32: NonZeroU32);
super::impl_nice_int!(NiceU32);



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u32() {
		// Check the min and max.
		assert_eq!(NiceU32::from(0).as_str(), "0");
		assert_eq!(NiceU32::min(), NiceU32::from(0));
		assert_eq!(
			NiceU32::from(u32::MAX).as_str(),
			u32::MAX.to_formatted_string(&Locale::en),
		);

		// Test the defaults too.
		assert_eq!(NiceU32::default().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU32::default().as_str(), "");

		// Check ordering too.
		let one = NiceU32::from(10);
		let two = NiceU32::from(90);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);

		// Check a subset of everything else.
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(..)).take(1_000_000) {
			assert_eq!(
				NiceU32::from(i).as_str(),
				i.to_formatted_string(&Locale::en),
			);
		}
	}


	#[test]
	fn t_nice_nonzero_u32() {
		assert_eq!(NiceU32::min(), NiceU32::from(NonZeroU32::new(0)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50).unwrap()));
	}

	#[test]
	fn t_as() {
		let num = NiceU32::from(12_345_678_u32);
		assert_eq!(num.as_str(), num.as_string());
		assert_eq!(num.as_bytes(), num.as_vec());
	}
}
