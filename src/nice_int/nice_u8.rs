/*!
# Dactyl: Nice u8.
*/

use std::num::NonZeroU8;



/// # Total Buffer Size.
const SIZE: usize = 3;



#[derive(Debug, Clone, Copy)]
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
pub struct NiceU8 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU8 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [b'0', b'0', b'0'],
			from: SIZE,
		}
	}
}

impl From<u8> for NiceU8 {
	fn from(num: u8) -> Self {
		let mut out = Self::default();

		if num >= 100 {
			out.from -= 3;
			unsafe { super::write_u8_3(out.inner.as_mut_ptr().add(out.from), usize::from(num)); }
		}
		else if num >= 10 {
			out.from -= 2;
			unsafe { super::write_u8_2(out.inner.as_mut_ptr().add(out.from), usize::from(num)); }
		}
		else {
			out.from -= 1;
			unsafe { super::write_u8_1(out.inner.as_mut_ptr().add(out.from), usize::from(num)); }
		}

		out
	}
}

// A few Macro traits.
super::impl_nice_nonzero_int!(NiceU8: NonZeroU8);
super::impl_nice_int!(NiceU8);

impl NiceU8 {
	#[must_use]
	#[inline]
	/// # Min.
	///
	/// This is equivalent to zero.
	pub const fn min() -> Self {
		Self {
			inner: [b'0', b'0', b'0'],
			from: SIZE - 1,
		}
	}

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
		unsafe { std::str::from_utf8_unchecked(self.as_bytes2()) }
	}

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
		assert_eq!(NiceU8::default().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU8::default().as_str(), "");

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
		assert_eq!(NiceU8::default().as_str2(), "00");
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
		assert_eq!(NiceU8::default().as_str3(), "000");
	}
}
