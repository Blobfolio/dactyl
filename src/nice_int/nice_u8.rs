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
	fn from(num: u8) -> Self {
		if 99 < num {
			Self {
				inner: crate::triple(num as usize),
				from: 0,
			}
		}
		else {
			let [b, c] = crate::double(num as usize);
			Self {
				inner: [b'0', b, c],
				from: if b == b'0' { 2 } else { 1 },
			}
		}
	}
}

super::nice_default!(NiceU8, ZERO, SIZE);
super::nice_from_nz!(NiceU8, NonZeroU8);

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
	pub const fn as_bytes2(&self) -> &[u8] {
		if self.from == 0 { &self.inner }
		else {
			let [ _, rest @ .. ] = &self.inner;
			rest
		}
	}

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

	#[allow(unsafe_code)] // Content is ASCII.
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
	pub const fn as_str2(&self) -> &str {
		debug_assert!(
			(self.from != 0 || self.inner[0].is_ascii_digit()) &&
			self.inner[1].is_ascii_digit() &&
			self.inner[2].is_ascii_digit(),
			"Bug: NiceU8 is not ASCII."
		);
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes2()) }
	}

	#[allow(unsafe_code)] // Content is ASCII.
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
		debug_assert!(
			self.inner[0].is_ascii_digit() &&
			self.inner[1].is_ascii_digit() &&
			self.inner[2].is_ascii_digit(),
			"Bug: NiceU8 is not ASCII."
		);
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes3()) }
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_u8() {
		assert_eq!(NiceU8::default(), NiceU8::from(0_u8));

		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			let nice = NiceU8::from(i);
			assert_eq!(
				nice.as_str(),
				format!("{i}"),
			);
			assert_eq!(nice.len(), nice.as_str().len());
			assert_eq!(nice.len(), nice.as_bytes().len());
			assert!(! nice.is_empty());
		}

		// Test the defaults too.
		assert_eq!(NiceU8::empty().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU8::empty().as_str(), "");
		assert!(NiceU8::empty().is_empty());

		// Test some Option variants.
		let foo: Option<u8> = None;
		assert_eq!(NiceU8::default(), NiceU8::from(foo));
		let foo = Some(13_u8);
		assert_eq!(NiceU8::from(13_u8), NiceU8::from(foo));

		// Check ordering too.
		let one = NiceU8::from(10_u8);
		let two = NiceU8::from(90_u8);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);
	}

	#[test]
	fn t_nice_nonzero_u8() {
		assert_eq!(NiceU8::default(), NiceU8::from(NonZeroU8::new(0)));
		assert_eq!(NiceU8::from(50_u8), NiceU8::from(NonZeroU8::new(50)));
		assert_eq!(NiceU8::from(50_u8), NiceU8::from(NonZeroU8::new(50).unwrap()));

		// Test some Option variants.
		let foo: Option<NonZeroU8> = None;
		assert_eq!(NiceU8::from(foo), NiceU8::default());
		let foo = NonZeroU8::new(13);
		assert_eq!(NiceU8::from(13_u8), NiceU8::from(foo));
	}

	#[test]
	fn t_nice_u8_pad2() {
		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			assert_eq!(
				NiceU8::from(i).as_str2(),
				format!("{i:02}"),
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
				format!("{i:03}"),
			);
		}

		// Test the default.
		assert_eq!(NiceU8::empty().as_str3(), "000");
	}

	#[test]
	fn t_as() {
		let num = NiceU8::from(253_u8);
		assert_eq!(num.as_str(), String::from(num));
		assert_eq!(num.as_bytes(), Vec::<u8>::from(num));
	}
}
