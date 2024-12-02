/*!
# Dactyl: Nice u64.
*/

use crate::NiceWrapper;
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
///
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NiceU64` has a bunch, including:
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
/// You can instantiate a `NiceU64` with:
///
/// * `From<u64>`
/// * `From<Option<u64>>`
/// * `From<NonZeroU64>`
/// * `From<Option<NonZeroU64>>`
/// * `From<usize>`
/// * `From<Option<usize>>`
/// * `From<NonZeroUsize>`
/// * `From<Option<NonZeroUsize>>`
///
/// When converting from a `None`, the result will be equivalent to zero.
///
/// For targets with 128-bit pointers, `usize` values cannot exceed [`u64::MAX`]
/// or a panic will ensue.
pub type NiceU64 = NiceWrapper<SIZE>;

impl From<usize> for NiceU64 {
	fn from(num: usize) -> Self { Self::from(num as u64) }
}

super::nice_default!(NiceU64, inner!(b','), SIZE);
super::nice_from_nz!(NiceU64, NonZeroU64, NonZeroUsize);
super::nice_parse!(NiceU64, u64);

impl NiceU64 {
	/// # Minimum Value.
	///
	/// The nice equivalent of `u64::MIN`.
	///
	/// ```
	/// use dactyl::NiceU64;
	///
	/// assert_eq!(
	///     NiceU64::MIN.as_str(),
	///     "0"
	/// );
	///
	/// assert_eq!(
	///     NiceU64::MIN,
	///     NiceU64::from(u64::MIN),
	/// );
	/// ```
	pub const MIN: Self = Self {
		inner: inner!(b','),
		from: SIZE - 1,
	};

	/// # Maximum Value.
	///
	/// The nice equivalent of `u64::MAX`.
	///
	/// ```
	/// use dactyl::NiceU64;
	///
	/// assert_eq!(
	///     NiceU64::MAX.as_str(),
	///     "18,446,744,073,709,551,615"
	/// );
	///
	/// assert_eq!(
	///     NiceU64::MAX,
	///     NiceU64::from(u64::MAX),
	/// );
	/// ```
	pub const MAX: Self = Self {
		inner: *b"18,446,744,073,709,551,615",
		from: 0,
	};
}

impl NiceU64 {
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
}



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u64() {
		#[cfg(not(miri))]
		const SAMPLE_SIZE: usize = 1_000_000;

		#[cfg(miri)]
		const SAMPLE_SIZE: usize = 500; // Miri runs way too slow for a million tests.

		// Check the min and max.
		assert_eq!(NiceU64::from(0_u64).as_str(), "0");
		assert_eq!(NiceU64::default(), NiceU64::from(0_u64));
		assert_eq!(NiceU64::default(), NiceU64::from(0_usize));
		assert_eq!(
			NiceU64::from(u64::MAX).as_str(),
			u64::MAX.to_formatted_string(&Locale::en),
		);

		// Test the defaults too.
		assert_eq!(NiceU64::empty().as_bytes(), <&[u8]>::default());
		assert_eq!(NiceU64::empty().as_str(), "");
		assert!(NiceU64::empty().is_empty());

		// Test some Option variants.
		let foo: Option<u64> = None;
		assert_eq!(NiceU64::default(), NiceU64::from(foo));
		let foo = Some(13_u64);
		assert_eq!(NiceU64::from(13_u64), NiceU64::from(foo));

		let foo: Option<usize> = None;
		assert_eq!(NiceU64::default(), NiceU64::from(foo));
		let foo = Some(13_usize);
		assert_eq!(NiceU64::from(13_usize), NiceU64::from(foo));

		// Check ordering too.
		let one = NiceU64::from(10_u64);
		let two = NiceU64::from(90_u64);
		assert_eq!(one.cmp(&two), std::cmp::Ordering::Less);
		assert_eq!(one.cmp(&one), std::cmp::Ordering::Equal);
		assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);

		// Check a subset of everything else.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(..)).take(SAMPLE_SIZE) {
			let nice = NiceU64::from(i);
			assert_eq!(
				nice.as_str(),
				i.to_formatted_string(&Locale::en),
			);
			assert_eq!(nice.len(), nice.as_str().len());
			assert_eq!(nice.len(), nice.as_bytes().len());
			assert!(! nice.is_empty());
		}
	}

	#[test]
	fn t_nice_nonzero_u64() {
		assert_eq!(NiceU64::default(), NiceU64::from(NonZeroU64::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50).unwrap()));

		assert_eq!(NiceU64::default(), NiceU64::from(NonZeroUsize::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50).unwrap()));

		// Test some Option variants.
		let foo: Option<NonZeroU64> = None;
		assert_eq!(NiceU64::from(foo), NiceU64::default());
		let foo = NonZeroU64::new(13);
		assert_eq!(NiceU64::from(13_u64), NiceU64::from(foo));

		let foo: Option<NonZeroUsize> = None;
		assert_eq!(NiceU64::from(foo), NiceU64::default());
		let foo = NonZeroUsize::new(13);
		assert_eq!(NiceU64::from(13_usize), NiceU64::from(foo));
	}

	#[test]
	fn t_as() {
		let num = NiceU64::from(12_345_678_912_345_u64);
		assert_eq!(num.as_str(), String::from(num));
		assert_eq!(num.as_bytes(), Vec::<u8>::from(num));
	}
}
