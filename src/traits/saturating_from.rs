/*!
# Dactyl: Saturated Unsigned Integer Conversion

The `SaturatingFrom` trait allows integer primitives to be freely converted
between one another in a saturating fashion.

To make life easy, `int::saturating_from(float)` is implemented as well, but
this is functionally identical to writing `float as int`, since such casts are
already saturating in Rust.

## Examples

```
use dactyl::traits::SaturatingFrom;

// Too big.
assert_eq!(u8::saturating_from(1026_u16), 255_u8);

// Too small.
assert_eq!(u8::saturating_from(-1026_i32), 0_u8);

// Just right.
assert_eq!(u8::saturating_from(99_u64), 99_u8);
```
*/

#![expect(
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_sign_loss,
	reason = "We're doing a lot of this here.",
)]



/// # Saturating From.
///
/// Convert between numeric types, clamping to `Self::MIN..=Self::MAX` to
/// prevent overflow or wrapping issues.
pub trait SaturatingFrom<T> {
	/// # Saturating From.
	///
	/// Convert `T` to `Self`, clamping to `Self::MIN..=Self::MAX` as required
	/// to prevent overflow or wrapping.
	fn saturating_from(src: T) -> Self;
}

// All the integer conversions are built at compile-time.
include!(concat!(env!("OUT_DIR"), "/dactyl-saturation.rs"));

/// # Helper: Generate Float Impls.
///
/// Floats are mercifully saturating on their own.
macro_rules! float {
	($from:ty, $($to:ty),+) => ($(
		impl SaturatingFrom<$from> for $to {
			#[inline]
			#[doc = concat!("# Saturating From `", stringify!($from), "`")]
			#[doc = ""]
			#[doc = concat!("This method will safely recast any `", stringify!($from), "` into a `", stringify!($to), "`, clamping the values to `", stringify!($to), "::MIN..=", stringify!($to), "::MAX` to prevent overflow or wrapping.")]
			fn saturating_from(src: $from) -> Self { src as Self }
		}
	)+);
}

float!(f32, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
float!(f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);



#[cfg(test)]
#[expect(
	clippy::cognitive_complexity,
	trivial_numeric_casts,
	reason = "It is what it is.",
)]
/// # Saturation Tests.
///
/// There isn't a particularly good way to do this other than to walk through
/// fixed ranges and assert smaller types get clamped, and greater-equal ones
/// don't.
///
/// Usize/isize tests vanish beyond the 16-bit ranges to avoid clutter, but
/// have their own separate cfg-gated test that should verify they work/fail
/// beyond that given the target's pointer width.
///
/// The 16-bit and 128-bit sized tests may be buggy as they haven't actually
/// been run since Rust doesn't support any architectures at either width
/// yet. TBD.
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 500_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 500; // Miri runs way too slow for half a million tests.

	/// # Helper: Assert `SaturatingFrom` is Lossless.
	macro_rules! cast_assert_same {
		($to:ty, $raw:ident, $($from:ty),+) => ($(
			assert_eq!(
				<$to>::saturating_from($raw as $from),
				$raw as $to,
				concat!("Expected {}_", stringify!($to), " from {}_", stringify!($from), "."),
				$raw,
				$raw,
			);
		)+);
	}

	/// # Helper: Assert `SaturatingFrom` Clamps to `Self::MAX`.
	macro_rules! cast_assert_max {
		($to:ty, $raw:ident, $($from:ty),+) => ($(
			assert_eq!(
				<$to>::saturating_from($raw as $from),
				<$to>::MAX,
				concat!("Expected {}_", stringify!($to), " from {}_", stringify!($from), "."),
				<$to>::MAX,
				$raw,
			);
		)+);
	}

	/// # Helper: Assert `SaturatingFrom` Clamps to `Self::MIN`.
	macro_rules! cast_assert_min {
		($to:ty, $raw:ident, $($from:ty),+) => ($(
			assert_eq!(
				<$to>::saturating_from($raw as $from),
				<$to>::MIN,
				concat!("Expected {}_", stringify!($to), " from {}_", stringify!($from), "."),
				<$to>::MIN,
				$raw,
			);
		)+);
	}

	#[test]
	fn t_saturating_rng_i28min_i64min() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i128(i128::MIN..i64::MIN as i128)).take(SAMPLE_SIZE) {
			// Floor reached.
			cast_assert_min!( u8,    i, i128);
			cast_assert_min!( u16,   i, i128);
			cast_assert_min!( u32,   i, i128);
			cast_assert_min!( u64,   i, i128);
			cast_assert_min!( u128,  i, i128);
			cast_assert_min!( usize, i, i128);
			cast_assert_min!( i8,    i, i128);
			cast_assert_min!( i16,   i, i128);
			cast_assert_min!( i32,   i, i128);
			cast_assert_min!( i64,   i, i128);

			// Still in range.
			cast_assert_same!(i128,  i, i128);
		}
	}

	#[test]
	fn t_saturating_rng_i64min_i32min() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i64(i64::MIN..i32::MIN as i64)).take(SAMPLE_SIZE) {
			// Floor reached.
			cast_assert_min!( u8,    i, i64, i128);
			cast_assert_min!( u16,   i, i64, i128);
			cast_assert_min!( u32,   i, i64, i128);
			cast_assert_min!( u64,   i, i64, i128);
			cast_assert_min!( u128,  i, i64, i128);
			cast_assert_min!( usize, i, i64, i128);
			cast_assert_min!( i8,    i, i64, i128);
			cast_assert_min!( i16,   i, i64, i128);
			cast_assert_min!( i32,   i, i64, i128);

			// Still in range.
			cast_assert_same!(i64,   i, i64, i128);
			cast_assert_same!(i128,  i, i64, i128);
		}
	}

	#[test]
	fn t_saturating_rng_i32min_i16min() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i32(i32::MIN..i16::MIN as i32)).take(SAMPLE_SIZE) {
			// Floor reached.
			cast_assert_min!( u8,    i, i32, i64, i128);
			cast_assert_min!( u16,   i, i32, i64, i128);
			cast_assert_min!( u32,   i, i32, i64, i128);
			cast_assert_min!( u64,   i, i32, i64, i128);
			cast_assert_min!( u128,  i, i32, i64, i128);
			cast_assert_min!( usize, i, i32, i64, i128);
			cast_assert_min!( i8,    i, i32, i64, i128);
			cast_assert_min!( i16,   i, i32, i64, i128);

			// Still in range.
			cast_assert_same!(i32,   i, i32, i64, i128);
			cast_assert_same!(i64,   i, i32, i64, i128);
			cast_assert_same!(i128,  i, i32, i64, i128);
		}
	}

	#[cfg(not(miri))]
	#[test]
	fn t_saturating_rng_i16min_i8min() {
		for i in i16::MIN..i8::MIN as i16 {
			// Floor reached.
			cast_assert_min!( u8,    i, i16, i32, i64, i128, isize);
			cast_assert_min!( u16,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u32,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u64,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u128,  i, i16, i32, i64, i128, isize);
			cast_assert_min!( usize, i, i16, i32, i64, i128, isize);
			cast_assert_min!( i8,    i, i16, i32, i64, i128, isize);

			// Still in range.
			cast_assert_same!(i16,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i32,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i64,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i128,  i, i16, i32, i64, i128, isize);
			cast_assert_same!(isize, i, i16, i32, i64, i128, isize);
		}
	}

	#[cfg(miri)]
	#[test]
	fn t_saturating_rng_i16min_i8min() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i16(i16::MIN..i8::MIN as i16)).take(SAMPLE_SIZE) {
			// Floor reached.
			cast_assert_min!( u8,    i, i16, i32, i64, i128, isize);
			cast_assert_min!( u16,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u32,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u64,   i, i16, i32, i64, i128, isize);
			cast_assert_min!( u128,  i, i16, i32, i64, i128, isize);
			cast_assert_min!( usize, i, i16, i32, i64, i128, isize);
			cast_assert_min!( i8,    i, i16, i32, i64, i128, isize);

			// Still in range.
			cast_assert_same!(i16,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i32,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i64,   i, i16, i32, i64, i128, isize);
			cast_assert_same!(i128,  i, i16, i32, i64, i128, isize);
			cast_assert_same!(isize, i, i16, i32, i64, i128, isize);
		}
	}

	#[test]
	fn t_saturating_rng_i8min_0() {
		// All unsigned should be floored, but all signed should be fine.
		for i in i8::MIN..0 {
			// Floor reached.
			cast_assert_min!( u8,    i, i8, i16, i32, i64, i128, isize);
			cast_assert_min!( u16,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_min!( u32,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_min!( u64,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_min!( u128,  i, i8, i16, i32, i64, i128, isize);
			cast_assert_min!( usize, i, i8, i16, i32, i64, i128, isize);

			// Still in range.
			cast_assert_same!(i8,    i, i8, i16, i32, i64, i128, isize);
			cast_assert_same!(i16,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_same!(i32,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_same!(i64,   i, i8, i16, i32, i64, i128, isize);
			cast_assert_same!(i128,  i, i8, i16, i32, i64, i128, isize);
			cast_assert_same!(isize, i, i8, i16, i32, i64, i128, isize);
		}
	}

	#[test]
	fn t_saturating_rng_0_i8max() {
		// All saturations should be lossless for upper i8 range.
		for i in 0..=i8::MAX {
			cast_assert_same!(i8,    i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i16,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i32,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(isize, i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u8,    i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
		}
	}

	#[test]
	fn t_saturating_rng_i8max_u8max() {
		for i in (i8::MAX as u8 + 1)..=u8::MAX {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

			// Still in range.
			cast_assert_same!(i16,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i32,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(isize, i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u8,    i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
		}
	}

	#[cfg(not(miri))]
	#[test]
	fn t_saturating_rng_u8max_i16max() {
		for i in (u8::MAX as i16 + 1)..=i16::MAX {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_max!( u8,    i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);

			// Still in range.
			cast_assert_same!(i16,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i32,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(isize, i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
		}
	}

	#[cfg(miri)]
	#[test]
	fn t_saturating_rng_u8max_i16max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i16((u8::MAX as i16 + 1)..=i16::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_max!( u8,    i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);

			// Still in range.
			cast_assert_same!(i16,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i32,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(isize, i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize);
		}
	}

	#[cfg(not(miri))]
	#[test]
	fn t_saturating_rng_i16max_u16max() {
		for i in (i16::MAX as u16 + 1)..=u16::MAX {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_max!( u8,    i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_max!( i16,   i, i32, i64, i128, u16, u32, u64, u128, usize);

			// Still in range.
			cast_assert_same!(i32,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i32, i64, i128, u16, u32, u64, u128, usize);
		}
	}

	#[cfg(miri)]
	#[test]
	fn t_saturating_rng_i16max_u16max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u16((i16::MAX as u16 + 1)..=u16::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_max!( u8,    i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_max!( i16,   i, i32, i64, i128, u16, u32, u64, u128, usize);

			// Still in range.
			cast_assert_same!(i32,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(i64,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(i128,  i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u16,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u32,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u64,   i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(u128,  i, i32, i64, i128, u16, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i32, i64, i128, u16, u32, u64, u128, usize);
		}
	}

	#[test]
	fn t_saturating_rng_u16max_i32max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i32((u16::MAX as i32 + 1)..=i32::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i32, i64, i128, u32, u64, u128);
			cast_assert_max!( u8,    i, i32, i64, i128, u32, u64, u128);
			cast_assert_max!( i16,   i, i32, i64, i128, u32, u64, u128);
			cast_assert_max!( u16,   i, i32, i64, i128, u32, u64, u128);

			// Still in range.
			cast_assert_same!(i32,   i, i32, i64, i128, u32, u64, u128);
			cast_assert_same!(i64,   i, i32, i64, i128, u32, u64, u128);
			cast_assert_same!(i128,  i, i32, i64, i128, u32, u64, u128);
			cast_assert_same!(u32,   i, i32, i64, i128, u32, u64, u128);
			cast_assert_same!(u64,   i, i32, i64, i128, u32, u64, u128);
			cast_assert_same!(u128,  i, i32, i64, i128, u32, u64, u128);
		}
	}

	#[test]
	fn t_saturating_rng_i32max_u32max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32((i32::MAX as u32 + 1)..=u32::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i64, i128, u32, u64, u128);
			cast_assert_max!( u8,    i, i64, i128, u32, u64, u128);
			cast_assert_max!( i16,   i, i64, i128, u32, u64, u128);
			cast_assert_max!( u16,   i, i64, i128, u32, u64, u128);
			cast_assert_max!( i32,   i, i64, i128, u32, u64, u128);

			// Still in range.
			cast_assert_same!(i64,   i, i64, i128, u32, u64, u128);
			cast_assert_same!(i128,  i, i64, i128, u32, u64, u128);
			cast_assert_same!(u32,   i, i64, i128, u32, u64, u128);
			cast_assert_same!(u64,   i, i64, i128, u32, u64, u128);
			cast_assert_same!(u128,  i, i64, i128, u32, u64, u128);
		}
	}

	#[test]
	fn t_saturating_rng_u32max_i64max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i64((u32::MAX as i64 + 1)..=i64::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i64, i128, u64, u128);
			cast_assert_max!( u8,    i, i64, i128, u64, u128);
			cast_assert_max!( i16,   i, i64, i128, u64, u128);
			cast_assert_max!( u16,   i, i64, i128, u64, u128);
			cast_assert_max!( i32,   i, i64, i128, u64, u128);
			cast_assert_max!( u32,   i, i64, i128, u64, u128);

			// Still in range.
			cast_assert_same!(i64,   i, i64, i128, u64, u128);
			cast_assert_same!(i128,  i, i64, i128, u64, u128);
			cast_assert_same!(u64,   i, i64, i128, u64, u128);
			cast_assert_same!(u128,  i, i64, i128, u64, u128);
		}
	}

	#[test]
	fn t_saturating_rng_i64max_u64max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64((i64::MAX as u64 + 1)..=u64::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i128, u64, u128);
			cast_assert_max!( u8,    i, i128, u64, u128);
			cast_assert_max!( i16,   i, i128, u64, u128);
			cast_assert_max!( u16,   i, i128, u64, u128);
			cast_assert_max!( i32,   i, i128, u64, u128);
			cast_assert_max!( u32,   i, i128, u64, u128);
			cast_assert_max!( i64,   i, i128, u64, u128);

			// Still in range.
			cast_assert_same!(i128,  i, i128, u64, u128);
			cast_assert_same!(u64,   i, i128, u64, u128);
			cast_assert_same!(u128,  i, i128, u64, u128);
		}
	}

	#[test]
	fn t_saturating_rng_u64max_i128max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.i128((u64::MAX as i128 + 1)..=i128::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, i128, u128);
			cast_assert_max!( u8,    i, i128, u128);
			cast_assert_max!( i16,   i, i128, u128);
			cast_assert_max!( u16,   i, i128, u128);
			cast_assert_max!( i32,   i, i128, u128);
			cast_assert_max!( u32,   i, i128, u128);
			cast_assert_max!( i64,   i, i128, u128);
			cast_assert_max!( u64,   i, i128, u128);

			// Still in range.
			cast_assert_same!(i128,  i, i128, u128);
			cast_assert_same!(u128,  i, i128, u128);
		}
	}

	#[test]
	fn t_saturating_rng_i128max_u128max() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u128((i128::MAX as u128 + 1)..=u128::MAX)).take(SAMPLE_SIZE) {
			// Ceiling reached.
			cast_assert_max!( i8,    i, u128);
			cast_assert_max!( u8,    i, u128);
			cast_assert_max!( i16,   i, u128);
			cast_assert_max!( u16,   i, u128);
			cast_assert_max!( i32,   i, u128);
			cast_assert_max!( u32,   i, u128);
			cast_assert_max!( i64,   i, u128);
			cast_assert_max!( u64,   i, u128);
			cast_assert_max!( i128,  i, u128);

			// Still in range.
			cast_assert_same!(u128,  i, u128);
		}
	}

	#[cfg(target_pointer_width = "16")]
	#[test]
	fn t_saturating_sized16() {
		let mut rng = fastrand::Rng::new();

		// Both should be floored below i16.
		for i in std::iter::repeat_with(|| rng.i32(i32::MIN..i16::MIN as i32)).take(SAMPLE_SIZE) {
			cast_assert_min!(isize, i, i32, i64, i128);
			cast_assert_min!(usize, i, i32, i64, i128);
		}

		// isize should max out after i16::MAX.
		for i in (i16::MAX as u16 + 1)..=u16::MAX {
			cast_assert_max!(isize, i, i32, i64, i128, u16, u32, u64, u128, usize);
		}

		// usize should max out after u16::MAX.
		for i in std::iter::repeat_with(|| rng.i32((u16::MAX as i32 + 1)..=i32::MAX)).take(SAMPLE_SIZE) {
			cast_assert_max!(usize, i, i32, i64, i128, u32, u64, u128);
		}
	}

	#[cfg(target_pointer_width = "32")]
	#[test]
	fn t_saturating_sized32() {
		let mut rng = fastrand::Rng::new();

		// isize should be fine down to i32::MIN, but usize will get floored.
		for i in std::iter::repeat_with(|| rng.i32(i32::MIN..i16::MIN as i32)).take(SAMPLE_SIZE) {
			cast_assert_same!(isize, i, i32, i64, i128, isize);
			cast_assert_min!( usize, i, i32, i64, i128, isize);
		}

		// Below i32, isize should be floored too.
		for i in std::iter::repeat_with(|| rng.i64(i64::MIN..i32::MIN as i64)).take(SAMPLE_SIZE) {
			cast_assert_min!(isize, i, i64, i128);
		}

		// Both should be fine up to i32::MAX.
		for i in std::iter::repeat_with(|| rng.i32((u16::MAX as i32 + 1)..=i32::MAX)).take(SAMPLE_SIZE) {
			cast_assert_same!(isize, i, i32, i64, i128, isize, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i32, i64, i128, isize, u32, u64, u128, usize);
		}

		// isize should max out after i32::MAX, but usize should be fine.
		for i in std::iter::repeat_with(|| rng.u32((i32::MAX as u32 + 1)..=u32::MAX)).take(SAMPLE_SIZE) {
			cast_assert_max!( isize, i, i64, i128, u32, u64, u128, usize);
			cast_assert_same!(usize, i, i64, i128, u32, u64, u128, usize);
		}

		// usize should max out after u32::MAX.
		for i in std::iter::repeat_with(|| rng.i64((u32::MAX as i64 + 1)..=i64::MAX)).take(SAMPLE_SIZE) {
			cast_assert_max!(usize, i, i64, i128, u64, u128);
		}
	}

	#[cfg(target_pointer_width = "64")]
	#[test]
	fn t_saturating_sized64() {
		let mut rng = fastrand::Rng::new();

		// isize should be fine down to i64::MIN, but usize will get floored.
		for i in std::iter::repeat_with(|| rng.i64(i64::MIN..i32::MIN as i64)).take(SAMPLE_SIZE) {
			cast_assert_same!(isize, i, i64, i128, isize);
			cast_assert_min!( usize, i, i64, i128, isize);
		}

		// Below i64, isize should be floored too.
		for i in std::iter::repeat_with(|| rng.i128(i128::MIN..i64::MIN as i128)).take(SAMPLE_SIZE) {
			cast_assert_min!(isize, i, i128);
		}

		// Both should be fine up to i64::MAX.
		for i in std::iter::repeat_with(|| rng.i64((u32::MAX as i64 + 1)..=i64::MAX)).take(SAMPLE_SIZE) {
			cast_assert_same!(isize, i, i64, i128, isize, u64, u128, usize);
			cast_assert_same!(usize, i, i64, i128, isize, u64, u128, usize);
		}

		// isize should max out after i64::MAX, but usize should be fine.
		for i in std::iter::repeat_with(|| rng.u64((i64::MAX as u64 + 1)..=u64::MAX)).take(SAMPLE_SIZE) {
			cast_assert_max!( isize, i, i128, u64, u128, usize);
			cast_assert_same!(usize, i, i128, u64, u128, usize);
		}

		// usize should max out after u64::MAX.
		for i in std::iter::repeat_with(|| rng.i128((u64::MAX as i128 + 1)..=i128::MAX)).take(SAMPLE_SIZE) {
			cast_assert_max!(usize, i, i128, u128);
		}
	}
}
