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

impl<const S: usize> AsRef<str> for NiceWrapper<S> {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl<const S: usize> ::std::borrow::Borrow<str> for NiceWrapper<S> {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

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
		<str as fmt::Display>::fmt(self.as_str(), f)
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
	pub const fn as_bytes(&self) -> &[u8] {
		let (_, out) = self.inner.as_slice().split_at(self.from);
		out
	}

	#[expect(unsafe_code, reason = "Content is UTF-8.")]
	#[must_use]
	#[inline]
	/// # As Str.
	///
	/// Return the value as a string slice.
	pub const fn as_str(&self) -> &str {
		debug_assert!(
			std::str::from_utf8(self.as_bytes()).is_ok(),
			"BUG: NiceWrapper is not UTF-8?!",
		);

		// Safety: values are always ASCII, except for NiceFloat::INFINITY.
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
/// # Helper: Default/Empty.
///
/// This is shared by the various `NiceU*` types to generate appropriate
/// `Self::default` and `Self::empty` helpers.
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

use nice_default;
