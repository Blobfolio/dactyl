/*!
# Dactyl: Nice u64.
*/

use std::num::{
	NonZeroU64,
	NonZeroUsize,
};



/// # Total Buffer Size.
const SIZE: usize = 26;



#[derive(Debug, Clone, Copy, Hash, PartialEq)]
/// `NiceU64` provides a quick way to convert a `u64` into a formatted byte
/// string for e.g. printing. Commas are added for every thousand.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU64;
/// assert_eq!(
///     NiceU64::from(33231_u64).as_str(),
///     "33,231"
/// );
/// ```
pub struct NiceU64 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU64 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE,
		}
	}
}

impl From<usize> for NiceU64 {
	fn from(mut num: usize) -> Self {
		let mut out = Self::default();
		let ptr = out.inner.as_mut_ptr();

		while num >= 1000 {
			let (div, rem) = num_integer::div_mod_floor(num, 1000);
			unsafe { super::write_u8_3(ptr.add(out.from - 3), rem); }
			num = div;
			out.from -= 4;
		}

		if num >= 100 {
			out.from -= 3;
			unsafe { super::write_u8_3(ptr.add(out.from), num); }
		}
		else if num >= 10 {
			out.from -= 2;
			unsafe { super::write_u8_2(ptr.add(out.from), num); }
		}
		else {
			out.from -= 1;
			unsafe { super::write_u8_1(ptr.add(out.from), num); }
		}

		out
	}
}

impl From<u64> for NiceU64 {
	#[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
	#[allow(clippy::cast_possible_truncation)] // We've already asserted pointer widths.
	fn from(num: u64) -> Self {
		// Skip all the index casts.
		Self::from(num as usize)
	}

	#[cfg(not(target_pointer_width = "64"))]
	fn from(mut num: u64) -> Self {
		let mut out = Self::default();
		let ptr = out.inner.as_mut_ptr();

		while num >= 1000 {
			let (div, rem) = num_integer::div_mod_floor(num, 1000);
			unsafe { super::write_u8_3(ptr.add(out.from - 3), rem as usize); }
			num = div;
			out.from -= 4;
		}

		if num >= 100 {
			out.from -= 3;
			unsafe { super::write_u8_3(ptr.add(out.from), num as usize); }
		}
		else if num >= 10 {
			out.from -= 2;
			unsafe { super::write_u8_2(ptr.add(out.from), num as usize); }
		}
		else {
			out.from -= 1;
			unsafe { super::write_u8_1(ptr.add(out.from), num as usize); }
		}

		out
	}
}

impl NiceU64 {
	#[must_use]
	#[inline]
	/// # Min.
	///
	/// This is equivalent to zero.
	pub const fn min() -> Self {
		Self {
			inner: [b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE - 1,
		}
	}
}

// A few Macro traits.
crate::impl_nice_nonzero_int!(NonZeroU64, NiceU64);
crate::impl_nice_nonzero_int!(NonZeroUsize, NiceU64);
crate::impl_nice_int!(NiceU64);



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u64() {
		// Check the min and max.
		assert_eq!(NiceU64::from(0_u64).as_str(), "0");
		assert_eq!(NiceU64::min(), NiceU64::from(0_u64));
		assert_eq!(NiceU64::min(), NiceU64::from(0_usize));
		assert_eq!(
			NiceU64::from(u64::MAX).as_str(),
			u64::MAX.to_formatted_string(&Locale::en),
		);

		// Check a subset of everything else.
		let mut step = 1_u64;
		let mut i = 0_u64;
		loop {
			for _ in 0..10 {
				if u64::MAX - i < step { return; }
				i += step;
				assert_eq!(
					NiceU64::from(i).as_str(),
					i.to_formatted_string(&Locale::en),
				);
			}

			step *= 10;
		}
	}

	#[test]
	fn t_nice_nonzero_u64() {
		assert_eq!(NiceU64::min(), NiceU64::from(NonZeroU64::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroU64::new(50).unwrap()));

		assert_eq!(NiceU64::min(), NiceU64::from(NonZeroUsize::new(0)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50)));
		assert_eq!(NiceU64::from(50_u64), NiceU64::from(NonZeroUsize::new(50).unwrap()));
	}
}
