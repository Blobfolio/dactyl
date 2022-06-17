/*!
# Dactyl: Nice Percent.

Note: this module is "in development". It is subject to change, and may eventually be spun off into its own crate.
*/

use crate::NiceWrapper;



/// # Total Buffer Size.
const SIZE: usize = 7;

/// # Starting Index For Percentage Decimal.
const IDX_PERCENT_DECIMAL: usize = SIZE - 3;

/// # Zero.
const ZERO: [u8; SIZE] = [b'0', b'0', b'0', b'.', b'0', b'0', b'%'];



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
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NicePercent` has a bunch, including:
///
/// * `AsRef<[u8]>`
/// * `AsRef<str>`
/// * `Borrow<[u8]>`
/// * `Borrow<str>`
/// * `Clone`
/// * `Copy`
/// * `Default`
/// * `Deref<Target=[u8]>`
/// * `Display`
/// * `Eq` / `PartialEq`
/// * `Hash`
/// * `Ord` / `PartialOrd`
///
/// ## Note
///
/// This module is "in development". It is subject to change, and may eventually be spun off into its own crate.
pub type NicePercent = NiceWrapper<SIZE>;

impl Default for NicePercent {
	fn default() -> Self { Self::min() }
}

/// # Helper: From
///
/// This code is identical for `f32` and `f64` types.
macro_rules! nice_from {
	($($float:ty),+ $(,)?) => ($(
		impl From<$float> for NicePercent {
			#[allow(unsafe_code)]
			fn from(mut num: $float) -> Self {
				// Shortcut for overflowing values.
				if num <= 0.0 || ! num.is_normal() {
					return Self::min();
				}
				else if 1.0 <= num {
					return Self::max();
				}

				// Start with the bits we know.
				let mut out = Self {
					inner: ZERO,
					from: SIZE - 4,
				};
				let ptr = out.inner.as_mut_ptr();

				// Convert it to the kind of percent people think about.
				num *= 100.0;

				// Write the integer parts.
				let base = num.trunc() as usize;
				if 9 < base {
					out.from -= 2;
					unsafe {
						std::ptr::copy_nonoverlapping(
							crate::double(base),
							ptr.add(out.from),
							2
						);
					}
				}
				else {
					out.from -= 1;
					unsafe { std::ptr::write(ptr.add(out.from), base as u8 + b'0'); }
				}

				// Write the fraction.
				unsafe {
					std::ptr::copy_nonoverlapping(
						crate::double(<$float>::floor(num.fract() * 100.0) as usize),
						ptr.add(IDX_PERCENT_DECIMAL),
						2
					);
				}

				out
			}
		}
	)+);
}

nice_from!(f32, f64);

impl<T> TryFrom<(T, T)> for NicePercent
where T: num_traits::cast::AsPrimitive<f64> {
	type Error = ();

	/// # Percent From T/T.
	///
	/// This method is a shorthand that performs the (decimal) division of
	/// `T1 / T2` for you, then converts the result into a [`NicePercent`] if
	/// it falls between `0.0..=1.0`.
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// assert_eq!(
	///     NicePercent::from(0.5_f64),
	///     NicePercent::try_from((10_u8, 20_u8)).unwrap(),
	/// );
	/// ```
	///
	/// ## Errors
	///
	/// Conversion will fail if the enumerator is larger than the denominator,
	/// or if the denominator is zero.
	fn try_from(src: (T, T)) -> Result<Self, Self::Error> {
		crate::int_div_float(src.0, src.1)
			.map(Self::from)
			.ok_or(())
	}
}

impl NicePercent {
	#[must_use]
	/// # Minimum value.
	///
	/// This reads: `0.00%`.
	pub const fn min() -> Self {
		Self {
			inner: ZERO,
			from: SIZE - 5,
		}
	}

	#[must_use]
	/// # Maximum value.
	///
	/// This reads: `100.00%`.
	pub const fn max() -> Self {
		Self {
			inner: [b'1', b'0', b'0', b'.', b'0', b'0', b'%'],
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
