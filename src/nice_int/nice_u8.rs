/*!
# Dactyl: Nice u8.
*/

use crate::NiceWrapper;
use std::num::NonZeroU8;



/// # Total Buffer Size.
///
/// 255 needs no commas, so is only three bytes.
const SIZE: usize = 3;

/// # Zero.
const ZERO: [u8; SIZE] = [b'0', b'0', b'0'];



/// `NiceU8` provides a quick way to convert a `u8` into a formatted byte
/// string for e.g. printing.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU8;
/// assert_eq!(
///     NiceU8::from(231).as_str(),
///     "231"
/// );
/// ```
///
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NiceU8` has a bunch, including:
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
/// You can instantiate a `NiceU8` with:
///
/// * `From<u8>`
/// * `From<Option<u8>>`
/// * `From<NonZeroU8>`
/// * `From<Option<NonZeroU8>>`
///
/// When converting from a `None`, the result will be equivalent to zero.
pub type NiceU8 = NiceWrapper<SIZE>;

impl From<u8> for NiceU8 {
	#[allow(clippy::cast_lossless)] // Seems less performant.
	#[allow(unsafe_code)]
	fn from(num: u8) -> Self {
		if 99 < num {
			let mut inner = ZERO;
			unsafe { super::write_u8_3(inner.as_mut_ptr(), num as u16); }
			Self {
				inner,
				from: 0,
			}
		}
		else if 9 < num {
			let mut inner = ZERO;
			unsafe {
				std::ptr::copy_nonoverlapping(
					crate::double(num as usize),
					inner.as_mut_ptr().add(1),
					2
				);
			}
			Self {
				inner,
				from: 1,
			}
		}
		else {
			Self {
				inner: [b'0', b'0', num + b'0'],
				from: 2,
			}
		}
	}
}

super::nice_default!(NiceU8, ZERO, SIZE);
super::nice_from_nz!(NiceU8, NonZeroU8);
super::nice_from_opt!(NiceU8);

impl NiceU8 {
	#[must_use]
	#[inline]
	/// # Double Digit Bytes.
	///
	/// This method will return return a byte slice that is *at least* two
	/// bytes long, left padding the value with a zero if its natural length is
	/// shorter. (In other words, this has no effect if the value is >= 10.)
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_bytes2(), b"03");
	/// assert_eq!(dactyl::NiceU8::from(50).as_bytes2(), b"50");
	/// assert_eq!(dactyl::NiceU8::from(113).as_bytes2(), b"113");
	/// ```
	pub fn as_bytes2(&self) -> &[u8] { &self.inner[1.min(self.from)..] }

	#[must_use]
	#[inline]
	/// # Triple Digit Bytes.
	///
	/// This method will return return a byte slice that is *at least* three
	/// bytes long, left padding the value with a zero if its natural length is
	/// shorter. (In other words, this has no effect if the value is >= 100.)
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_bytes3(), b"003");
	/// assert_eq!(dactyl::NiceU8::from(50).as_bytes3(), b"050");
	/// assert_eq!(dactyl::NiceU8::from(113).as_bytes3(), b"113");
	/// ```
	pub const fn as_bytes3(&self) -> &[u8] { &self.inner }

	#[allow(unsafe_code)]
	#[must_use]
	#[inline]
	/// # Double Digit Str.
	///
	/// This method will return return a string slice that is *at least* two
	/// chars long, left padding the value with a zero if its natural length is
	/// shorter. (In other words, this has no effect if the value is >= 10.)
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_str2(), "03");
	/// assert_eq!(dactyl::NiceU8::from(50).as_str2(), "50");
	/// assert_eq!(dactyl::NiceU8::from(113).as_str2(), "113");
	/// ```
	pub fn as_str2(&self) -> &str {
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes2()) }
	}

	#[allow(unsafe_code)]
	#[must_use]
	#[inline]
	/// # Triple Digit Str.
	///
	/// This method will return return a string slice that is *at least* three
	/// chars long, left padding the value with zeroes if its natural length is
	/// shorter. (In other words, this has no effect if the value is >= 100.)
	///
	/// ## Examples
	///
	/// ```
	/// assert_eq!(dactyl::NiceU8::from(3).as_str3(), "003");
	/// assert_eq!(dactyl::NiceU8::from(50).as_str3(), "050");
	/// assert_eq!(dactyl::NiceU8::from(113).as_str3(), "113");
	/// ```
	pub const fn as_str3(&self) -> &str {
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes3()) }
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_u8() {
		assert_eq!(NiceU8::min(), NiceU8::from(0));

		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			assert_eq!(
				NiceU8::from(i).as_str(),
				format!("{}", i),
			);
		}

		// Test the defaults too.
		assert_eq!(NiceU8::empty().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU8::empty().as_str(), "");

		// Test some Option variants.
		let foo: Option<u8> = None;
		assert_eq!(NiceU8::min(), NiceU8::from(foo));
		let foo = Some(13_u8);
		assert_eq!(NiceU8::from(13_u8), NiceU8::from(foo));

		// Check ordering too.
		let one = NiceU8::from(10);
		let two = NiceU8::from(90);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);
	}

	#[test]
	fn t_nice_nonzero_u8() {
		assert_eq!(NiceU8::min(), NiceU8::from(NonZeroU8::new(0)));
		assert_eq!(NiceU8::from(50_u8), NiceU8::from(NonZeroU8::new(50)));
		assert_eq!(NiceU8::from(50_u8), NiceU8::from(NonZeroU8::new(50).unwrap()));

		// Test some Option variants.
		let foo: Option<NonZeroU8> = None;
		assert_eq!(NiceU8::from(foo), NiceU8::min());
		let foo = NonZeroU8::new(13);
		assert_eq!(NiceU8::from(13_u8), NiceU8::from(foo));
	}

	#[test]
	fn t_nice_u8_pad2() {
		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			assert_eq!(
				NiceU8::from(i).as_str2(),
				format!("{:02}", i),
			);
		}

		// Test the default.
		assert_eq!(NiceU8::empty().as_str2(), "00");
	}

	#[test]
	fn t_nice_u8_pad3() {
		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			assert_eq!(
				NiceU8::from(i).as_str3(),
				format!("{:03}", i),
			);
		}

		// Test the default.
		assert_eq!(NiceU8::empty().as_str3(), "000");
	}

	#[test]
	fn t_as() {
		let num = NiceU8::from(253);
		assert_eq!(num.as_str(), String::from(num));
		assert_eq!(num.as_bytes(), Vec::<u8>::from(num));
	}
}
