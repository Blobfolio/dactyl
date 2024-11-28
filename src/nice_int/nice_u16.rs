/*!
# Dactyl: Nice u16.
*/

use crate::NiceWrapper;
use std::num::NonZeroU16;



/// # Total Buffer Size.
///
/// 65535 + one comma = six bytes.
const SIZE: usize = 6;

/// # Default Buffer.
const ZERO: [u8; SIZE] = [b'0', b'0', b',', b'0', b'0', b'0'];



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

super::nice_default!(NiceU16, ZERO, SIZE);
super::nice_from_nz!(NiceU16, NonZeroU16);

impl From<u16> for NiceU16 {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[expect(clippy::integer_division, reason = "We want this.")]
	#[expect(clippy::many_single_char_names, reason = "Consistency is preferred.")]
	fn from(num: u16) -> Self {
		if 999 < num {
			let (num, rem) = (num / 1000, num % 1000);
			let [c, d, e] = crate::triple(rem as usize);

			if 9 < num {
				let [a, b] = crate::double(num as usize);
				Self {
					inner: [a, b, b',', c, d, e],
					from: 0,
				}
			}
			else {
				let b = num as u8 + b'0';
				Self {
					inner: [b'0', b, b',', c, d, e],
					from: 1,
				}
			}
		}
		else if 99 < num {
			let [c, d, e] = crate::triple(num as usize);
			Self {
				inner: [b'0', b'0', b',', c, d, e],
				from: 3,
			}
		}
		else {
			let [d, e] = crate::double(num as usize);
			Self {
				inner: [b'0', b'0', b',', b'0', d, e],
				from: if d == b'0' { 5 } else { 4 },
			}
		}
	}
}

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
		let mut out = Self::from(num);
		out.inner[2] = sep;
		out
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u16() {
		assert_eq!(NiceU16::default(), NiceU16::from(0_u16));

		#[cfg(not(miri))]
		for i in 0..=u16::MAX {
			let nice = NiceU16::from(i);
			assert_eq!(
				nice.as_str(),
				i.to_formatted_string(&Locale::en),
			);
			assert_eq!(nice.len(), nice.as_str().len());
			assert_eq!(nice.len(), nice.as_bytes().len());
			assert!(! nice.is_empty());
		}

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u16(..)).take(1000) {
				let nice = NiceU16::from(i);
				assert_eq!(
					nice.as_str(),
					i.to_formatted_string(&Locale::en),
				);
				assert_eq!(nice.len(), nice.as_str().len());
				assert_eq!(nice.len(), nice.as_bytes().len());
				assert!(! nice.is_empty());
			}
		}

		// Test the defaults too.
		assert_eq!(NiceU16::empty().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU16::empty().as_str(), "");
		assert!(NiceU16::empty().is_empty());

		// Test some Option variants.
		let foo: Option<u16> = None;
		assert_eq!(NiceU16::default(), NiceU16::from(foo));
		let foo = Some(13_u16);
		assert_eq!(NiceU16::from(13_u16), NiceU16::from(foo));

		// Check ordering too.
		let one = NiceU16::from(10_u16);
		let two = NiceU16::from(90_u16);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);
	}

	#[test]
	fn t_nice_nonzero_u16() {
		assert_eq!(NiceU16::default(), NiceU16::from(NonZeroU16::new(0)));
		assert_eq!(NiceU16::from(50_u16), NiceU16::from(NonZeroU16::new(50)));
		assert_eq!(NiceU16::from(50_u16), NiceU16::from(NonZeroU16::new(50).unwrap()));

		// Test some Option variants.
		let foo: Option<NonZeroU16> = None;
		assert_eq!(NiceU16::from(foo), NiceU16::default());
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
