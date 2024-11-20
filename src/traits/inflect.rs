/*!
# Dactyl: Inflection.
*/

use crate::{
	NiceU8,
	NiceU16,
	NiceU32,
	NiceU64,
	NiceWrapper,
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
	///     3283.inflect("book", "books"),
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
pub trait NiceInflection<const S: usize>: Inflection {
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
	/// assert_eq!(
	///     3283.nice_inflect("book", "books").to_string(),
	///     "3,283 books",
	/// );
	///
	/// // The return type implements fmt::Display, so you can do things
	/// // like this too:
	/// assert_eq!(
	///     format!("I have {}!", 3283.nice_inflect("book", "books")),
	///     "I have 3,283 books!",
	/// );
	/// ```
	fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, S>;
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
pub struct NiceInflected<'a, const S: usize> {
	/// # Negative?
	neg: bool,

	/// # The Number.
	nice: NiceWrapper<S>,

	/// # The Inflected Text.
	unit: &'a str,
}

impl<'a, const S: usize> fmt::Display for NiceInflected<'a, S> {
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



/// # Helper: Generate `Inflection` impls.
macro_rules! inflect {
	// Unsigned.
	($ty:ty, $one:literal) => (
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == $one { singular } else { plural }
			}
		}
	);

	// Nonzero.
	($ty:ty, $one:expr) => (
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == $one { singular } else { plural }
			}
		}
	);

	// Signed.
	($ty:ty, $one:literal, $cast:ident) => (
		impl Inflection for $ty {
			#[inline]
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self.$cast() == $one { singular } else { plural }
			}
		}
	);
}

/// # Helper: Generate `Inflection` and `NiceInflection` impls.
macro_rules! inflect_nice {
	// Unsigned.
	($size:literal, $ty:ty, $nice:ty) => (
		impl NiceInflection<$size> for $ty {
			#[inline]
			/// # Inflect a String.
			fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, $size> {
				let nice = <$nice>::from(self);
				let unit = self.inflect(singular, plural);
				NiceInflected { neg: false, nice, unit }
			}
		}
	);

	// Signed.
	($size:literal, $ty:ty, $nice:ty, $cast:ident) => (
		impl NiceInflection<$size> for $ty {
			#[inline]
			/// # Inflect a String.
			fn nice_inflect<'a>(self, singular: &'a str, plural: &'a str) -> NiceInflected<'a, $size> {
				let neg = self < 0;
				let nice = <$nice>::from(self.$cast());
				let unit = self.inflect(singular, plural);
				NiceInflected { neg, nice, unit }
			}
		}
	);

	// Unsigned, both impls.
	($size:literal, $ty:ty, $nice:ty, $one:literal) => (
		inflect!($ty, $one);
		inflect_nice!($size, $ty, $nice);
	);

	// Nonzero, both impls.
	($size:literal, $ty:ty, $nice:ty, $one:expr) => (
		inflect!($ty, $one);
		inflect_nice!($size, $ty, $nice);
	);

	// Signed, both impls.
	($size:literal, $ty:ty, $nice:ty, $one:literal, $cast:ident) => (
		inflect!($ty, $one, $cast);
		inflect_nice!($size, $ty, $nice, $cast);
	);
}

inflect_nice!(3,  u8,           NiceU8,  1);
inflect_nice!(6,  u16,          NiceU16, 1);
inflect_nice!(13, u32,          NiceU32, 1);
inflect_nice!(26, u64,          NiceU64, 1);
inflect_nice!(26, usize,        NiceU64, 1);
inflect_nice!(3,  NonZeroU8,    NiceU8,  Self::MIN);
inflect_nice!(6,  NonZeroU16,   NiceU16, Self::MIN);
inflect_nice!(13, NonZeroU32,   NiceU32, Self::MIN);
inflect_nice!(26, NonZeroU64,   NiceU64, Self::MIN);
inflect_nice!(26, NonZeroUsize, NiceU64, Self::MIN);
inflect_nice!(3,  i8,           NiceU8,  1,         unsigned_abs);
inflect_nice!(6,  i16,          NiceU16, 1,         unsigned_abs);
inflect_nice!(13, i32,          NiceU32, 1,         unsigned_abs);
inflect_nice!(26, i64,          NiceU64, 1,         unsigned_abs);
inflect_nice!(26, isize,        NiceU64, 1,         unsigned_abs);

// These aren't nice, but we can still do basic inflection.
inflect!(u128, 1);
inflect!(i128, 1, unsigned_abs);
inflect!(NonZeroU128, Self::MIN);

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
