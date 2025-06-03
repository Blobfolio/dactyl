/*!
# Dactyl: Nice u16.
*/

use crate::{
	Digiter,
	NiceWrapper,
};
use std::num::NonZeroU16;



/// # Total Buffer Size.
///
/// 65535 + one comma = six bytes.
const SIZE: usize = 6;

/// # Digit Indices.
const INDICES: [usize; 5] = [5, 4, 3, 1, 0];

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

super::nice_default!(NiceU16, inner!(b','), SIZE);

impl From<u16> for NiceU16 {
	#[inline]
	fn from(num: u16) -> Self {
		let Some(digits) = Digiter::<u16>::new(num) else { return Self::MIN; };
		let from = INDICES[digits.len() - 1];

		let mut inner = inner!(b',');
		let Ok(indices) = inner.get_disjoint_mut(INDICES) else { unreachable!(); };
		for (d, v) in digits.zip(indices) {
			*v = d;
		}

		Self { inner, from }
	}
}

impl From<NonZeroU16> for NiceU16 {
	#[inline]
	fn from(num: NonZeroU16) -> Self {
		let digits = Digiter(num.get());
		let from = INDICES[digits.len() - 1];

		let mut inner = inner!(b',');
		let Ok(indices) = inner.get_disjoint_mut(INDICES) else { unreachable!(); };
		for (d, v) in digits.zip(indices) {
			*v = d;
		}

		Self { inner, from }
	}
}

impl NiceU16 {
	/// # Minimum Value.
	///
	/// The nice equivalent of `u16::MIN`.
	///
	/// ```
	/// use dactyl::NiceU16;
	///
	/// assert_eq!(
	///     NiceU16::MIN.as_str(),
	///     "0"
	/// );
	///
	/// assert_eq!(
	///     NiceU16::MIN,
	///     NiceU16::from(u16::MIN),
	/// );
	/// ```
	pub const MIN: Self = Self {
		inner: inner!(b','),
		from: SIZE - 1,
	};

	/// # Maximum Value.
	///
	/// The nice equivalent of `u16::MAX`.
	///
	/// ```
	/// use dactyl::NiceU16;
	///
	/// assert_eq!(
	///     NiceU16::MAX.as_str(),
	///     "65,535"
	/// );
	///
	/// assert_eq!(
	///     NiceU16::MAX,
	///     NiceU16::from(u16::MAX),
	/// );
	/// ```
	pub const MAX: Self = Self {
		inner: *b"65,535",
		from: 0,
	};
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

		let mut inner = inner!(sep);
		let Some(digits) = Digiter::<u16>::new(num) else {
			return Self { inner, from: SIZE - 1 };
		};
		let from = INDICES[digits.len() - 1];

		let Ok(indices) = inner.get_disjoint_mut(INDICES) else { unreachable!(); };
		for (d, v) in digits.zip(indices) {
			*v = d;
		}

		Self { inner, from }
	}

	/// # Replace.
	///
	/// Reuse the backing storage behind `self` to hold a new nice number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceU16;
	///
	/// let mut num = NiceU16::from(123_u16);
	/// assert_eq!(num.as_str(), "123");
	///
	/// num.replace(12345);
	/// assert_eq!(num.as_str(), "12,345");
	/// ```
	///
	/// Note that custom separators, if any, are preserved.
	///
	/// ```
	/// use dactyl::NiceU16;
	///
	/// let mut num = NiceU16::with_separator(123_u16, b'_');
	/// assert_eq!(num.as_str(), "123");
	///
	/// num.replace(12345);
	/// assert_eq!(num.as_str(), "12_345");
	/// ```
	pub fn replace(&mut self, num: u16) {
		let Some(digits) = Digiter::<u16>::new(num) else {
			self.from = SIZE - 1;
			self.inner[SIZE - 1] = b'0';
			return;
		};

		self.from = INDICES[digits.len() - 1];

		let Ok(indices) = self.inner.get_disjoint_mut(INDICES) else { unreachable!(); };
		for (d, v) in digits.zip(indices) {
			*v = d;
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_digit_indices() {
		// Find the digit indices.
		let mut idx: Vec<usize> = inner!(b',').into_iter()
			.enumerate()
			.filter_map(|(k, v)|
				if v == b',' { None }
				else { Some(k) }
			)
			.collect();

		// Reverse it to match our constant.
		idx.reverse();

		// Now they should match!
		assert_eq!(INDICES.as_slice(), idx);
	}

	#[test]
	fn t_nice_u16() {
		assert_eq!(NiceU16::default(), NiceU16::from(0_u16));

		let mut last = NiceU16::empty();

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

			// Replacement should yield the same thing.
			assert_ne!(nice, last);
			last.replace(i);
			assert_eq!(nice, last);
		}

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			let set = std::iter::repeat_with(|| rng.u16(..))
				.take(1000)
				.collect::<std::collections::BTreeSet<_>>();
			for i in set {
				let nice = NiceU16::from(i);
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
		}

		// Make sure back to zero works.
		last.replace(0);
		assert_eq!(last.as_str(), "0");

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
