/*!
# Dactyl: Nice u16.
*/

use crate::NiceWrapper;
use std::num::NonZeroU16;



/// # Total Buffer Size.
///
/// 65535 + one comma = six bytes.
const SIZE: usize = 6;

/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b'0', b'0', $sep, b'0', b'0', b'0']);
}



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
///
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NiceU16` has a bunch, including:
///
/// * `AsRef<[u8]>`
/// * `AsRef<str>`
/// * `Borrow<[u8]>`
/// * `Borrow<str>`
/// * `Clone`
/// * `Copy`
/// * `Default`
/// * `Deref<Target=[u8]>`
/// * `Display`
/// * `Eq` / `PartialEq`
/// * `Hash`
/// * `Ord` / `PartialOrd`
///
/// You can instantiate a `NiceU16` with:
///
/// * `From<u16>`
/// * `From<Option<u16>>`
/// * `From<NonZeroU16>`
/// * `From<Option<NonZeroU16>>`
///
/// When converting from a `None`, the result will be equivalent to zero.
pub type NiceU16 = NiceWrapper<SIZE>;

super::nice_default!(NiceU16, SIZE);
super::nice_from!(NiceU16, u16);
super::nice_from_nz!(NiceU16, NonZeroU16);
super::nice_from_opt!(NiceU16);

impl NiceU16 {
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
			let (num, rem) = crate::div_mod(num, 1000);
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

		// Test some Option variants.
		let foo: Option<u16> = None;
		assert_eq!(NiceU16::min(), NiceU16::from(foo));
		let foo = Some(13_u16);
		assert_eq!(NiceU16::from(13_u16), NiceU16::from(foo));

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

		// Test some Option variants.
		let foo: Option<NonZeroU16> = None;
		assert_eq!(NiceU16::from(foo), NiceU16::min());
		let foo = NonZeroU16::new(13);
		assert_eq!(NiceU16::from(13_u16), NiceU16::from(foo));
	}

	#[test]
	fn t_as() {
		let num = NiceU16::from(1234_u16);
		assert_eq!(num.as_str(), String::from(num));
		assert_eq!(num.as_bytes(), Vec::<u8>::from(num));
	}
}
