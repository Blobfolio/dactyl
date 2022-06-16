/*!
# Dactyl: Nice u64.
*/

use std::num::{
	NonZeroU64,
	NonZeroUsize,
};



/// # Total Buffer Size.
///
/// 18446744073709551615 + six commas = 26 bytes.
const SIZE: usize = 26;



/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0']);
}



#[derive(Debug, Clone, Copy)]
/// `NiceU64` provides a quick way to convert a `u64` into a formatted byte
/// string for e.g. printing. Commas are added for every thousand.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU64;
/// assert_eq!(
///     NiceU64::from(33231_u64).as_str(),
///     "33,231"
/// );
/// ```
pub struct NiceU64 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU64 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: inner!(b','),
			from: SIZE,
		}
	}
}

impl From<usize> for NiceU64 {
	#[allow(clippy::cast_possible_truncation)] // It fits.
	fn from(num: usize) -> Self {
		#[cfg(target_pointer_width = "128")]
		assert!(num <= 18_446_744_073_709_551_615);

		Self::from(num as u64)
	}
}

impl From<u64> for NiceU64 {
	fn from(num: u64) -> Self {
		let mut out = Self::default();
		out.parse(num);
		out
	}
}

impl NiceU64 {
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
	/// If you're good with commas, just use [`NiceU64::from`] instead.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceU64;
	///
	/// let num = NiceU64::from(3141592653589793238_u64);
	/// assert_eq!(num.as_str(), "3,141,592,653,589,793,238");
	///
	/// let num = NiceU64::with_separator(3141592653589793238_u64, b'_');
	/// assert_eq!(num.as_str(), "3_141_592_653_589_793_238");
	/// ```
	///
	/// ## Panics
	///
	/// This method will panic if the separator is invalid ASCII.
	pub fn with_separator(num: u64, sep: u8) -> Self {
		assert!(sep.is_ascii(), "Invalid separator.");
		let mut out = Self {
			inner: inner!(sep),
			from: SIZE,
		};
		out.parse(num);
		out
	}

	#[allow(clippy::cast_possible_truncation)] // Usize casting never exceeds 100; u8 casting never exceeds 9.
	#[allow(unsafe_code)]
	/// # Parse.
	///
	/// This handles the actual crunching.
	fn parse(&mut self, mut num: u64) {
		let ptr = self.inner.as_mut_ptr();

		while 999 < num {
			let (div, rem) = crate::div_mod(num, 1000);
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
super::impl_nice_nonzero_int!(NiceU64: NonZeroU64, NonZeroUsize);
super::impl_nice_int!(NiceU64);



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u64() {
		#[cfg(not(miri))]
		const SAMPLE_SIZE: usize = 1_000_000;

		#[cfg(miri)]
		const SAMPLE_SIZE: usize = 1000; // Miri runs way too slow for a million tests.

		// Check the min and max.
		assert_eq!(NiceU64::from(0_u64).as_str(), "0");
		assert_eq!(NiceU64::min(), NiceU64::from(0_u64));
		assert_eq!(NiceU64::min(), NiceU64::from(0_usize));
		assert_eq!(
			NiceU64::from(u64::MAX).as_str(),
			u64::MAX.to_formatted_string(&Locale::en),
		);

		// Test the defaults too.
		assert_eq!(NiceU64::default().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU64::default().as_str(), "");

		// Check ordering too.
		let one = NiceU64::from(10_u64);
		let two = NiceU64::from(90_u64);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);

		// Check a subset of everything else.
		let rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(..)).take(SAMPLE_SIZE) {
			assert_eq!(
				NiceU64::from(i).as_str(),
				i.to_formatted_string(&Locale::en),
			);
		}
	}

	#[test]
	fn t_nice_nonzero_u64() {
		assert_eq!(NiceU64::min(), NiceU64::from(NonZeroU64::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50).unwrap()));

		assert_eq!(NiceU64::min(), NiceU64::from(NonZeroUsize::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50).unwrap()));
	}

	#[test]
	fn t_as() {
		let num = NiceU64::from(12_345_678_912_345_u64);
		assert_eq!(num.as_str(), num.as_string());
		assert_eq!(num.as_bytes(), num.as_vec());
	}
}
