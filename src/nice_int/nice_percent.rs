/*!
# Dactyl: Nice Percent.
*/

use crate::{
	Digiter,
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
				// Treat NaN as zero.
				if num.is_nan() { return Self::MIN; }

				// We can maintain precision better by working from an integer.
				// Clamp and multiply by the desired precision.
				let whole = (num.clamp(0.0, 1.0) * 10_000.0).round() as u16;

				// Manually handle the edges.
				if whole == 0 { return Self::MIN; }
				else if 9999 < whole { return Self::MAX; }

				// Split the top and bottom.
				let [a, b] = Digiter(whole / 100).double();
				let [c, d] = Digiter(whole % 100).double();
				let from = if a == b'0' { SIZE - 5 } else { SIZE - 6 };

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

impl NicePercent {
	#[expect(
		clippy::cast_possible_truncation,
		clippy::cast_sign_loss,
		reason = "False positive.",
	)]
	/// # Replace.
	///
	/// Reuse the backing storage behind `self` to hold a new nice percent.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// let mut num = NicePercent::from(0.85);
	/// assert_eq!(num.as_str(), "85.00%");
	///
	/// num.replace(0.334);
	/// assert_eq!(num.as_str(), "33.40%");
	/// ```
	pub fn replace(&mut self, num: f32) {
		// Treat NaN as zero.
		if num.is_nan() { return self.reset_min(); }

		// We can maintain precision better by working from an integer.
		// Clamp and multiply by the desired precision.
		let whole = (num.clamp(0.0, 1.0) * 10_000.0).round() as u16;

		// Manually handle the edges.
		if whole == 0 { return self.reset_min(); }
		else if 9999 < whole { return self.reset_max(); }

		// Split the top and bottom.
		[self.inner[1], self.inner[2]] = Digiter(whole / 100).double();
		[self.inner[4], self.inner[5]] = Digiter(whole % 100).double();
		self.from = if self.inner[1] == b'0' { SIZE - 5 } else { SIZE - 6 };
	}

	/// # Reset to Minimum.
	const fn reset_min(&mut self) {
		self.inner[2] = b'0';
		self.inner[4] = b'0';
		self.inner[5] = b'0';
		self.from = SIZE - 5;
	}

	/// # Reset to Maximum.
	const fn reset_max(&mut self) {
		self.inner[0] = b'1';
		self.inner[1] = b'0';
		self.inner[2] = b'0';
		self.inner[4] = b'0';
		self.inner[5] = b'0';
		self.from = 0;
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_percent() {
		// There will be disagreements with a denominator of 100_000.
		const TOTAL: u32 = 10_000_u32;

		let mut last = NicePercent::MIN;

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

				// Test replacments.
				if fraction == 0.0 { assert_eq!(last, nice); }
				else { assert_ne!(last, nice); }
				last.replace(fraction);
				assert_eq!(last, nice);

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

		// Replacement to zero should be happy.
		if last.as_str() != "0.00%" {
			last.replace(0.0);
			assert_eq!(last.as_str(), "0.00%");
		}

		// And a few edge cases.
		assert_eq!(NicePercent::from(0_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(f64::NAN).as_str(), "0.00%");
		assert_eq!(NicePercent::from(-10_f64).as_str(), "0.00%");
		assert_eq!(NicePercent::from(1.03_f64).as_str(), "100.00%");
		assert_eq!(NicePercent::from(10_f64).as_str(), "100.00%");
	}
}
