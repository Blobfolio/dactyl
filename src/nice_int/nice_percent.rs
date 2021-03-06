/*!
# Dactyl: Nice Percent.

Note: this module is "in development". It is subject to change, and may eventually be spun off into its own crate.
*/



/// # Total Buffer Size.
const SIZE: usize = 7;

/// # Starting Index For Percentage Decimal.
const IDX_PERCENT_DECIMAL: usize = SIZE - 3;



#[derive(Debug, Clone, Copy, Hash, PartialEq)]
/// `NicePercent` provides a quick way to convert an `f32` or `f64` into a
/// formatted byte string for e.g. printing.
///
/// The precision is fixed at two decimal places — truncated, not rounded —
/// with values ranging from `0.00%` to `100.00%`.
///
/// Inputs are expected to be in `0..=1`. Values less than zero are cast to
/// `0.00%`, while values greater than `1` are cast to `100.00%`.
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NicePercent;
/// assert_eq!(NicePercent::from(0.321).as_str(), "32.10%");
/// ```
///
/// ## Note
///
/// This module is "in development". It is subject to change, and may eventually be spun off into its own crate.
pub struct NicePercent {
	inner: [u8; SIZE],
	from: usize,
}

crate::impl_nice_int!(NicePercent);

impl Default for NicePercent {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [0, 0, 0, b'.', 0, 0, b'%'],
			from: SIZE,
		}
	}
}

/// # Helper: From
///
/// This code is identical for `f32` and `f64` types.
macro_rules! impl_from {
	($type:ty) => {
		impl From<$type> for NicePercent {
			fn from(mut num: $type) -> Self {
				// Shortcut for overflowing values.
				if num <= 0.0 || ! num.is_normal() {
					return Self::min();
				}
				else if 1.0 <= num {
					return Self::max();
				}

				// Start with the bits we know.
				let mut out = Self {
					inner: *b"000.00%",
					from: SIZE - 4,
				};
				let ptr = out.inner.as_mut_ptr();

				// Convert it to the kind of percent people think about.
				num *= 100.0;

				// Write the integer parts.
				let base = num.trunc() as usize;
				if base >= 10 {
					out.from -= 2;
					unsafe { super::write_u8_2(ptr.add(out.from), base); }
				}
				else {
					out.from -= 1;
					unsafe { super::write_u8_1(ptr.add(out.from), base); }
				}

				// Write the fraction.
				unsafe {
					super::write_u8_2(
						ptr.add(IDX_PERCENT_DECIMAL),
						<$type>::floor(num.fract() * 100.0) as usize
					);
				}

				out
			}
		}
	};
}

impl_from!(f32);
impl_from!(f64);

impl NicePercent {
	#[must_use]
	/// # Minimum value.
	///
	/// This reads: `0.00%`.
	pub const fn min() -> Self {
		Self {
			inner: *b"000.00%",
			from: SIZE - 5,
		}
	}

	#[must_use]
	/// # Maximum value.
	///
	/// This reads: `100.00%`.
	pub const fn max() -> Self {
		Self {
			inner: *b"100.00%",
			from: SIZE - 7,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_percent() {
		for i in 0..1_000 {
			let fraction = i as f32 / 1000_f32;
			let num = fraction * 100_f32;
			let base = f32::floor(num);

			assert_eq!(
				NicePercent::from(fraction).as_str(),
				format!("{}.{:02}%", base, f32::floor((num - base) * 100_f32)),
			);
		}

		// And a few edge cases.
		assert_eq!(NicePercent::from(0_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(-10_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(1.03_f64).as_str(), "100.00%");
		assert_eq!(NicePercent::from(10_f64).as_str(), "100.00%");
	}
}
