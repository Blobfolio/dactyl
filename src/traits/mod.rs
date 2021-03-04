/*!
# Dactyl: Saturated Unsigned Integer Conversion

The `SaturatingFrom` trait allows large primitives to be downcast into smaller
types with values capped at the smaller type's `::MAX` value, avoiding any
possible overflow or wrapping issues. It's a clamp, basically, except all uints
share the same bottom.

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



/// # Helper: Title/Description.
///
/// This generates a formatted title and description for the documentation.
macro_rules! impl_meta {
	// Title/Desc.
	($to:ty, $from:ty) => (
		concat!(
			"# Saturating From `",
			stringify!($from),
			"`\n",
			"This method will safely recast any `",
			stringify!($from),
			"` into a `",
			stringify!($to),
			"`, capping the values at `0` or `",
			stringify!($to),
			"::MAX` to prevent overflow or wrapping."
		)
	);
}

/// # Helper: Generate Trait Implementations.
///
/// This generates implementations for unsigned sources, with or without an
/// upper cap.
macro_rules! unsigned_to_unsigned {
	// Cap to max.
	($meta:expr, $from:ty, $to:ty, $MAX:literal) => (
		impl SaturatingFrom<$from> for $to {
			#[doc(inline)]
			#[doc = $meta]
			fn saturating_from(src: $from) -> Self {
				if src >= $MAX { $MAX }
				else { src as Self }
			}
		}
	);

	// Direct cast.
	($meta:expr, $from:ty, $to:ty) => (
		impl SaturatingFrom<$from> for $to {
			#[doc(inline)]
			#[doc = $meta]
			fn saturating_from(src: $from) -> Self { src as Self }
		}
	);

	// Cap to max.
	($to:ty, $MAX:literal, ($($from:ty),+)) => (
		$( unsigned_to_unsigned!(impl_meta!($to, $from), $from, $to, $MAX); )+
	);

	// Direct cast.
	($to:ty, ($($from:ty),+)) => (
		$( unsigned_to_unsigned!(impl_meta!($to, $from), $from, $to); )+
	);
}

/// # Helper: Generate Trait Implementations (Signed).
///
/// This generates implementations for signed sources, with or without an
/// upper cap. All signed types have a lower cap of zero.
macro_rules! signed_to_unsigned {
	// Cap to min/max.
	($meta:expr, $from:ty, $to:ty, $MAX:literal) => (
		impl SaturatingFrom<$from> for $to {
			#[doc(inline)]
			#[doc = $meta]
			fn saturating_from(src: $from) -> Self {
				if src <= 0 { 0 }
				else if src >= $MAX { Self::MAX }
				else { src as Self }
			}
		}
	);

	// Cap to min.
	($meta:expr, $from:ty, $to:ty) => (
		impl SaturatingFrom<$from> for $to {
			#[doc(inline)]
			#[doc = $meta]
			fn saturating_from(src: $from) -> Self {
				if src <= 0 { 0 }
				else { src as Self }
			}
		}
	);

	// Cap to min/max.
	($to:ty, $MAX:literal, ($($from:ty),+)) => (
		$( signed_to_unsigned!(impl_meta!($to, $from), $from, $to, $MAX); )+
	);

	// Cap to min.
	($to:ty, ($($from:ty),+)) => (
		$( signed_to_unsigned!(impl_meta!($to, $from), $from, $to); )+
	);
}



/// # Saturating From.
///
/// Convert an unsigned integer of a larger type into `Self`, capping the
/// maximum value to `Self::MAX` to prevent overflow or wrapping.
pub trait SaturatingFrom<T> {
	/// # Saturating From.
	fn saturating_from(src: T) -> Self;
}



// These three are always the same.
unsigned_to_unsigned!(u8, 255, (u16, u32, u64, u128, usize));
unsigned_to_unsigned!(u16, 65_535, (u32, u64, u128, usize));
unsigned_to_unsigned!(u32, 4_294_967_295, (u64, u128)); // Usize conditional, below.
unsigned_to_unsigned!(u64, 18_446_744_073_709_551_615, (u128)); // Usize conditional, below.
unsigned_to_unsigned!(u128, (usize));

// usize-to-u32 varies by pointer.
#[cfg(any(target_pointer_width = "16", target_pointer_width="32"))] // 16/32 fit.
unsigned_to_unsigned!(u32, (usize));
#[cfg(any(target_pointer_width = "64", target_pointer_width="128"))] // 64/128 don't.
unsigned_to_unsigned!(u32, 4_294_967_295, (usize));

// usize-to-u64 varies by pointer.
#[cfg(not(target_pointer_width = "128"))] // 16/32/64 fit.
unsigned_to_unsigned!(u64, (usize));
#[cfg(target_pointer_width = "128")] // 128 doesn't.
unsigned_to_unsigned!(u64, 18_446_744_073_709_551_615, (usize));

// Usize varies by pointer.
#[cfg(target_pointer_width = "16")] // 32/64/128 don't fit.
unsigned_to_unsigned!(usize, 65_535, (u32, u64, u128));

#[cfg(target_pointer_width = "32")] // 32 fits.
unsigned_to_unsigned!(usize, (u32));
#[cfg(target_pointer_width = "32")] // 64, 128 don't.
unsigned_to_unsigned!(usize, 4_294_967_295, (u64, u128));

#[cfg(target_pointer_width = "64")] // 32/64 fits.
unsigned_to_unsigned!(usize, (u32, u64));
#[cfg(target_pointer_width = "64")] // 128 doesn't.
unsigned_to_unsigned!(usize, 18_446_744_073_709_551_615, (u128));

#[cfg(target_pointer_width = "128")]
unsigned_to_unsigned!(usize, (u32, u64, u128));



// Converting from signed types. These are straight conversions.
signed_to_unsigned!(u8, (i8));
signed_to_unsigned!(u16, (i8, i16));
signed_to_unsigned!(u32, (i8, i16, i32));
signed_to_unsigned!(u64, (i8, i16, i32, i64));
signed_to_unsigned!(u128, (i8, i16, i32, i64, i128, isize));
signed_to_unsigned!(usize, (i8, i16, isize));

// These require max capping.
signed_to_unsigned!(u8, 255, (i16, i32, i64, i128, isize));
signed_to_unsigned!(u16, 65_535, (i32, i64, i128, isize));
signed_to_unsigned!(u32, 4_294_967_295, (i64, i128));
signed_to_unsigned!(u64, 18_446_744_073_709_551_615, (i128));

// U32/isize varies by pointer.
#[cfg(any(target_pointer_width = "16", target_pointer_width="32"))]
signed_to_unsigned!(u32, (isize));
#[cfg(any(target_pointer_width = "64", target_pointer_width="128"))]
signed_to_unsigned!(u32, 4_294_967_295, (isize));

// U64/isize varies by pointer.
#[cfg(not(target_pointer_width = "128"))]
signed_to_unsigned!(u64, (isize));
#[cfg(target_pointer_width = "128")]
signed_to_unsigned!(u64, 18_446_744_073_709_551_615, (isize));

// All other usize conversions vary by pointer.
#[cfg(target_pointer_width = "16")]
signed_to_unsigned!(usize, 65_535, (i32, i64, i128));

#[cfg(target_pointer_width = "32")]
signed_to_unsigned!(usize, (i32));
#[cfg(target_pointer_width = "32")]
signed_to_unsigned!(usize, 4_294_967_295, (i64, i128));

#[cfg(target_pointer_width = "64")]
signed_to_unsigned!(usize, (i32, i64));
#[cfg(target_pointer_width = "64")]
signed_to_unsigned!(usize, 18_446_744_073_709_551_615, (i128));

#[cfg(target_pointer_width = "128")]
signed_to_unsigned!(usize, (i32, i64, i128));



#[cfg(test)]
mod tests {
	use super::*;

	/// # Test Flooring.
	macro_rules! test_impl {
		($type:ty, ($($val:literal),+)) => (
			$( assert_eq!(<$type>::saturating_from($val), <$type>::MIN); )+
		);

		// SaturatingFrom is implemented for all signed types.
		($type:ty) => {
			test_impl!($type, (-1_i8, -1_i16, -1_i32, -1_i64, -1_i128, -1_isize));
			test_impl!($type, (0_i8, 0_i16, 0_i32, 0_i64, 0_i128, 0_isize));
		};
	}

	/// # Test Ceiling.
	macro_rules! test_impl_max {
		($type:ty, ($($from:ty),+)) => (
			$( assert_eq!(<$type>::saturating_from(<$from>::MAX), <$type>::MAX); )+
		);
	}

	/// # Range Testing.
	macro_rules! test_impl_range {
		($type:ty, ($($from:ty),+)) => {
			for i in 0..=<$type>::MAX {
				$( assert_eq!(<$type>::saturating_from(i as $from), i); )+
			}
		};
	}

	/// # Range Testing (subset).
	///
	/// This computes casting for a subset of the total type range; this allows
	/// testing large types to finish in a reasonable amount of time.
	macro_rules! test_impl_subrange {
		($type:ty, ($($from:ty),+)) => {
			let mut i = <$type>::MIN;
			let mut step: $type = 0;
			while <$type>::MAX - i > step {
				i += step;
				$( assert_eq!(<$type>::saturating_from(i as $from), i); )+
				step += 1;
			}
		};
	}

	#[test]
	/// # Test Implementations
	///
	/// This makes sure we've actually implemented all the expected type-to-
	/// type conversions, as well as making sure negative/zero signed integer
	/// conversions work as expected.
	fn t_impls() {
		test_impl!(u8, (0_u16, 0_u32, 0_u64, 0_u128, 0_usize));
		test_impl!(u8);

		test_impl!(u16, (0_u32, 0_u64, 0_u128, 0_usize));
		test_impl!(u16);

		test_impl!(u32, (0_u64, 0_u128, 0_usize));
		test_impl!(u32);

		test_impl!(u64, (0_u128, 0_usize));
		test_impl!(u64);

		test_impl!(u128, (0_usize));
		test_impl!(u128);

		test_impl!(usize, (0_u32, 0_u64, 0_u128));
		test_impl!(usize);
	}

	#[test]
	/// # Test u8
	///
	/// Make sure larger ints correctly saturate to u8.
	fn t_u8_from() {
		test_impl_range!(u8, (u16, u32, u64, u128, usize));
		test_impl_max!(u8, (u16, u32, u64, u128, usize));
	}

	#[test]
	fn t_u16_from() {
		test_impl_range!(u16, (u32, u64, u128, usize));
		test_impl_max!(u16, (u32, u64, u128, usize));
	}

	#[test]
	fn t_u32_from() {
		test_impl_subrange!(u32, (u64, u128));
		test_impl_max!(u32, (u64, u128));
	}

	#[test]
	fn t_u64_from() {
		test_impl_subrange!(u64, (u128));
		test_impl_max!(u64, (u128));
	}

	#[cfg(target_pointer_width = "16")]
	#[test]
	fn t_usize_from() {
		test_impl_range!(usize, (u32, u64, u128));
		test_impl_max!(usize, (u32, u64, u128));
	}

	#[cfg(target_pointer_width = "32")]
	#[test]
	fn t_usize_from() {
		test_impl_subrange!(usize, (u32, u64, u128));
		test_impl_max!(usize, (u32, u64, u128));
		test_impl_max!(u32, (usize));
	}

	#[cfg(target_pointer_width = "64")]
	#[test]
	fn t_usize_from() {
		test_impl_subrange!(usize, (u64, u128));
		assert_eq!(u32::saturating_from(usize::MAX), u32::MAX);
		test_impl_max!(u64, (usize));
	}

	#[cfg(target_pointer_width = "128")]
	#[test]
	fn t_usize_from() {
		test_impl_subrange!(usize, (u128));
		assert_eq!(u32::saturating_from(usize::MAX), u32::MAX);
		assert_eq!(u64::saturating_from(usize::MAX), u64::MAX);
		test_impl_max!(u128, (usize));
	}
}
