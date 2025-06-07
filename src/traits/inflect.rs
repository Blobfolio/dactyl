/*!
# Dactyl: Inflection.
*/

use crate::{
	NiceU8,
	NiceU16,
	NiceU32,
	NiceU64,
};
use std::{
	fmt,
	num::{
		NonZeroU8,
		NonZeroU16,
		NonZeroU32,
		NonZeroU64,
		NonZeroUsize,
		NonZeroU128,
	},
};



/// # Inflection.
///
/// This trait gives you a way to choose between singular and plural versions
/// of a string based on the value of `self`. (If the value is `1` or `-1`, the
/// singular version is chosen; everything else is plural.)
///
/// This is implemented for `i/u/NonZeroU 8–128`, `f32`, and `f64`.
pub trait Inflection: Sized + Copy + PartialEq {
	/// # Inflect a String.
	///
	/// Returns one of two strings depending on if `self.abs() == 1`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::Inflection;
	/// assert_eq!(
	///     3283_u16.inflect("book", "books"),
	///     "books"
	/// );
	/// ```
	fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str;
}

/// # Nice Inflection.
///
/// This extends the `Inflection` trait for types which can be represented as
/// one of the `NiceU*` types, and their signed equivalents (minus signs will
/// be prepended as necessary), i.e. `i/u/NonZeroU 8–64`.
pub trait NiceInflection<T>: Inflection {
	/// # Inflect a String (Prefixed w/ Value)
	///
	/// This is like [`Inflection::inflect`], but prefixes the output with a
	/// nicely-formatted representation of the numeric value.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::NiceInflection;
	///
	/// // The return type implements fmt::Display, so you can chuck it
	/// // straight into a formatter pattern like this:
	/// assert_eq!(
	///     format!("I have {}!", 3283_u16.nice_inflect("book", "books")),
	///     "I have 3,283 books!",
	/// );
	///
	/// // Alternatively, you can save it to a variable to access the
	/// // inner parts.
	/// let nice = 3283_u16.nice_inflect("book", "books");
	/// assert!(! nice.is_negative()); // Would be true for e.g. -5.
	/// assert_eq!(
	///     nice.nice().as_str(), // Note: always positive.
	///     "3,283",
	/// );
	/// assert_eq!(nice.unit(), "books");
	/// ```
	fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, T>;
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Nice Inflection Wrapper.
///
/// This struct serves as the return type for [`NiceInflection::nice_inflect`].
/// It implements [`Display`](fmt::Display) so can be chucked straight into a
/// formatting pattern or converted to a string via `to_string()`.
///
/// ## Examples
///
/// ```
/// use dactyl::traits::NiceInflection;
///
/// assert_eq!(
///     format!(
///         "I have eaten {} and {}!",
///          1001.nice_inflect("hotdog", "hotdogs"),
///          1.nice_inflect("hamburger", "hamburger"),
///     ),
///     "I have eaten 1,001 hotdogs and 1 hamburger!",
/// );
/// ```
pub struct NiceInflected<'a, T> {
	/// # Negative?
	neg: bool,

	/// # The Number.
	nice: T,

	/// # The Inflected Text.
	unit: &'a str,
}

impl<T: Copy> NiceInflected<'_, T> {
	/// Is Negative?
	///
	/// Returns `true` if the original number was negative.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::NiceInflection;
	///
	/// let dogs = 8_i32;
	/// let nice_dogs = dogs.nice_inflect("dog", "dogs");
	/// assert!(! nice_dogs.is_negative());
	///
	/// let cats = -13_i32;
	/// let nice_cats = cats.nice_inflect("cat", "cats");
	/// assert!(nice_cats.is_negative());
	/// ```
	pub const fn is_negative(&self) -> bool { self.neg }

	/// Nice Number.
	///
	/// Returns the nicely-formatted number.
	///
	/// Note: the nice value does not include the signing bit; if the raw value
	/// was negative, you'll need to prepend an "-" before printing, etc.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::NiceInflection;
	///
	/// let nice = 3000_u16.nice_inflect("dog", "dogs");
	/// assert_eq!(nice.nice().as_str(), "3,000");
	/// ```
	pub const fn nice(&self) -> T { self.nice }

	/// Inflected Unit.
	///
	/// Returns the inflected unit.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::NiceInflection;
	///
	/// let nice = 13_i32.nice_inflect("apple", "apples");
	/// assert_eq!(nice.unit(), "apples");
	/// ```
	pub const fn unit(&self) -> &str { self.unit }
}



/// # Helper: Inflection.
macro_rules! inflect {
	// Signed.
	(@signed $($ty:ty)+) => ($(
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self.unsigned_abs() == 1 { singular } else { plural }
			}
		}
	)+);

	// Unsigned.
	(@unsigned $($ty:ty)+) => ($(
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == 1 { singular } else { plural }
			}
		}
	)+);

	// Unsigned/non-zero.
	(@nonzero $($ty:ty)+) => ($(
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == Self::MIN { singular } else { plural }
			}
		}
	)+);
}

inflect!(@unsigned u8        u16        u32        u64        u128        usize);
inflect!(@nonzero  NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);
inflect!(@signed   i8        i16        i32        i64        i128        isize);

impl Inflection for f32 {
	#[inline]
	/// # Inflect a String.
	fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
		if self.abs().eq(&1.0) { singular } else { plural }
	}
}

impl Inflection for f64 {
	#[inline]
	/// # Inflect a String.
	fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
		if self.abs().eq(&1.0) { singular } else { plural }
	}
}



/// # Helper: Nice Inflection.
macro_rules! inflect_nice {
	// Signed types.
	(@signed $nice:ty: $($ty:ty)+) => ($(
		impl NiceInflection<$nice> for $ty {
			#[inline]
			/// # Inflect a String.
			fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, $nice> {
				let neg = self < 0;
				let nice = <$nice>::from(self.unsigned_abs());
				let unit = self.inflect(singular, plural);
				NiceInflected { neg, nice, unit }
			}
		}
	)+);

	// Default implementations.
	($($nice:ty)+) => ($(
		impl fmt::Display for NiceInflected<'_, $nice> {
			#[inline]
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				// Add a minus sign to the start if negative.
				if self.neg { f.write_str("-")?; }

				// Print the number.
				f.write_str(self.nice.as_str())?;

				// Add a space.
				f.write_str(" ")?;

				// And print the text value.
				f.write_str(self.unit)
			}
		}

		impl NiceInflected<'_, $nice> {
			/// # Length.
			///
			/// Return the length of the string.
			///
			/// ## Examples
			///
			/// ```
			/// use dactyl::traits::NiceInflection;
			///
			/// let dogs = 8_i32;
			/// let nice_dogs = dogs.nice_inflect("dog", "dogs");
			/// assert_eq!(nice_dogs.len(), 6); // "8 dogs"
			///
			/// let cats = -13_i32;
			/// let nice_cats = cats.nice_inflect("cat", "cats");
			/// assert_eq!(nice_cats.len(), 8); // "-13 cats"
			/// ```
			pub const fn len(&self) -> usize {
				self.neg as usize + self.nice.len() + 1 + self.unit.len()
			}
		}

		impl<T: Inflection> NiceInflection<$nice> for T
		where $nice: From<T> {
			#[inline]
			/// # Inflect a String.
			fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, $nice> {
				let nice = <$nice>::from(self);
				let unit = self.inflect(singular, plural);
				NiceInflected { neg: false, nice, unit }
			}
		}
	)+);
}

inflect_nice!(NiceU8 NiceU16 NiceU32 NiceU64);
inflect_nice!(@signed NiceU8:  i8);
inflect_nice!(@signed NiceU16: i16);
inflect_nice!(@signed NiceU32: i32);
inflect_nice!(@signed NiceU64: i64 isize);




#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 250; // Miri runs way too slow for a million tests.

	macro_rules! t_inflect {
		($num:expr, $str:literal) => (
			assert_eq!($num.inflect("book", "books"), $str, "{}.inflect()", $num);
		);
	}

	macro_rules! t_nice_inflect {
		($num:expr, $str:literal) => (
			t_inflect!($num, $str);
			assert_eq!(
				$num.nice_inflect("book", "books").to_string(),
				format!(concat!("{} ", $str), $num.to_formatted_string(&Locale::en)),
				"{}.nice_inflect()", $num
			);
		);
	}

	macro_rules! t_nice_basics {
		($ty:ty, $nz:ty, $i:ty) => (
			let num: $ty = 0;
			t_nice_inflect!(num, "books");

			let num: $ty = 1;
			t_nice_inflect!(num, "book");
			t_nice_inflect!(<$nz>::new(num).unwrap(), "book");

			let num: $i = 0;
			t_nice_inflect!(num, "books");

			let num: $i = 1;
			t_nice_inflect!(num, "book");

			let num: $i = -1;
			t_nice_inflect!(num, "book");
		);
	}

	#[test]
	fn t_u8() {
		t_nice_basics!(u8, NonZeroU8, i8);

		for i in 2..=u8::MAX {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroU8::new(i).unwrap(), "books");
		}
		for i in 2..=i8::MAX { t_nice_inflect!(i, "books"); }
		for i in i8::MIN..-1 { t_nice_inflect!(i, "books"); }
	}

	#[cfg(not(miri))]
	#[test]
	fn t_u16() {
		t_nice_basics!(u16, NonZeroU16, i16);

		for i in 2..=u16::MAX {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroU16::new(i).unwrap(), "books");
		}
		for i in 2..=i16::MAX { t_nice_inflect!(i, "books"); }
		for i in i16::MIN..-1 { t_nice_inflect!(i, "books"); }
	}

	#[cfg(miri)]
	#[test]
	fn t_u16() {
		t_nice_basics!(u16, NonZeroU16, i16);

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u16(2..=u16::MAX)).take(SAMPLE_SIZE) {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroU16::new(i).unwrap(), "books");
		}
		for i in std::iter::repeat_with(|| rng.i16(i16::MIN..-1)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
		for i in std::iter::repeat_with(|| rng.i16(2..i16::MAX)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
	}

	#[test]
	fn t_u32() {
		t_nice_basics!(u32, NonZeroU32, i32);

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(2..=u32::MAX)).take(SAMPLE_SIZE) {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroU32::new(i).unwrap(), "books");
		}
		for i in std::iter::repeat_with(|| rng.i32(i32::MIN..-1)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
		for i in std::iter::repeat_with(|| rng.i32(2..i32::MAX)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
	}

	#[test]
	fn t_u64() {
		t_nice_basics!(u64, NonZeroU64, i64);

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(2..=u64::MAX)).take(SAMPLE_SIZE) {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroU64::new(i).unwrap(), "books");
		}
		for i in std::iter::repeat_with(|| rng.i64(i64::MIN..-1)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
		for i in std::iter::repeat_with(|| rng.i64(2..i64::MAX)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
	}

	#[test]
	fn t_usize() {
		t_nice_basics!(usize, NonZeroUsize, isize);

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.usize(2..=usize::MAX)).take(SAMPLE_SIZE) {
			t_nice_inflect!(i, "books");
			t_nice_inflect!(NonZeroUsize::new(i).unwrap(), "books");
		}
		for i in std::iter::repeat_with(|| rng.isize(isize::MIN..-1)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
		for i in std::iter::repeat_with(|| rng.isize(2..isize::MAX)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_nice_inflect!(i, "books");
		}
	}

	#[test]
	fn t_u128() {
		t_inflect!(0_u128, "books");
		t_inflect!(1_u128, "book");
		t_inflect!(NonZeroU128::new(1).unwrap(), "book");

		t_inflect!((-1_i128), "book");
		t_inflect!(0_i128, "books");
		t_inflect!(1_i128, "book");

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u128(2..=u128::MAX)).take(SAMPLE_SIZE) {
			t_inflect!(i, "books");
			t_inflect!(NonZeroU128::new(i).unwrap(), "books");
		}
		for i in std::iter::repeat_with(|| rng.i128(i128::MIN..-1)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_inflect!(i, "books");
		}
		for i in std::iter::repeat_with(|| rng.i128(2..i128::MAX)).take(SAMPLE_SIZE.wrapping_div(2)) {
			t_inflect!(i, "books");
		}
	}

	#[test]
	fn t_f32() {
		t_inflect!(0_f32, "books");
		t_inflect!(1_f32, "book");
		t_inflect!((-1_f32), "book");
		t_inflect!(1.05_f32, "books");
	}

	#[test]
	fn t_f64() {
		t_inflect!(0_f64, "books");
		t_inflect!(1_f64, "book");
		t_inflect!((-1_f64), "book");
		t_inflect!(1.05_f64, "books");
	}
}
