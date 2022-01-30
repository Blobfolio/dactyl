#![allow(deprecated)]



#[deprecated(since = "0.2.4", note="just test for `T > 0` manually")]
/// # Greater Than Zero
///
/// This trait exposes methods for determining whether or not a type is greater
/// than zero. For floats, this implies not `NAN` and not `NEG_INFINITY` too.
///
/// This is automatically implemented for all Rust primitive integer and float
/// types.
pub trait GtZero: Copy + PartialEq + PartialOrd {
	/// # Zero.
	///
	/// The zero equivalent of `Self`.
	const GTZERO_ZERO: Self;

	#[inline]
	/// # Is Greater Than Zero.
	///
	/// This method returns `true` if the value is greater than zero.
	fn is_gt_zero(self) -> bool { self > Self::GTZERO_ZERO }
}

/// # Helper: Implement `GtZero` For Unsigned Integers.
macro_rules! impl_gtzero {
	($(($from:ty, $zero:literal)),+) => (
		$( impl GtZero for $from { const GTZERO_ZERO: Self = $zero; } )+
	);

	($($from:ty),+) => (
		$( impl GtZero for $from { const GTZERO_ZERO: Self = 0; } )+
	);
}

impl_gtzero!(
	u8, u16, u32, u64, u128, usize,
	i8, i16, i32, i64, i128, isize
);

impl_gtzero!((f32, 0.0), (f64, 0.0));
