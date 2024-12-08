/*!
# Dactyl: Nice u32.
*/

use crate::NiceWrapper;
use std::num::NonZeroU32;



/// # Total Buffer Size.
///
/// 4294967295 + three commas = thirteen bytes.
const SIZE: usize = 13;

/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0']);
}



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
///
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NiceU32` has a bunch, including:
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
/// You can instantiate a `NiceU32` with:
///
/// * `From<u32>`
/// * `From<Option<u32>>`
/// * `From<NonZeroU32>`
/// * `From<Option<NonZeroU32>>`
///
/// When converting from a `None`, the result will be equivalent to zero.
pub type NiceU32 = NiceWrapper<SIZE>;

super::nice_default!(NiceU32, inner!(b','), SIZE);
super::nice_from_nz!(NiceU32, NonZeroU32);
super::nice_parse!(NiceU32, u32);

impl NiceU32 {
	/// # Minimum Value.
	///
	/// The nice equivalent of `u32::MIN`.
	///
	/// ```
	/// use dactyl::NiceU32;
	///
	/// assert_eq!(
	///     NiceU32::MIN.as_str(),
	///     "0"
	/// );
	///
	/// assert_eq!(
	///     NiceU32::MIN,
	///     NiceU32::from(u32::MIN),
	/// );
	/// ```
	pub const MIN: Self = Self {
		inner: inner!(b','),
		from: SIZE - 1,
	};

	/// # Maximum Value.
	///
	/// The nice equivalent of `u32::MAX`.
	///
	/// ```
	/// use dactyl::NiceU32;
	///
	/// assert_eq!(
	///     NiceU32::MAX.as_str(),
	///     "4,294,967,295"
	/// );
	///
	/// assert_eq!(
	///     NiceU32::MAX,
	///     NiceU32::from(u32::MAX),
	/// );
	/// ```
	pub const MAX: Self = Self {
		inner: *b"4,294,967,295",
		from: 0,
	};
}

impl NiceU32 {
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

	/// # Replace.
	///
	/// Reuse the backing storage behind `self` to hold a new nice number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceU32;
	///
	/// let mut num = NiceU32::from(3141592653_u32);
	/// assert_eq!(num.as_str(), "3,141,592,653");
	///
	/// num.replace(12345);
	/// assert_eq!(num.as_str(), "12,345");
	/// ```
	///
	/// Note that custom separators, if any, are preserved.
	///
	/// ```
	/// use dactyl::NiceU32;
	///
	/// let mut num = NiceU32::with_separator(3141592653_u32, b'_');
	/// assert_eq!(num.as_str(), "3_141_592_653");
	///
	/// num.replace(12345);
	/// assert_eq!(num.as_str(), "12_345");
	/// ```
	pub fn replace(&mut self, num: u32) {
		self.from = SIZE;
		self.parse(num);
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u32() {
		#[cfg(not(miri))]
		const SAMPLE_SIZE: usize = 1_000_000;

		#[cfg(miri)]
		const SAMPLE_SIZE: usize = 500; // Miri runs way too slow for a million tests.

		// Check the min and max.
		assert_eq!(NiceU32::from(0).as_str(), "0");
		assert_eq!(NiceU32::default(), NiceU32::from(0));
		assert_eq!(
			NiceU32::from(u32::MAX).as_str(),
			u32::MAX.to_formatted_string(&Locale::en),
		);

		// Test some Option variants.
		let foo: Option<u32> = None;
		assert_eq!(NiceU32::default(), NiceU32::from(foo));
		let foo = Some(13_u32);
		assert_eq!(NiceU32::from(13_u32), NiceU32::from(foo));

		// Test the defaults too.
		assert_eq!(NiceU32::empty().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU32::empty().as_str(), "");
		assert!(NiceU32::empty().is_empty());

		// Check ordering too.
		let one = NiceU32::from(10_u32);
		let two = NiceU32::from(90_u32);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);

		// Check a subset of everything else.
		let mut last = NiceU32::empty();
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(..)).take(SAMPLE_SIZE) {
			let nice = NiceU32::from(i);
			assert_eq!(
				nice.as_str(),
				i.to_formatted_string(&Locale::en),
			);
			assert_eq!(nice.len(), nice.as_str().len());
			assert_eq!(nice.len(), nice.as_bytes().len());
			assert!(! nice.is_empty());

			// Replacement should yield the same thing.
			assert_ne!(nice, last);
			last.replace(i);
			assert_eq!(nice, last);
		}

		// Make sure back to zero works.
		last.replace(0);
		assert_eq!(last.as_str(), "0");
	}


	#[test]
	fn t_nice_nonzero_u32() {
		assert_eq!(NiceU32::default(), NiceU32::from(NonZeroU32::new(0)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50).unwrap()));

		// Test some Option variants.
		let foo: Option<NonZeroU32> = None;
		assert_eq!(NiceU32::from(foo), NiceU32::default());
		let foo = NonZeroU32::new(13);
		assert_eq!(NiceU32::from(13_u32), NiceU32::from(foo));
	}

	#[test]
	fn t_as() {
		let num = NiceU32::from(12_345_678_u32);
		assert_eq!(num.as_str(), String::from(num));
		assert_eq!(num.as_bytes(), Vec::<u8>::from(num));
	}
}
