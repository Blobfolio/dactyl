/*!
# Dactyl: Greater-Than-Zero
*/

use crate::traits::GtZero;
use std::{
	cmp::Ordering,
	fmt,
	hash,
	num::{
		NonZeroU8,
		NonZeroU16,
		NonZeroU32,
		NonZeroU64,
		NonZeroU128,
		NonZeroUsize,
		NonZeroI8,
		NonZeroI16,
		NonZeroI32,
		NonZeroI64,
		NonZeroI128,
		NonZeroIsize,
	},
	ops::{
		Add,
		AddAssign,
		Deref,
		DerefMut,
		Div,
		DivAssign,
		Mul,
		MulAssign,
		Rem,
		RemAssign,
		Sub,
		SubAssign,
	},
	str::FromStr,
};



#[derive(Debug, Copy, Clone)]
/// # Greater Than Zero
///
/// [`GreaterThanZero`] is a memory-saving replacement for `Option<T>` where
/// `T` is only "some" if it is `> 0` (and not `NAN` or `NEG_INFINITY`). In
/// other words, `GreaterThanZero<T>` has the same size as `T`.
///
/// It is similar to the `std::num::NonZeroX` types, except it is its own
/// `Option` so does not need to be wrapped (it is infallible). Instead of
/// `is_some`, you'd use [`GreaterThanZero::is_gt_zero`].
///
/// To obtain the underlying primitive, you can use [`GreaterThanZero::get`] to
/// ensure the value is "some", or [`GreaterThanZero::get_unchecked`] to
/// return the value regardless of zeroness. (Dereferencing also returns the
/// unchecked value, albeit as a reference.)
///
/// Non-qualifying values are stored regardless and can be obtained by
/// dereferencing or [`GreaterThanZero::get_unchecked`].
///
/// Other neat things: this struct implements `Add`, `Div`, `Mul`, `Rem`,
/// `Sub`, and the corresponding `*Assign` traits, so you can manipulate
/// values in place. Zeroness is always evaluated in realtime, so this works
/// even in cases where the result crosses that line.
///
/// It also implements `Option`-like methods [`GreaterThanZero::filter`],
/// [`GreaterThanZero::map`], [`GreaterThanZero::replace`], and
/// [`GreaterThanZero::zip`].
///
/// ## Examples
///
/// ```no_run
/// use dactyl::GreaterThanZero;
///
/// let gt0 = GreaterThanZero::from(-3_isize);
/// assert!(! gt0.is_gt_zero());
/// assert_eq!(gt0.get(), None);
/// assert_eq!(gt0.get_unchecked(), -3_isize);
///
/// let gt1 = GreaterThanZero::from(5.5_f64);
/// assert!(gt1.is_gt_zero());
/// assert_eq!(gt1.get(), Some(5.5_f64));
/// assert_eq!(gt1.get_unchecked(), 5.5_f64);
/// ```
pub struct GreaterThanZero<T>(T)
where T: GtZero;

/// # Helper: Math Impls (`Add`, `Sub`, etc.)
macro_rules! impl_math {
	($(($trait:ident, $method:ident, $op:tt)),+) => (
		$(
			impl<T: GtZero + $trait<Output = T>> $trait for GreaterThanZero<T> {
				type Output = Self;

				#[inline]
				fn $method(self, other: Self) -> Self {
					Self(self.0 $op other.0)
				}
			}
		)+
	);
}

impl_math!(
	(Add, add, +),
	(Div, div, /),
	(Mul, mul, *),
	(Rem, rem, %),
	(Sub, sub, -)
);

/// # Helper: Math Assignment Impls (`AddAssign`, `SubAssign`, etc.)
macro_rules! impl_math_assign {
	($(($trait:ident, $method:ident, $op:tt)),+) => (
		$(
			impl<T: GtZero + $trait> $trait for GreaterThanZero<T> {
				#[inline]
				fn $method(&mut self, other: Self) {
					self.0 $op other.0;
				}
			}
		)+
	);
}

impl_math_assign!(
	(AddAssign, add_assign, +=),
	(DivAssign, div_assign, /=),
	(MulAssign, mul_assign, *=),
	(RemAssign, rem_assign, %=),
	(SubAssign, sub_assign, -=)
);

impl<T: GtZero> Default for GreaterThanZero<T> {
	#[inline]
	fn default() -> Self { Self(T::GTZERO_ZERO) }
}

impl<T: GtZero> Deref for GreaterThanZero<T> {
	type Target = T;
	#[inline]
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: GtZero> DerefMut for GreaterThanZero<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: GtZero + fmt::Display> fmt::Display for GreaterThanZero<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl<T: GtZero> Eq for GreaterThanZero<T> {}

impl<T: GtZero + FromStr> From<&str> for GreaterThanZero<T> {
	#[inline]
	fn from(src: &str) -> Self { Self(T::from_str(src).unwrap_or(T::GTZERO_ZERO)) }
}

impl<T: GtZero + hash::Hash> hash::Hash for GreaterThanZero<T> {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<T: GtZero + Ord> Ord for GreaterThanZero<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
}

impl<T: GtZero> PartialEq for GreaterThanZero<T> {
	#[inline]
	fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T: GtZero> PartialEq<T> for GreaterThanZero<T> {
	#[inline]
	fn eq(&self, other: &T) -> bool { &self.0 == other }
}

impl<T: GtZero> PartialOrd for GreaterThanZero<T> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.0.partial_cmp(other)
	}
}

impl<T: GtZero> From<T> for GreaterThanZero<T> {
	#[inline]
	fn from(src: T) -> Self { Self(src) }
}

impl<T: GtZero> From<GreaterThanZero<T>> for Option<T> {
	#[inline]
	fn from(src: GreaterThanZero<T>) -> Self { src.get() }
}

/// # Helper: Implement `From<NonZero*> For GreaterThanZero<*>`.
macro_rules! from_nonzero {
	($(($from:ty, $to:ty)),+) => (
		$(
			impl From<$from> for GreaterThanZero<$to> {
				#[inline]
				fn from(src: $from) -> Self { Self(src.get()) }
			}
		)+
	);
}

from_nonzero!(
	(NonZeroU8, u8),
	(NonZeroU16, u16),
	(NonZeroU32, u32),
	(NonZeroU64, u64),
	(NonZeroU128, u128),
	(NonZeroUsize, usize),
	(NonZeroI8, i8),
	(NonZeroI16, i16),
	(NonZeroI32, i32),
	(NonZeroI64, i64),
	(NonZeroI128, i128),
	(NonZeroIsize, isize)
);

#[allow(clippy::use_self)] // Self<U> doesn't work here.
impl<T: GtZero> GreaterThanZero<T> {
	/// # Filter.
	///
	/// Like [`std::option::Option::filter`], this will evaluate the instance
	/// value with your callback, leaving it as-was if `true`, or zeroing it
	/// out if `false`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::GreaterThanZero;
	///
	/// // This filter is not applied as the value is not > 0.
	/// assert_eq!(
	///     GreaterThanZero::from(-3_isize).filter(|x| x != 4),
	///     GreaterThanZero::from(-3_isize)
	/// );
	///
	/// // This filter is applied and fails, zeroing the value.
	/// assert_eq!(
	///     GreaterThanZero::from(4_u8).filter(|x| x != 4),
	///     GreaterThanZero::from(0_u8)
	/// );
	///
	/// // This filter is applied and passes, keeping the original value.
	/// assert_eq!(
	///     GreaterThanZero::from(32_u32).filter(|x| x != 4),
	///     GreaterThanZero::from(32_u32)
	/// );
	/// ```
	pub fn filter<F>(self, f: F) -> Self
	where F: FnOnce(T) -> bool {
		if self.0.is_gt_zero() && ! f(self.0) {
			Self(T::GTZERO_ZERO)
		}
		else { self }
	}

	#[inline]
	/// # Get Inner Value.
	///
	/// This will return `Some(T)` if the value is greater than zero, otherwise
	/// `None`.
	pub fn get(self) -> Option<T> {
		if self.is_gt_zero() { Some(self.0) }
		else { None }
	}

	/// # Get Unchecked.
	///
	/// This will return the inner value regardless of its greater-than-
	/// zeroness. This is equivalent to dereferencing with copy.
	pub fn get_unchecked(self) -> T { self.0 }

	#[inline]
	/// # Is Greater Than Zero.
	///
	/// This will return `true` if the value is `> 0`, not `NAN`, and not
	/// `NEG_INFINITY`.
	pub fn is_gt_zero(&self) -> bool { self.0.is_gt_zero() }

	/// # Map.
	///
	/// Operate on the inner value if it is greater than zero, otherwise do
	/// nothing. This is equivalent to [`std::option::Option::map`], except
	/// that the returned value can be less than or equal to zero (our
	/// equivalent of `None`).
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::GreaterThanZero;
	///
	/// // Turn 4 to 8.
	/// assert_eq!(
	///     GreaterThanZero::from(4_u8).map(|_| 8),
	///     GreaterThanZero::from(8_u8)
	/// );
	/// ```
	pub fn map<F>(self, f: F) -> Self
	where F: FnOnce(T) -> T {
		if self.0.is_gt_zero() {
			Self(f(self.0))
		}
		else { self }
	}

	#[inline]
	/// # Replace.
	///
	/// This is a convenience method allowing the inner value to be replaced
	/// with any arbitrary value. As this type is `Copy`, you could avoid the
	/// `&mut` and just reassign.
	pub fn replace(&mut self, b: T) { self.0 = b; }

	/// # Zip.
	///
	/// Produce a tuple of the inner value of A and B if both are greater than
	/// zero. This is comparable to [`std::option::Option::zip`] except that
	/// the inner types of A and B are allowed to be different.
	pub fn zip<U: GtZero>(self, b: GreaterThanZero<U>) -> Option<(T, U)> {
		if self.is_gt_zero() && b.is_gt_zero() { Some((self.0, b.0)) }
		else { None }
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_zeroness() {
		macro_rules! test_min_max {
			($($type:ty),+) => {
				$(
					assert!(! GreaterThanZero::from(<$type>::MIN).is_gt_zero());
					assert!(GreaterThanZero::from(<$type>::MAX).is_gt_zero());
				)+
			};
		}

		test_min_max!(
			u8, u16, u32, u64, u128, usize,
			i8, i16, i32, i64, i128, isize,
			f32, f64
		);

		assert!(GreaterThanZero::from(f32::INFINITY).is_gt_zero());
		assert!(GreaterThanZero::from(f64::INFINITY).is_gt_zero());

		assert!(! GreaterThanZero::from(f32::NEG_INFINITY).is_gt_zero());
		assert!(! GreaterThanZero::from(f64::NEG_INFINITY).is_gt_zero());

		assert!(! GreaterThanZero::from(f32::NAN).is_gt_zero());
		assert!(! GreaterThanZero::from(f64::NAN).is_gt_zero());
	}

	#[test]
	fn t_ops() {
		assert_eq!(
			GreaterThanZero::from(2_u32) + GreaterThanZero::from(3_u32),
			GreaterThanZero::from(5_u32)
		);

		assert_eq!(
			GreaterThanZero::from(3_u32) - GreaterThanZero::from(2_u32),
			GreaterThanZero::from(1_u32)
		);

		assert_eq!(
			GreaterThanZero::from(2_u32) * GreaterThanZero::from(3_u32),
			GreaterThanZero::from(6_u32)
		);

		assert_eq!(
			GreaterThanZero::from(6_u32) / GreaterThanZero::from(3_u32),
			GreaterThanZero::from(2_u32)
		);

		assert_eq!(
			GreaterThanZero::from(10_u32) % GreaterThanZero::from(3_u32),
			GreaterThanZero::from(1_u32)
		);
	}
}
