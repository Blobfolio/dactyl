/*!
# Dactyl: Nice u32.
*/

use std::num::NonZeroU32;



/// # Total Buffer Size.
const SIZE: usize = 13;



#[derive(Debug, Clone, Copy, Hash, PartialEq)]
/// `NiceU32` provides a quick way to convert a `u32` into a formatted byte
/// string for e.g. printing. Commas are added for every thousand.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceU32;
/// assert_eq!(
///     NiceU32::from(33231).as_str(),
///     "33,231"
/// );
/// ```
pub struct NiceU32 {
	inner: [u8; SIZE],
	from: usize,
}

impl Default for NiceU32 {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE,
		}
	}
}

impl From<u32> for NiceU32 {
	fn from(mut num: u32) -> Self {
		let mut out = Self::default();
		let ptr = out.inner.as_mut_ptr();

		while num >= 1000 {
			let (div, rem) = crate::div_mod_u32(num, 1000);
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

impl NiceU32 {
	#[must_use]
	#[inline]
	/// # Min.
	///
	/// This is equivalent to zero.
	pub const fn min() -> Self {
		Self {
			inner: [b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0', b',', b'0', b'0', b'0'],
			from: SIZE - 1,
		}
	}
}

// A few Macro traits.
crate::impl_nice_nonzero_int!(NonZeroU32, NiceU32);
crate::impl_nice_int!(NiceU32);



#[cfg(test)]
mod tests {
	use super::*;
	use num_format::{ToFormattedString, Locale};

	#[test]
	fn t_nice_u32() {
		// Check the min and max.
		assert_eq!(NiceU32::from(0).as_str(), "0");
		assert_eq!(NiceU32::min(), NiceU32::from(0));
		assert_eq!(
			NiceU32::from(u32::MAX).as_str(),
			u32::MAX.to_formatted_string(&Locale::en),
		);

		// Check a subset of everything else.
		let mut step = 1_u32;
		let mut i = 0_u32;
		loop {
			for _ in 0..10 {
				if u32::MAX - i < step { return; }
				i += step;
				assert_eq!(
					NiceU32::from(i).as_str(),
					i.to_formatted_string(&Locale::en),
				);
			}

			step *= 10;
		}
	}


	#[test]
	fn t_nice_nonzero_u32() {
		assert_eq!(NiceU32::min(), NiceU32::from(NonZeroU32::new(0)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50)));
		assert_eq!(NiceU32::from(50_u32), NiceU32::from(NonZeroU32::new(50).unwrap()));
	}
}
