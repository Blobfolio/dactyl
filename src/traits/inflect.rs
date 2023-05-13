/*!
# Dactyl: Inflection.
*/

use crate::{
	NiceU8,
	NiceU16,
	NiceU32,
	NiceU64,
};
use std::num::{
	NonZeroU8,
	NonZeroU16,
	NonZeroU32,
	NonZeroU64,
	NonZeroUsize,
	NonZeroU128,
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
pub trait NiceInflection: Inflection {
	/// # Inflect a String (Prefixed w/ Value)
	///
	/// This is like [`Inflection::inflect`], but prefixes the output with a
	/// nicely-formatted representation of the numeric value.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::traits::NiceInflection;
	/// assert_eq!(
	///     3283.nice_inflect("book", "books"),
	///     "3,283 books"
	/// );
	/// ```
	fn nice_inflect<S>(self, singular: S, plural: S) -> String
	where S: AsRef<str>;
}



/// # Helper: Generate `Inflection` impls.
macro_rules! inflect {
	// Unsigned.
	($ty:ty, $one:literal) => (
		impl Inflection for $ty {
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == $one { singular } else { plural }
			}
		}
	);

	// Nonzero.
	($ty:ty, $one:expr) => (
		impl Inflection for $ty {
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self == $one { singular } else { plural }
			}
		}
	);

	// Signed.
	($ty:ty, $one:literal, $cast:ident) => (
		impl Inflection for $ty {
			fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
				if self.$cast() == $one { singular } else { plural }
			}
		}
	);
}

/// # Helper: Generate `Inflection` and `NiceInflection` impls.
macro_rules! inflect_nice {
	// Unsigned.
	($ty:ty, $nice:ty) => (
		impl NiceInflection for $ty {
			/// # Inflect a String.
			fn nice_inflect<S>(self, singular: S, plural: S) -> String
			where S: AsRef<str> {
				[
					<$nice>::from(self).as_str(),
					" ",
					self.inflect(singular.as_ref(), plural.as_ref()),
				].concat()
			}
		}
	);

	// Signed.
	($ty:ty, $nice:ty, $cast:ident) => (
		impl NiceInflection for $ty {
			/// # Inflect a String.
			fn nice_inflect<S>(self, singular: S, plural: S) -> String
			where S: AsRef<str> {
				[
					if self < 0 { "-" } else { "" },
					<$nice>::from(self.$cast()).as_str(),
					" ",
					self.inflect(singular.as_ref(), plural.as_ref()),
				].concat()
			}
		}
	);

	// Unsigned, both impls.
	($ty:ty, $nice:ty, $one:literal) => (
		inflect!($ty, $one);
		inflect_nice!($ty, $nice);
	);

	// Nonzero, both impls.
	($ty:ty, $nice:ty, $one:expr) => (
		inflect!($ty, $one);
		inflect_nice!($ty, $nice);
	);

	// Signed, both impls.
	($ty:ty, $nice:ty, $one:literal, $cast:ident) => (
		inflect!($ty, $one, $cast);
		inflect_nice!($ty, $nice, $cast);
	);
}

inflect_nice!(u8, NiceU8, 1);
inflect_nice!(u16, NiceU16, 1);
inflect_nice!(u32, NiceU32, 1);
inflect_nice!(u64, NiceU64, 1);
inflect_nice!(usize, NiceU64, 1);
inflect_nice!(NonZeroU8, NiceU8, Self::MIN);
inflect_nice!(NonZeroU16, NiceU16, Self::MIN);
inflect_nice!(NonZeroU32, NiceU32, Self::MIN);
inflect_nice!(NonZeroU64, NiceU64, Self::MIN);
inflect_nice!(NonZeroUsize, NiceU64, Self::MIN);
inflect_nice!(i8, NiceU8, 1, unsigned_abs);
inflect_nice!(i16, NiceU16, 1, unsigned_abs);
inflect_nice!(i32, NiceU32, 1, unsigned_abs);
inflect_nice!(i64, NiceU64, 1, unsigned_abs);
inflect_nice!(isize, NiceU64, 1, unsigned_abs);

// These aren't nice, but we can still do basic inflection.
inflect!(u128, 1);
inflect!(i128, 1, unsigned_abs);
inflect!(NonZeroU128, Self::MIN);

impl Inflection for f32 {
	/// # Inflect a String.
	fn inflect<'a>(self, singular: &'a str, plural: &'a str) -> &'a str {
		if self.abs().eq(&1.0) { singular } else { plural }
	}
}

impl Inflection for f64 {
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
				$num.nice_inflect("book", "books"),
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

		let rng = fastrand::Rng::new();
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

		let rng = fastrand::Rng::new();
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

		let rng = fastrand::Rng::new();
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

		let rng = fastrand::Rng::new();
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

		let rng = fastrand::Rng::new();
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
