/*!
# Dactyl: Saturated Unsigned Integer Conversion

The `SaturatingFrom` trait allows large primitives to be downcast into smaller
types with values capped at the smaller type's `::MAX` value, avoiding any
possible overflow or wrapping issues.

It is implemented for `u8`, `u16`, `u32`, and `u64` for all types larger than
said type, up to `u128`.

The `usize` type, being variable, works a little differently. It implements
`SaturatingFrom` on `u32`, `u64`, and `u128` regardless of the machine's bit
size, but its ceiling will vary based on the machine's bit size (it could be as
low as `u16::MAX` or as high as `u64::MAX`).

## Examples

```no_run
pub use dactyl::traits::SaturatingFrom;

assert_eq!(u8::saturating_from(1026_u16), 255_u8);
assert_eq!(u8::saturating_from(99_u16), 99_u8);
```
*/

/// # Helper: Generate Trait Implementations.
macro_rules! make_impls {
	($title:expr, $desc:expr, $from:ty, $to:ty) => {
		impl SaturatingFrom<$from> for $to {
			#[doc = $title]
			///
			#[doc = $desc]
			fn saturating_from(src: $from) -> Self {
				src.min(<$from>::from(Self::MAX)) as Self
			}
		}
	};

	($to:ty, $($from:ty),+) => {
		$(
			make_impls!(
				concat!("# Saturating From `", stringify!($from), "`"),
				concat!("This method will safely downcast any `", stringify!($from), "` into a `", stringify!($to), "`, capping the value to `", stringify!($to), "::MAX` to prevent overflow or wrapping."),
				$from,
				$to
			);
		)+
	}
}

/// # Saturating From.
///
/// Convert an unsigned integer of a larger type into `Self`, capping the
/// maximum value to `Self::MAX` to prevent overflow or wrapping.
pub trait SaturatingFrom<T> {
	/// # Saturating From.
	fn saturating_from(src: T) -> Self;
}

// Most conversions work nice and neat.
make_impls!(u8, u16, u32, u64, u128, usize);
make_impls!(u16, u32, u64, u128, usize);
make_impls!(u32, u64, u128); // from<usize> is manual, below.
make_impls!(u64, u128);      // from<usize> is manual, below.



impl SaturatingFrom<usize> for u32 {
	/// # Saturating From `usize`
	fn saturating_from(src: usize) -> Self {
		// 64-bit pointers have to be saturated down.
		if cfg!(target_pointer_width = "64") {
			src.min(Self::MAX as usize) as Self
		}
		else { src as Self }
	}
}

impl SaturatingFrom<usize> for u64 {
	/// # Saturating From `usize`
	fn saturating_from(src: usize) -> Self { src as Self }
}



// The conversion traits for `u32`, `u64`, and `u128` have to be handled
// manually to account for `usize`'s variable width.

impl SaturatingFrom<u32> for usize {
	/// # Saturating From `u32`
	fn saturating_from(src: u32) -> Self {
		// 16-bit pointers have to be saturated down.
		if cfg!(target_pointer_width = "16") {
			src.min(u32::from(u16::MAX)) as Self
		}
		else { src as Self }
	}
}

impl SaturatingFrom<u64> for usize {
	/// # Saturating From `u64`
	fn saturating_from(src: u64) -> Self {
		// Saturate 16-bits.
		if cfg!(target_pointer_width = "16") {
			src.min(u64::from(u16::MAX)) as Self
		}
		// Saturate 32-bits.
		else if cfg!(target_pointer_width = "32") {
			src.min(u64::from(u32::MAX)) as Self
		}
		else { src as Self }
	}
}

impl SaturatingFrom<u128> for usize {
	/// # Saturating From `u128`
	fn saturating_from(src: u128) -> Self {
		// Saturate to 64-bits and try again.
		Self::saturating_from(u64::saturating_from(src))
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_u8_from() {
		for (i, t) in (0..=u16::from(u8::MAX)).zip(0..=u8::MAX) {
			assert_eq!(u8::saturating_from(i), t);
		}
		for (i, t) in (0..=u32::from(u8::MAX)).zip(0..=u8::MAX) {
			assert_eq!(u8::saturating_from(i), t);
		}
		for (i, t) in (0..=u64::from(u8::MAX)).zip(0..=u8::MAX) {
			assert_eq!(u8::saturating_from(i), t);
		}
		for (i, t) in (0..=u128::from(u8::MAX)).zip(0..=u8::MAX) {
			assert_eq!(u8::saturating_from(i), t);
		}
		for (i, t) in (0..=usize::from(u8::MAX)).zip(0..=u8::MAX) {
			assert_eq!(u8::saturating_from(i), t);
		}

		assert_eq!(u8::saturating_from(u16::MAX), u8::MAX);
		assert_eq!(u8::saturating_from(u32::MAX), u8::MAX);
		assert_eq!(u8::saturating_from(u64::MAX), u8::MAX);
		assert_eq!(u8::saturating_from(u128::MAX), u8::MAX);
		assert_eq!(u8::saturating_from(usize::MAX), u8::MAX);
	}

	#[test]
	fn t_u16_from() {
		for (i, t) in (0..=u32::from(u16::MAX)).zip(0..=u16::MAX) {
			assert_eq!(u16::saturating_from(i), t);
		}
		for (i, t) in (0..=u64::from(u16::MAX)).zip(0..=u16::MAX) {
			assert_eq!(u16::saturating_from(i), t);
		}
		for (i, t) in (0..=u128::from(u16::MAX)).zip(0..=u16::MAX) {
			assert_eq!(u16::saturating_from(i), t);
		}
		for (i, t) in (0..=usize::from(u16::MAX)).zip(0..=u16::MAX) {
			assert_eq!(u16::saturating_from(i), t);
		}

		assert_eq!(u16::saturating_from(u32::MAX), u16::MAX);
		assert_eq!(u16::saturating_from(u64::MAX), u16::MAX);
		assert_eq!(u16::saturating_from(u128::MAX), u16::MAX);
		assert_eq!(u16::saturating_from(usize::MAX), u16::MAX);
	}

	#[test]
	#[ignore] // This takes a very long time to run.
	fn t_u32_from() {
		for (i, t) in (0..=u64::from(u32::MAX)).zip(0..=u32::MAX) {
			assert_eq!(u32::saturating_from(i), t);
		}
		for (i, t) in (0..=u128::from(u32::MAX)).zip(0..=u32::MAX) {
			assert_eq!(u32::saturating_from(i), t);
		}

		assert_eq!(u32::saturating_from(u64::MAX), u32::MAX);
		assert_eq!(u32::saturating_from(u128::MAX), u32::MAX);
		assert!(u32::saturating_from(usize::MAX) <= u32::MAX);
	}

	#[test]
	fn t_usize_from() {
		assert!(usize::saturating_from(u32::MAX) <= usize::MAX);
		assert!(usize::saturating_from(u64::MAX) <= usize::MAX);
		assert_eq!(usize::saturating_from(u128::MAX), usize::MAX);
	}
}
