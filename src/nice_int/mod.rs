/*!
# Dactyl: "Nice" Unsigned Integers (and Floats)

See the main crate documentation for details.
*/

pub(super) mod nice_u8;
pub(super) mod nice_u16;
pub(super) mod nice_u32;
pub(super) mod nice_u64;
pub(super) mod nice_float;
pub(super) mod nice_percent;



use std::{
	cmp::Ordering,
	fmt,
	hash::{
		Hash,
		Hasher,
	},
	ops::Deref,
};



/// # Helper: `AsRef` and `Borrow`.
macro_rules! as_ref_borrow_cast {
	($($cast:ident $ty:ty),+ $(,)?) => ($(
		impl<const S: usize> AsRef<$ty> for NiceWrapper<S> {
			#[inline]
			fn as_ref(&self) -> &$ty { self.$cast() }
		}
		impl<const S: usize> ::std::borrow::Borrow<$ty> for NiceWrapper<S> {
			#[inline]
			fn borrow(&self) -> &$ty { self.$cast() }
		}
	)+);
}



#[doc(hidden)]
#[derive(Clone, Copy)]
/// # Nice Unsigned.
///
/// This is the master struct for [`NiceU16`](crate::NiceU16), [`NiceU32`](crate::NiceU32), etc.
/// Don't use this directly. Use the type aliases instead.
pub struct NiceWrapper<const S: usize> {
	pub(crate) inner: [u8; S],
	pub(crate) from: usize,
}

impl<const S: usize> AsRef<[u8]> for NiceWrapper<S> {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

as_ref_borrow_cast!(as_str str);

impl<const S: usize> fmt::Debug for NiceWrapper<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(&format!("NiceWrapper<{S}>"))
			.field(&self.as_str())
			.finish()
	}
}

impl<const S: usize> Deref for NiceWrapper<S> {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_bytes() }
}

impl<const S: usize> fmt::Display for NiceWrapper<S> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl<const S: usize> Eq for NiceWrapper<S> {}

impl<const S: usize> From<NiceWrapper<S>> for String {
	#[inline]
	fn from(src: NiceWrapper<S>) -> Self { src.as_str().to_owned() }
}

impl<const S: usize> From<NiceWrapper<S>> for Vec<u8> {
	#[inline]
	fn from(src: NiceWrapper<S>) -> Self { src.as_bytes().to_vec() }
}

impl<const S: usize, T> From<Option<T>> for NiceWrapper<S>
where Self: From<T> + Default {
	#[inline]
	/// `None` is treated like zero, otherwise this will simply unwrap the
	/// inner value and run `From` against that.
	fn from(num: Option<T>) -> Self { num.map_or_else(Self::default, Self::from) }
}

impl<const S: usize> Hash for NiceWrapper<S> {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_bytes()) }
}

impl<const S: usize> Ord for NiceWrapper<S> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) }
}

impl<const S: usize> PartialEq for NiceWrapper<S> {
	#[inline]
	fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() }
}

impl<const S: usize> PartialOrd for NiceWrapper<S> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

/// ## Casting.
///
/// This section provides methods for converting instances into other types.
///
/// Note: this can also be dereferenced to a slice, or `AsRef`ed to a slice or
/// string slice.
impl<const S: usize> NiceWrapper<S> {
	#[must_use]
	#[inline]
	/// # As Bytes.
	///
	/// Return the value as a byte string.
	pub fn as_bytes(&self) -> &[u8] { &self.inner[self.from..] }

	#[allow(unsafe_code)] // Content is ASCII.
	#[must_use]
	#[inline]
	/// # As Str.
	///
	/// Return the value as a string slice.
	pub fn as_str(&self) -> &str {
		debug_assert!(std::str::from_utf8(self.as_bytes()).is_ok(), "NiceWrapper is not UTF.");
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}

	#[must_use]
	/// # Is Empty?
	pub const fn is_empty(&self) -> bool { S <= self.from }

	#[must_use]
	/// # Length.
	pub const fn len(&self) -> usize { S.wrapping_sub(self.from) }
}



#[doc(hidden)]
/// # Helper: From<nonzero>
macro_rules! nice_from_nz {
	($nice:ty, $($nz:ty),+ $(,)?) => ($(
		impl From<$nz> for $nice {
			#[inline]
			fn from(num: $nz) -> Self { Self::from(num.get()) }
		}
	)+);
}

#[doc(hidden)]
/// # Helper: Default and Min.
macro_rules! nice_default {
	($nice:ty, $zero:expr, $size:ident) => (
		impl Default for $nice {
			#[inline]
			fn default() -> Self { Self { inner: $zero, from: $size - 1 } }
		}

		impl $nice {
			#[inline]
			#[doc(hidden)]
			#[must_use]
			/// # Empty.
			///
			/// This returns an empty object.
			pub const fn empty() -> Self { Self { inner: $zero, from: $size } }
		}
	);
}

#[doc(hidden)]
/// # Helper: Generic From/Parsing (u32 and larger).
macro_rules! nice_parse {
	($nice:ty, $uint:ty) => (
		impl From<$uint> for $nice {
			#[inline]
			fn from(num: $uint) -> Self {
				let mut out = Self::empty();
				out.parse(num);
				out
			}
		}

		impl $nice {
			#[allow(clippy::cast_possible_truncation)] // False positive.
			/// # Parse.
			fn parse(&mut self, mut num: $uint) {
				for chunk in self.inner.rchunks_exact_mut(4) {
					if 999 < num {
						let rem = num % 1000;
						num /= 1000;
						chunk[1..].copy_from_slice(crate::triple(rem as usize).as_slice());
						self.from -= 4;
					}
					else { break; }
				}

				if 99 < num {
					self.from -= 3;
					self.inner[self.from..self.from + 3].copy_from_slice(
						crate::triple(num as usize).as_slice()
					);
				}
				else if 9 < num {
					self.from -= 2;
					self.inner[self.from..self.from + 2].copy_from_slice(
						crate::double(num as usize).as_slice()
					);
				}
				else {
					self.from -= 1;
					self.inner[self.from] = num as u8 + b'0';
				}
			}
		}
	);
}

use {
	nice_default,
	nice_from_nz,
	nice_parse,
};
