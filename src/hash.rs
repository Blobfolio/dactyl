/*!
# Dactyl: Hashing
*/

#![expect(clippy::cast_lossless, reason = "False positive.")]

use std::hash::{
	BuildHasherDefault,
	Hasher,
};



/// # No-Hash (Passthrough) Hash State.
///
/// Hashing can be expensive, and is totally unnecessary for most numeric or
/// pre-hashed types. (You don't need a hash to tell you that `1_u8` is
/// different than `2_u8`!)
///
/// `NoHash` is a drop in replacement for the standard library's hasher used in
/// [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet) that lets
/// the values speak for themselves (e.g. `hash(13_u16) == 13_u64`), bringing a
/// free performance boost.
///
/// This idea isn't new, but unlike the hashers offered by [`nohash`](https://crates.io/crates/nohash) or [`prehash`](https://crates.io/crates/prehash),
/// `NoHash` does not limit itself to primitives or require any custom trait
/// implementations.
///
/// It "just works" for any type whose [`std::hash::Hash`] implementation writes
/// a single <= 64-bit integer via one of the following:
/// * [`write_i8`](std::hash::Hasher::write_i8)
/// * [`write_i16`](std::hash::Hasher::write_i16)
/// * [`write_i32`](std::hash::Hasher::write_i32)
/// * [`write_i64`](std::hash::Hasher::write_i64)
/// * [`write_isize`](std::hash::Hasher::write_isize) (if the target pointer width is <= 64)
/// * [`write_u8`](std::hash::Hasher::write_u8)
/// * [`write_u16`](std::hash::Hasher::write_u16)
/// * [`write_u32`](std::hash::Hasher::write_u32)
/// * [`write_u64`](std::hash::Hasher::write_u64)
/// * [`write_usize`](std::hash::Hasher::write_usize) (if the target pointer width is <= 64)
///
/// In other words, `NoHash` can always be used for `i8`, `i16`, `i32`, `i64`,
/// `u8`, `u16`, `u32`, `u64`, all their `NonZero` and [`Wrapping`](std::num::Wrapping) counterparts,
/// and any custom types that derive their hashes from one of these types.
///
/// (`isize` and `usize` will work on most platforms too, just not those with
/// monstrous 128-bit pointer widths.)
///
/// ## Examples
///
/// ```
/// use dactyl::NoHash;
/// use std::collections::{HashMap, HashSet};
///
/// let mut set: HashSet<u32, NoHash> = HashSet::default();
/// assert!(set.insert(0_u32));
/// assert!(set.insert(1_u32));
/// assert!(set.insert(2_u32));
/// assert!(! set.insert(2_u32)); // Not unique!
///
/// let mut set: HashMap<i8, &str, NoHash> = HashMap::default();
/// assert_eq!(set.insert(-2_i8, "Hello"), None);
/// assert_eq!(set.insert(-1_i8, "World"), None);
/// assert_eq!(set.insert(0_i8, "How"), None);
/// assert_eq!(set.insert(1_i8, "Are"), None);
/// assert_eq!(set.insert(1_i8, "You?"), Some("Are")); // Not unique!
/// ```
///
/// This can also be used with custom types that implement `Hash` in such a
/// way that only a single specialized `write_*` call occurs.
///
/// ```
/// use dactyl::NoHash;
/// use std::{
///    collections::HashSet,
///    hash::{Hash, Hasher},
/// };
///
/// struct Person {
///     name: String,
///     id: u64,
/// }
///
/// impl Eq for Person {}
///
/// impl Hash for Person {
///     fn hash<H: Hasher>(&self, state: &mut H) {
///         state.write_u64(self.id);
///         // Note: `self.id.hash(state)` would also work because it just
///         // calls `write_u64` under-the-hood.
///     }
/// }
///
/// impl PartialEq for Person {
///     fn eq(&self, b: &Self) -> bool { self.id == b.id }
/// }
///
/// let mut set: HashSet<Person, NoHash> = HashSet::default();
/// assert!(set.insert(Person { name: "Jane".to_owned(), id: 5 }));
/// assert!(set.insert(Person { name: "Joan".to_owned(), id: 6 }));
/// assert!(! set.insert(Person { name: "Jack".to_owned(), id: 6 })); // Duplicate ID.
/// ```
///
/// ## Panics
///
/// `NoHash` does **not** support slices, `i128`, or `u128` as they cannot be
/// losslessly converted to `u64`. If a `Hash` implementation tries to make use
/// of those write methods, it will panic. On 128-bit platforms, attempts to hash
/// `isize` or `usize` will likewise result in a panic.
///
/// `NoHash` will also panic if a `Hash` implementation writes two or more
/// values to the hasher — as a tuple would, for example — but only for `debug`
/// builds. When building in release mode, `NoHash` will simply pass-through
/// the last integer written to it, ignoring everything else.
pub type NoHash = BuildHasherDefault<NoHasher>;



#[derive(Debug, Default, Copy, Clone)]
/// # Passthrough Hasher.
///
/// See [`NoHash`] for usage details.
pub struct NoHasher(u64);

/// # Helper: Write Method(s) for Unsigned Ints.
macro_rules! write_unsigned {
	($($fn:ident, $ty:ty),+ $(,)?) => ($(
		#[inline]
		#[doc = concat!("# Write `", stringify!($ty), "`")]
		fn $fn(&mut self, val: $ty) {
			debug_assert!(self.0 == 0, "cannot call `Hasher::write_*` more than once");
			self.0 = val as u64;
		}
	)+);
}

/// # Helper: Write Method(s) for Signed Ints.
macro_rules! write_signed {
	($($fn:ident, $ty1:ty, $ty2:ty),+ $(,)?) => ($(
		#[expect(clippy::cast_sign_loss, reason = "False positive.")]
		#[inline]
		#[doc = concat!("# Write `", stringify!($ty1), "`")]
		fn $fn(&mut self, val: $ty1) {
			debug_assert!(self.0 == 0, "cannot call `Hasher::write_*` more than once");
			self.0 = (val as $ty2) as u64;
		}
	)+);
}

impl Hasher for NoHasher {
	#[cold]
	/// # Write.
	fn write(&mut self, _bytes: &[u8]) {
		unimplemented!("NoHash only implements the type-specific write methods (like `write_u16`)");
	}

	write_unsigned!(
		write_u8, u8,
		write_u16, u16,
		write_u32, u32,
		write_usize, usize,
	);
	write_signed!(
		write_i8, i8, u8,
		write_i16, i16, u16,
		write_i32, i32, u32,
		write_isize, isize, usize,
	);

	#[inline]
	/// # Real Write.
	fn write_u64(&mut self, val: u64) { self.0 = val; }

	#[inline]
	/// # Finish.
	fn finish(&self) -> u64 { self.0 }
}



#[cfg(test)]
mod tests {
	use super::*;
	use std::{
		collections::HashSet,
		num::{
			NonZeroU8,
			Wrapping,
		},
	};

	#[test]
	fn t_nonzero() {
		// This just verifies that nonzero types hash the way they're supposed
		// to, i.e. as the underlying type.
		let mut set: HashSet<NonZeroU8, NoHash> = (1..=u8::MAX).filter_map(NonZeroU8::new).collect();
		assert_eq!(set.len(), 255);
		assert!(!set.insert(NonZeroU8::new(1).unwrap())); // Should already be there.
	}

	#[test]
	fn t_wrapping() {
		// This just verifies that Wrapping hashes its inner value directly.
		let mut set: HashSet<Wrapping<u8>, NoHash> = (0..=u8::MAX).map(Wrapping).collect();
		assert_eq!(set.len(), 256);
		assert!(!set.insert(Wrapping(0))); // Should already be there.
	}

	#[test]
	fn t_u8() {
		let mut set: HashSet<u8, NoHash> = (0..=u8::MAX).collect();
		assert_eq!(set.len(), 256);
		assert!(!set.insert(0)); // Should already be there.

		let mut set: HashSet<i8, NoHash> = (i8::MIN..=i8::MAX).collect();
		assert_eq!(set.len(), 256);
		assert!(!set.insert(0)); // Should already be there.
	}

	#[cfg(not(miri))]
	#[test]
	fn t_u16() {
		let mut set: HashSet<u16, NoHash> = (0..=u16::MAX).collect();
		assert_eq!(set.len(), 65_536);
		assert!(!set.insert(0)); // Should already be there.

		let mut set: HashSet<i16, NoHash> = (i16::MIN..=i16::MAX).collect();
		assert_eq!(set.len(), 65_536);
		assert!(!set.insert(0)); // Should already be there.
	}

	macro_rules! sanity_check_signed {
		($ty:ty) => (
			let mut set: HashSet<$ty, NoHash> = HashSet::default();
			assert_eq!(set.insert(<$ty>::MIN), true);
			assert_eq!(set.insert(-2), true);
			assert_eq!(set.insert(-1), true);
			assert_eq!(set.insert(0), true);
			assert_eq!(set.insert(1), true);
			assert_eq!(set.insert(2), true);
			assert_eq!(set.insert(<$ty>::MAX), true);
			assert_eq!(set.insert(0), false); // Should already be there.
		);
	}

	macro_rules! sanity_check_unsigned {
		($ty:ty) => (
			let mut set: HashSet<$ty, NoHash> = HashSet::default();
			assert_eq!(set.insert(0), true);
			assert_eq!(set.insert(1), true);
			assert_eq!(set.insert(2), true);
			assert_eq!(set.insert(<$ty>::MAX), true);
			assert_eq!(set.insert(0), false); // Should already be there.
		);
	}

	#[cfg(miri)]
	#[test]
	fn t_u16() {
		sanity_check_unsigned!(u16);
		sanity_check_signed!(i16);
	}

	#[test]
	fn t_u32() {
		sanity_check_unsigned!(u32);
		sanity_check_signed!(i32);
	}

	#[test]
	fn t_u64() {
		sanity_check_unsigned!(u64);
		sanity_check_signed!(i64);
	}

	#[test]
	fn t_usize() {
		sanity_check_unsigned!(usize);
		sanity_check_signed!(isize);
	}

	#[test]
	#[should_panic]
	fn t_u128() {
		let mut set: HashSet<u128, NoHash> = HashSet::default();
		set.insert(0);
	}

	#[cfg(debug_assertions)]
	#[test]
	#[should_panic]
	fn t_double_write() {
		// In debug mode, attempts to write twice will panic.
		let mut set: HashSet<(u8, u8), NoHash> = HashSet::default();
		set.insert((1_u8, 2_u8));
	}

	#[cfg(not(debug_assertions))]
	#[test]
	fn t_double_write() {
		// In non-debug mode, the last integer written is used for hashing.
		let mut set: HashSet<(u8, u8), NoHash> = HashSet::default();
		assert!(set.insert((1_u8, 2_u8)));
		assert!(set.insert((1_u8, 3_u8)));
		assert!(set.insert((0_u8, 3_u8))); // 3 appears twice.
	}

	#[test]
	#[should_panic]
	fn t_write_bytes() {
		let mut set: HashSet<&str, NoHash> = HashSet::default();
		set.insert("hello");
	}
}
