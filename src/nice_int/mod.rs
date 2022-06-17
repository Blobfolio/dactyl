/*!
# Dactyl: "Nice" Integers

See the main crate documentation for details.
*/

pub(super) mod nice_u8;
pub(super) mod nice_u16;
pub(super) mod nice_u32;
pub(super) mod nice_u64;
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
			fn as_ref(&self) -> &$ty { self.$cast() }
		}
		impl<const S: usize> ::std::borrow::Borrow<$ty> for NiceWrapper<S> {
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

as_ref_borrow_cast!(as_bytes [u8], as_str str);

impl<const S: usize> fmt::Debug for NiceWrapper<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(&format!("NiceWrapper<{}>", S))
			.field(&self.as_str())
			.finish()
	}
}

impl<const S: usize> Deref for NiceWrapper<S> {
	type Target = [u8];
	fn deref(&self) -> &Self::Target { self.as_bytes() }
}

impl<const S: usize> fmt::Display for NiceWrapper<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl<const S: usize> Eq for NiceWrapper<S> {}

impl<const S: usize> From<NiceWrapper<S>> for String {
	fn from(src: NiceWrapper<S>) -> Self { src.as_str().to_owned() }
}

impl<const S: usize> From<NiceWrapper<S>> for Vec<u8> {
	fn from(src: NiceWrapper<S>) -> Self { src.as_bytes().to_vec() }
}

impl<const S: usize, T> From<Option<T>> for NiceWrapper<S>
where Self: From<T> + Default {
	/// `None` is treated like zero, otherwise this will simply unwrap the
	/// inner value and run `From` against that.
	fn from(num: Option<T>) -> Self { num.map_or_else(Self::default, Self::from) }
}

impl<const S: usize> Hash for NiceWrapper<S> {
	fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_bytes()) }
}

impl<const S: usize> Ord for NiceWrapper<S> {
	fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) }
}

impl<const S: usize> PartialEq for NiceWrapper<S> {
	fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() }
}

impl<const S: usize> PartialOrd for NiceWrapper<S> {
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

	#[allow(unsafe_code)]
	#[must_use]
	#[inline]
	/// # As Str.
	///
	/// Return the value as a string slice.
	pub fn as_str(&self) -> &str {
		// Safety: numbers are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}



#[doc(hidden)]
/// # Helper: From<nonzero>
macro_rules! nice_from_nz {
	($nice:ty, $($nz:ty),+ $(,)?) => ($(
		impl From<$nz> for $nice {
			fn from(num: $nz) -> Self { Self::from(num.get()) }
		}
	)+);
}

#[doc(hidden)]
/// # Helper: Default and Min.
macro_rules! nice_default {
	($nice:ty, $zero:expr, $size:ident) => (
		impl Default for $nice {
			fn default() -> Self { Self { inner: $zero, from: $size - 1 } }
		}

		impl $nice {
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
			fn from(num: $uint) -> Self {
				let mut out = Self::empty();
				out.parse(num);
				out
			}
		}

		impl $nice {
			#[allow(clippy::cast_possible_truncation, unsafe_code)]
			/// # Parse.
			fn parse(&mut self, mut num: $uint) {
				let ptr = self.inner.as_mut_ptr();

				while 999 < num {
					let (div, rem) = crate::div_mod(num, 1000);
					self.from -= 4;
					unsafe { super::write_u8_3(ptr.add(self.from + 1), rem as u16); }
					num = div;
				}

				if 99 < num {
					self.from -= 3;
					unsafe { super::write_u8_3(ptr.add(self.from), num as u16); }
				}
				else if 9 < num {
					self.from -= 2;
					unsafe {
						std::ptr::copy_nonoverlapping(
							crate::double_prt(num as usize),
							ptr.add(self.from),
							2
						);
					}
				}
				else {
					self.from -= 1;
					unsafe { std::ptr::write(ptr.add(self.from), num as u8 + b'0'); }
				}
			}
		}
	);
}

pub(self) use {
	nice_default,
	nice_from_nz,
	nice_parse,
};



#[allow(clippy::cast_possible_truncation)] // One digit always fits u8.
#[allow(unsafe_code)]
/// # Write `u8` x 3
///
/// ## Safety
///
/// The destination pointer must have at least 3 bytes free or undefined
/// things may happen!
unsafe fn write_u8_3(buf: *mut u8, num: u16) {
	let (div, rem) = crate::div_mod(num, 100);
	std::ptr::write(buf, div as u8 + b'0');
	std::ptr::copy_nonoverlapping(crate::double_prt(rem as usize), buf.add(1), 2);
}
