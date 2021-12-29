/*!
# Dactyl: "Nice" Integers

See the main crate documentation for details.
*/

pub(super) mod nice_u8;
pub(super) mod nice_u16;
pub(super) mod nice_u32;
pub(super) mod nice_u64;
pub(super) mod nice_percent;

use crate::DOUBLE;
use std::ptr;



#[doc(hidden)]
/// # Helper: Generic NiceU* traits.
///
/// This is not intended for use outside the crate.
macro_rules! impl_nice_int {
	($lhs:ty) => (
		impl ::std::ops::Deref for $lhs {
			type Target = [u8];
			#[inline]
			fn deref(&self) -> &Self::Target { self.as_bytes() }
		}

		$crate::macros::as_ref_borrow_cast!(
			$lhs:
				as_bytes [u8],
				as_str str,
		);

		$crate::macros::display_str!(as_str $lhs);

		impl Eq for $lhs {}

		impl ::std::hash::Hash for $lhs {
			#[inline]
			fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
				self.as_bytes().hash(state);
			}
		}

		impl Ord for $lhs {
			fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
				self.as_bytes().cmp(other.as_bytes())
			}
		}

		impl PartialEq for $lhs {
			#[inline]
			fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() }
		}

		impl PartialOrd for $lhs {
			#[inline]
			fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		/// ## Casting.
		///
		/// This section provides methods for converting instances into other
		/// types.
		///
		/// Note: this struct can also be dereferenced to `&[u8]`.
		impl $lhs {
			#[must_use]
			#[inline]
			/// # As Bytes.
			///
			/// Return the value as a byte string.
			pub fn as_bytes(&self) -> &[u8] { &self.inner[self.from..] }

			#[must_use]
			#[inline]
			/// # As Str.
			///
			/// Return the value as a string slice.
			pub fn as_str(&self) -> &str {
				unsafe { ::std::str::from_utf8_unchecked(self.as_bytes()) }
			}
		}
	);
}

#[doc(hidden)]
/// # Helper: Generic NiceU*::From<NonZero*>.
///
/// This is not intended for use outside the crate.
macro_rules! impl_nice_nonzero_int {
	($to:ty: $($from:ty),+ $(,)?) => ($(
		$crate::macros::from_cast!($to: get $from);

		impl From<Option<$from>> for $to {
			#[inline]
			fn from(src: Option<$from>) -> Self {
				src.map_or_else(Self::min, |s| Self::from(s.get()))
			}
		}
	)+);
}

pub(self) use {
	impl_nice_int,
	impl_nice_nonzero_int,
};



#[doc(hidden)]
/// # Write `u8` x 3
///
/// ## Safety
///
/// The destination pointer must have at least 3 bytes free or undefined
/// things may happen!
pub(super) unsafe fn write_u8_3(buf: *mut u8, num: usize) {
	let (div, rem) = crate::div_mod_usize(num, 100);
	let ptr = DOUBLE.as_ptr();
	ptr::copy_nonoverlapping(ptr.add((div << 1) + 1), buf, 1);
	ptr::copy_nonoverlapping(ptr.add(rem << 1), buf.add(1), 2);
}

#[doc(hidden)]
/// # Write `u8` x 2
///
/// ## Safety
///
/// The destination pointer must have at least 2 bytes free or undefined
/// things may happen!
pub(super) unsafe fn write_u8_2(buf: *mut u8, num: usize) {
	ptr::copy_nonoverlapping(DOUBLE.as_ptr().add(num << 1), buf, 2);
}

#[doc(hidden)]
/// # Write `u8` x 1
///
/// ## Safety
///
/// The destination pointer must have at least 1 byte free or undefined
/// things may happen!
pub(super) unsafe fn write_u8_1(buf: *mut u8, num: usize) {
	ptr::copy_nonoverlapping(DOUBLE.as_ptr().add((num << 1) + 1), buf, 1);
}
