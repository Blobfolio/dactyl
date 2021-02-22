/*!
# Dactyl: Nice u8.
*/



/// # Total Buffer Size.
const SIZE: usize = 3;



#[derive(Debug, Clone, Copy, Hash, PartialEq)]
/// `NiceU8` provides a quick way to convert a `u8` into a formatted byte
/// string for e.g. printing.
///
/// That's it!
///
/// ## Examples
///
/// ```no_run
/// use dactyl::NiceU8;
/// assert_eq!(
///     NiceU8::from(231).as_str(),
///     "231"
/// );
pub struct NiceU8 {
	inner: [u8; SIZE],
	from: usize,
}

crate::impl_nice_int!(NiceU8);

impl Default for NiceU8 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [48, 48, 48],
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
	/// ```no_run
	/// assert_eq(dactyl::NiceU8::from(3).as_bytes2(), b"02");
	/// assert_eq(dactyl::NiceU8::from(50).as_bytes2(), b"50");
	/// assert_eq(dactyl::NiceU8::from(113).as_bytes2(), b"113");
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
	/// ```no_run
	/// assert_eq(dactyl::NiceU8::from(3).as_bytes3(), b"002");
	/// assert_eq(dactyl::NiceU8::from(50).as_bytes3(), b"050");
	/// assert_eq(dactyl::NiceU8::from(113).as_bytes3(), b"113");
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
	/// ```no_run
	/// assert_eq(dactyl::NiceU8::from(3).as_str2(), "02");
	/// assert_eq(dactyl::NiceU8::from(50).as_str2(), "50");
	/// assert_eq(dactyl::NiceU8::from(113).as_str2(), "113");
	/// ```
	pub fn as_str2(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes2()) }
	}

	#[allow(clippy::missing_const_for_fn)] // Doesn't work with unsafe.
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
	/// ```no_run
	/// assert_eq(dactyl::NiceU8::from(3).as_str3(), "002");
	/// assert_eq(dactyl::NiceU8::from(50).as_str3(), "050");
	/// assert_eq(dactyl::NiceU8::from(113).as_str3(), "113");
	/// ```
	pub fn as_str3(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes3()) }
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_u8() {
		// Strings come from bytes, so this implicitly tests both.
		for i in 0..=u8::MAX {
			assert_eq!(
				NiceU8::from(i).as_str(),
				format!("{}", i),
			);
		}

		// Test the defaults too.
		let empty: &[u8] = &[];
		assert_eq!(NiceU8::default().as_bytes(), empty);
		assert_eq!(NiceU8::default().as_str(), "");
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
