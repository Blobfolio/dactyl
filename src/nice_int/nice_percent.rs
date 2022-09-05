/*!
# Dactyl: Nice Percent.
*/

use crate::NiceWrapper;



/// # Total Buffer Size.
const SIZE: usize = 7;

/// # Starting Index For Percentage Decimal.
const IDX_PERCENT_DECIMAL: usize = SIZE - 3;

/// # Zero.
const ZERO: [u8; SIZE] = [b'0', b'0', b'0', b'.', b'0', b'0', b'%'];



/// `NicePercent` provides a quick way to convert an `f32` or `f64` percent
/// — a value `0.0..=1.0` — into a formatted byte string for e.g. printing.
///
/// The precision is fixed at two decimal places (rounded at the thousandth),
/// with output ranging from `0.00%` to `100.00%`.
///
/// Inputs are expected to be in `0..=1`. Values less than zero are clamped to
/// `0.00%`, while values greater than `1` are clamped to `100.00%`.
///
/// For other types of floats, see [`NiceFloat`](crate::NiceFloat) instead.
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
			fn from(num: $float) -> Self {
				// Shortcut for overflowing values.
				if num <= 0.0 || ! num.is_normal() { return Self::min(); }
				else if 1.0 <= num { return Self::max(); }

				// We can maintain precision better by working from an integer.
				// We know there is no existing integer part, so at most we'll
				// wind up with four digits, which fits nicely in a u16.
				let whole = (num * 10_000.0).round() as u16;

				// Recheck the boundaries because of the rounding.
				if whole == 0 { return Self::min(); }
				else if 9999 < whole { return Self::max(); }

				// Start with 0.00%.
				let mut out = Self::min();
				let ptr = out.inner.as_mut_ptr();

				// Split the top and bottom.
				let (top, bottom) = crate::div_mod(whole, 100);

				// Write the integer part.
				if 9 < top {
					out.from -= 1;
					unsafe {
						std::ptr::copy_nonoverlapping(
							crate::double_ptr(top as usize),
							ptr.add(out.from),
							2
						);
					}
				}
				else if 0 < top {
					unsafe {
						std::ptr::write(ptr.add(out.from), top as u8 + b'0');
					}
				}

				// Write the fractional part.
				if 0 < bottom {
					unsafe {
						std::ptr::copy_nonoverlapping(
							crate::double_ptr(bottom as usize),
							ptr.add(IDX_PERCENT_DECIMAL),
							2
						);
					}
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
		const TOTAL: u32 = 10_000_u32;

		// There will be disagreements with a denominator of 100_000.
		for i in 0..TOTAL {
			let fraction = i as f32 / TOTAL as f32;
			assert_eq!(
				NicePercent::from(fraction).as_str(),
				format!("{:0.02}%", fraction * 100_f32),
				"{}/{} (f32)", i, TOTAL
			);

			let fraction = i as f64 / TOTAL as f64;
			assert_eq!(
				NicePercent::from(fraction).as_str(),
				format!("{:0.02}%", fraction * 100_f64),
				"{}/{} (f64)", i, TOTAL
			);
		}

		// And a few edge cases.
		assert_eq!(NicePercent::from(0_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(f64::NAN).as_str(), "0.00%");
		assert_eq!(NicePercent::from(-10_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(1.03_f64).as_str(), "100.00%");
		assert_eq!(NicePercent::from(10_f64).as_str(), "100.00%");
	}
}
