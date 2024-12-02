/*!
# Dactyl: Nice Percent.
*/

use crate::{
	NiceWrapper,
	traits::IntDivFloat,
};



/// # Total Buffer Size.
const SIZE: usize = 7;

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
	#[inline]
	fn default() -> Self { Self::MIN }
}

/// # Helper: From
///
/// This code is identical for `f32` and `f64` types.
macro_rules! nice_from {
	($($float:ty),+ $(,)?) => ($(
		#[expect(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "It is what it is.",
		)]
		#[expect(clippy::integer_division, reason = "We want this.")]
		impl From<$float> for NicePercent {
			fn from(num: $float) -> Self {
				// Shortcut for overflowing values.
				if num <= 0.0 || ! num.is_normal() { return Self::MIN; }
				else if 1.0 <= num { return Self::MAX; }

				// We can maintain precision better by working from an integer.
				// We know there is no existing integer part, so at most we'll
				// wind up with four digits, which fits nicely in a u16.
				let whole = (num * 10_000.0).round() as u16;

				// Recheck the boundaries because of the rounding.
				if whole == 0 { return Self::MIN; }
				else if 9999 < whole { return Self::MAX; }

				// Split the top and bottom.
				let (top, bottom) = (whole / 100, whole % 100);

				let [a, b] = crate::double(top as usize);
				let from = if a == b'0' { SIZE - 5 } else { SIZE - 6 };
				let [c, d] = crate::double(bottom as usize);

				Self {
					inner: [b'0', a, b, b'.', c, d, b'%'],
					from,
				}
			}
		}
	)+);
}

nice_from!(f32, f64);

impl<T: IntDivFloat> TryFrom<(T, T)> for NicePercent {
	type Error = ();

	#[inline]
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
		src.0.div_float(src.1)
			.map(Self::from)
			.ok_or(())
	}
}

impl NicePercent {
	/// # Minimum Value.
	///
	/// Zero percent.
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// assert_eq!(
	///     NicePercent::MIN.as_str(),
	///     "0.00%"
	/// );
	///
	/// assert_eq!(
	///     NicePercent::MIN,
	///     NicePercent::from(0_f32),
	/// );
	/// ```
	pub const MIN: Self = Self {
		inner: ZERO,
		from: SIZE - 5,
	};

	/// # Maximum Value.
	///
	/// One hundred percent.
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// assert_eq!(
	///     NicePercent::MAX.as_str(),
	///     "100.00%"
	/// );
	///
	/// assert_eq!(
	///     NicePercent::MAX,
	///     NicePercent::from(1_f32),
	/// );
	/// ```
	pub const MAX: Self = Self {
		inner: [b'1', b'0', b'0', b'.', b'0', b'0', b'%'],
		from: 0,
	};
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_percent() {
		// There will be disagreements with a denominator of 100_000.
		const TOTAL: u32 = 10_000_u32;

		// Test as_str sanity for f32, f64.
		macro_rules! t_str {
			($var:ident) => (
				let fraction = $var as f32 / TOTAL as f32;
				let nice = NicePercent::from(fraction);
				assert_eq!(
					nice.as_str(),
					format!("{:0.02}%", fraction * 100_f32),
					"{}/{} (f32)", $var, TOTAL
				);
				assert_eq!(nice.len(), nice.as_str().len());
				assert_eq!(nice.len(), nice.as_bytes().len());
				assert!(! nice.is_empty());

				let fraction = $var as f64 / TOTAL as f64;
				let nice = NicePercent::from(fraction);
				assert_eq!(
					nice.as_str(),
					format!("{:0.02}%", fraction * 100_f64),
					"{}/{} (f64)", $var, TOTAL
				);
				assert_eq!(nice.len(), nice.as_str().len());
				assert_eq!(nice.len(), nice.as_bytes().len());
				assert!(! nice.is_empty());
			);
		}

		// Either test everything or a subset depending on miri-ness.
		#[cfg(not(miri))] for i in 0..TOTAL { t_str!(i); }
		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u32(0..TOTAL)).take(500) {
				t_str!(i);
			}
		}

		// And a few edge cases.
		assert_eq!(NicePercent::from(0_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(f64::NAN).as_str(), "0.00%");
		assert_eq!(NicePercent::from(-10_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(1.03_f64).as_str(), "100.00%");
		assert_eq!(NicePercent::from(10_f64).as_str(), "100.00%");
	}
}
