/*!
# Dactyl: Saturated Unsigned Integer Conversion
*/

#![expect(
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_sign_loss,
	trivial_numeric_casts,
	reason = "We're doing a lot of this here.",
)]

use crate::int;



/// # Saturating From.
///
/// Convert between signed/unsigned integers, clamping to the target's
/// `MIN`/`MAX` to prevent overflow or wrapping issues.
///
/// This trait is implemented for all combinations of `u8`, `u16`, `u32`,
/// `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
///
/// ## Examples
///
/// ```
/// use dactyl::traits::SaturatingFrom;
///
/// assert_eq!(
///     i8::saturating_from(-123_456_789_i32),
///     -128_i8, // Saturated!
/// );
///
/// assert_eq!(
///     i8::saturating_from(7_u64),
///     7_i8, // Unsaturated.
/// );
/// ```
pub trait SaturatingFrom<T> {
	/// # Saturating From.
	fn saturating_from(src: T) -> Self;
}

/// # Helper: Saturating From!
///
/// The documentation makes this look worse than it is. The methods come in
/// four possible flavors:
///
/// * Unsaturated (type fits naturally);
/// * Saturated lower bound;
/// * Saturated upper bound;
/// * Saturated upper and lower bounds;
///
/// (There is also a float-specific version at the end, which just does a
/// naive `as` cast.)
macro_rules! sat {
	// Documentation.
	(@docs $from:expr, $to:expr, $examples:expr $(,)?) => (concat!(
		"# Saturating From `", $from, "`

Recast a `", $from, "` to a `", $to, "`, clamping values to `",
$to, "::MIN..=", $to, "::MAX` when necessary to prevent wrapping/overflow
issues.

## Examples

```
use dactyl::traits::SaturatingFrom;

",
$examples,
"
```",
	));

	// Unchecked.
	($ty:ident $($from:ident)+) => ($(
		impl SaturatingFrom<$from> for $ty {
			#[inline]
			#[doc = sat!(@docs
				stringify!($from),
				stringify!($ty),
				concat!("assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MIN),
    ", int!(@min $from), ",
);
assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MAX),
    ", int!(@max $from), ",
);"),
			)]
			fn saturating_from(src: $from) -> Self { src as Self }
		}
	)+);

	// Minimum Bound Check.
	(@min $ty:ident $($from:ident)+) => ($(
		impl SaturatingFrom<$from> for $ty {
			#[inline]
			#[doc = sat!(@docs
				stringify!($from),
				stringify!($ty),
				concat!("assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MIN),
    ", int!(@min $ty), ", // Saturated.
);
assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MAX),
    ", int!(@max $from), ",
);"),
			)]
			fn saturating_from(src: $from) -> Self {
				if src <= int!(@min $ty) { int!(@min $ty) }
				else { src as Self }
			}
		}
	)+);

	// Maximum Bound Check.
	(@max $ty:ident $($from:ident)+) => ($(
		impl SaturatingFrom<$from> for $ty {
			#[inline]
			#[doc = sat!(@docs
				stringify!($from),
				stringify!($ty),
				concat!("assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MIN),
    ", int!(@min $from), ",
);
assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MAX),
    ", int!(@max $ty), ", // Saturated.
);"),
			)]
			fn saturating_from(src: $from) -> Self {
				if int!(@max $ty) <= src { int!(@max $ty) }
				else { src as Self }
			}
		}
	)+);

	// Minimum and Maximum Bound Checks.
	(@both $ty:ident $($from:ident)+) => ($(
		impl SaturatingFrom<$from> for $ty {
			#[inline]
			#[doc = sat!(@docs
				stringify!($from),
				stringify!($ty),
				concat!("assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MIN),
    ", int!(@min $ty), ", // Saturated.
);
assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::MAX),
    ", int!(@max $ty), ", // Saturated.
);"),
			)]
			fn saturating_from(src: $from) -> Self {
				if src <= int!(@min $ty) { int!(@min $ty) }
				else if int!(@max $ty) <= src { int!(@max $ty) }
				else { src as Self }
			}
		}
	)+);

	// Floats.
	(@float $from:ident $($ty:ident)+) => ($(
		impl SaturatingFrom<$from> for $ty {
			#[inline]
			#[doc = sat!(@docs
				stringify!($from),
				stringify!($ty),
				concat!("assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::NEG_INFINITY),
    ", int!(@min $ty), ", // Saturated.
);
assert_eq!(
    ", stringify!($ty), "::saturating_from(", stringify!($from), "::INFINITY),
    ", int!(@max $ty), ", // Saturated.
);"),
			)]
			fn saturating_from(src: $from) -> Self { src as Self }
		}
	)+);
}

// No saturation.
sat!(u128 u128 u64 u32 u16 u8);
sat!(u64       u64 u32 u16 u8);
sat!(u32           u32 u16 u8);
sat!(u16               u16 u8);
sat!(u8                    u8);

#[cfg(target_pointer_width = "16")]
sat!(usize             u16 u8);
#[cfg(target_pointer_width = "32")]
sat!(usize         u32 u16 u8);
#[cfg(target_pointer_width = "64")]
sat!(usize     u64 u32 u16 u8);

sat!(i128      u64 u32 u16 u8 i128 i64 i32 i16 i8);
sat!(i64           u32 u16 u8      i64 i32 i16 i8);
sat!(i32               u16 u8          i32 i16 i8);
sat!(i16                   u8              i16 i8);
sat!(i8                                        i8);

#[cfg(target_pointer_width = "16")]
sat!(isize                 u8 i16 i8);
#[cfg(target_pointer_width = "32")]
sat!(isize             u16 u8 i32 i16 i8);
#[cfg(target_pointer_width = "64")]
sat!(isize         u32 u16 u8 i64 i32 i16 i8);

// Saturate MAX.
sat!(@max u8     u16 u32 u64 u128);
sat!(@max        u16 u32 u64 u128);
sat!(@max            u32 u64 u128);
sat!(@max                u64 u128);

#[cfg(target_pointer_width = "16")]
sat!(@max usize      u32 u64 u128);
#[cfg(target_pointer_width = "32")]
sat!(@max usize          u64 u128);
#[cfg(target_pointer_width = "64")]
sat!(@max usize              u128);

sat!(@max i8  u8 u16 u32 u64 u128);
sat!(@max i16    u16 u32 u64 u128);
sat!(@max i32        u32 u64 u128);
sat!(@max i64            u64 u128);
sat!(@max i128               u128);

#[cfg(target_pointer_width = "16")]
sat!(@max isize  u16 u32 u64 u128);
#[cfg(target_pointer_width = "32")]
sat!(@max isize      u32 u64 u128);
#[cfg(target_pointer_width = "64")]
sat!(@max isize          u64 u128);

// Saturate MIN.
sat!(@min u8    i8);
sat!(@min u16   i8 i16);
sat!(@min u32   i8 i16 i32);
sat!(@min u64   i8 i16 i32 i64);
sat!(@min u128  i8 i16 i32 i64 i128);

#[cfg(target_pointer_width = "16")]
sat!(@min usize i8 i16);
#[cfg(target_pointer_width = "32")]
sat!(@min usize i8 i16 i32);
#[cfg(target_pointer_width = "64")]
sat!(@min usize i8 i16 i32 i64);

// Saturate MIN and MAX.
sat!(@both u8   i16 i32 i64 i128);
sat!(@both u16      i32 i64 i128);
sat!(@both u32          i64 i128);
sat!(@both u64              i128);

#[cfg(target_pointer_width = "16")]
sat!(@both usize    i32 i64 i128);
#[cfg(target_pointer_width = "32")]
sat!(@both usize        i64 i128);
#[cfg(target_pointer_width = "64")]
sat!(@both usize            i128);

sat!(@both i8   i16 i32 i64 i128);
sat!(@both i16      i32 i64 i128);
sat!(@both i32          i64 i128);
sat!(@both i64              i128);

#[cfg(target_pointer_width = "16")]
sat!(@both isize    i32 i64 i128);
#[cfg(target_pointer_width = "32")]
sat!(@both isize        i64 i128);
#[cfg(target_pointer_width = "64")]
sat!(@both isize            i128);

// Handle reverse i/usize generically.
impl<T: SaturatingFrom<int!(@alias usize)>> SaturatingFrom<usize> for T {
	#[inline]
	/// # Saturating From `usize`
	fn saturating_from(src: usize) -> T {
		T::saturating_from(src as int!(@alias usize))
	}
}

impl<T: SaturatingFrom<int!(@alias isize)>> SaturatingFrom<isize> for T {
	#[inline]
	/// # Saturating From `isize`
	fn saturating_from(src: isize) -> T {
		T::saturating_from(src as int!(@alias isize))
	}
}

// Floats (one-way).
sat!(@float f32 u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
sat!(@float f64 u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);



#[cfg(test)]
#[expect(
	clippy::cognitive_complexity,
	reason = "It is what it is.",
)]
mod tests {
	use super::*;

	#[test]
	/// # Saturating From Coverage Check.
	///
	/// Test all combinations with a neutral zero to make sure we didn't
	/// accidentally miss any implementations.
	fn t_zero() {
		macro_rules! test {
			($($ty:ty)+) => ($(
				assert_eq!(<$ty>::saturating_from(0_u8), 0);
				assert_eq!(<$ty>::saturating_from(0_u16), 0);
				assert_eq!(<$ty>::saturating_from(0_u32), 0);
				assert_eq!(<$ty>::saturating_from(0_u64), 0);
				assert_eq!(<$ty>::saturating_from(0_u128), 0);
				assert_eq!(<$ty>::saturating_from(0_usize), 0);

				assert_eq!(<$ty>::saturating_from(0_i8), 0);
				assert_eq!(<$ty>::saturating_from(0_i16), 0);
				assert_eq!(<$ty>::saturating_from(0_i32), 0);
				assert_eq!(<$ty>::saturating_from(0_i64), 0);
				assert_eq!(<$ty>::saturating_from(0_i128), 0);
				assert_eq!(<$ty>::saturating_from(0_isize), 0);

				assert_eq!(<$ty>::saturating_from(0_f32), 0);
				assert_eq!(<$ty>::saturating_from(0_f64), 0);
			)+);
		}

		test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
	}

	#[test]
	/// # Test Minimum Clamps.
	///
	/// Test all pairs ordered by their minimum value, ensuring clamps happen
	/// when needed, and don't when not.
	fn t_min() {
		macro_rules! test {
			($target:ident) => (
				assert_eq!(
					<$target>::saturating_from(<$target>::MIN),
					<$target>::MIN,
					concat!("MIN saturation failed for ", stringify!($target), " to ", stringify!($target)),
				);
			);
			($target:ident $($src:ident)+) => (
				$(
					assert_eq!(
						<$target>::saturating_from(<$src>::MIN),
						<$target>::MIN,
						concat!("MIN saturation failed for ", stringify!($src), " to ", stringify!($target)),
					);

					// The reverse shouldn't saturate.
					assert_eq!(
						<$src>::saturating_from(<$target>::MIN),
						int!(@min $target),
						concat!("MIN saturation failed for ", stringify!($target), " to ", stringify!($src)),
					);
				)+

				// Recurse form the next size.
				test!($($src)+);
			);
		}

		#[cfg(target_pointer_width = "16")]
		test! { u16 usize u32 u64 u128 i8 i16 isize i32 i64 i128 }

		#[cfg(target_pointer_width = "32")]
		test! { u16 u32 usize u64 u128 i8 i16 i32 isize i64 i128 }

		#[cfg(target_pointer_width = "64")]
		test! { u16 u32 u64 usize u128 i8 i16 i32 i64 isize i128 }
	}

	#[test]
	/// # Test Maximum Clamps.
	///
	/// Test all pairs ordered by their maximum value, ensuring clamps happen
	/// when needed, and don't when not.
	fn t_max() {
		macro_rules! test {
			($target:ident) => (
				assert_eq!(
					<$target>::saturating_from(<$target>::MAX),
					<$target>::MAX,
					concat!("MAX saturation failed for ", stringify!($target), " to ", stringify!($target)),
				);
			);
			($target:ident $($src:ident)+) => (
				$(
					assert_eq!(
						<$target>::saturating_from(<$src>::MAX),
						<$target>::MAX,
						concat!("MAX saturation failed for ", stringify!($src), " to ", stringify!($target)),
					);

					// The reverse shouldn't saturate.
					assert_eq!(
						<$src>::saturating_from(<$target>::MAX),
						int!(@max $target),
						concat!("MAX saturation failed for ", stringify!($target), " to ", stringify!($src)),
					);
				)+

				// Recurse form the next size.
				test!($($src)+);
			);
		}

		#[cfg(target_pointer_width = "16")]
		test! { i8 u8 i16 isize u16 usize i32 u32 i64 u64 i128 u128 }

		#[cfg(target_pointer_width = "32")]
		test! { i8 u8 i16 u16 i32 isize u32 usize i64 u64 i128 u128 }

		#[cfg(target_pointer_width = "64")]
		test! { i8 u8 i16 u16 i32 u32 i64 isize u64 usize i128 u128 }
	}
}
