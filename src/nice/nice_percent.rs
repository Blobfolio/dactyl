/*!
# Dactyl: Nice Percent.
*/

use crate::traits::IntDivFloat;
use super::{
	Digiter,
	nice_uint,
	NiceChar,
};



#[expect(dead_code, reason = "For readability.")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
/// # `NicePercent` Indices.
enum NicePercentIdx {
	From00 = 0_u8, // 1
	From01 = 1_u8, // 0
	From02 = 2_u8, // 0
	From03 = 3_u8, // .
	From04 = 4_u8, // 0
	From05 = 5_u8, // 0
	From06 = 6_u8, // %
}

impl NicePercentIdx {
	/// # Digit Indices (Reverse Order).
	///
	/// Note that this does not include the first position, as that only
	/// applies to the maximum value.
	const DIGITS: [Self; 4] = [
		Self::From05, Self::From04, // .
		Self::From02, Self::From01,
	];

	/// # Length.
	const LEN: usize = 7;
}



#[derive(Clone, Copy)]
/// # Nice Percent.
///
/// This struct can be used to quickly and efficiently stringify a "percent" —
/// a float between `0.0..=1.0` — to a fixed precision of hundredths.
///
/// ## Examples
///
/// ```
/// use dactyl::NicePercent;
///
/// assert_eq!(
///     NicePercent::from(0.55012345_f32).as_str(),
///     "55.01%",
/// );
/// ```
pub struct NicePercent {
	/// # String Buffer.
	data: [NiceChar; NicePercentIdx::LEN],

	/// # Starting Position.
	///
	/// Data is written right to left.
	from: NicePercentIdx,
}

impl NicePercent {
	/// # Minimum Value.
	///
	/// The nice equivalent of `0.0`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// assert_eq!(
	///     NicePercent::MIN.as_str(),
	///     "0.00%",
	/// );
	/// ```
	pub const MIN: Self = Self {
		data: [
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Period,
			NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Percent,
		],
		from: NicePercentIdx::From02,
	};

	/// # Maximum Value.
	///
	/// The nice equivalent of `1.0`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NicePercent;
	///
	/// assert_eq!(
	///     NicePercent::MAX.as_str(),
	///     "100.00%",
	/// );
	/// ```
	pub const MAX: Self = Self {
		data: [
			NiceChar::Digit1, NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Period,
			NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Percent,
		],
		from: NicePercentIdx::From00,
	};
}

/// # Helper: From Float.
macro_rules! from {
	($($ty:ty)+) => ($(
		#[expect(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "It is what it is.",
		)]
		impl From<$ty> for NicePercent {
			fn from(num: $ty) -> Self {
				// Treat NaN as zero.
				if num.is_nan() { return Self::MIN; }

				// We can maintain precision better by working from an integer.
				// Clamp and multiply by the desired precision.
				let whole = (num.clamp(0.0, 1.0) * 10_000.0).round() as u16;

				// Manually handle the edges.
				if 9999 < whole { return Self::MAX; }

				let mut out = Self::MIN;
				if let Some(digits) = Digiter::<u16>::new(whole) {
					for (k, v) in NicePercentIdx::DIGITS.into_iter().zip(digits) {
						out.data[k as usize] = v;
					}

					if ! matches!(out.data[1], NiceChar::Digit0) {
						out.from = NicePercentIdx::From01;
					}
				}

				out
			}
		}
	)+);
}
from!(f32 f64);

impl<T: IntDivFloat> TryFrom<(T, T)> for NicePercent {
	type Error = ();

	#[inline]
	/// # Percent From T/T.
	///
	/// This method is a shorthand that performs the (decimal) division of
	/// `T1 / T2` for you, then converts the result into a [`NicePercent`],
	/// clamping to `0.0`/`1.0` as usual.
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
	/// Conversion will fail if the resulting float isn't finite.
	fn try_from(src: (T, T)) -> Result<Self, Self::Error> {
		src.0.div_float(src.1)
			.map(Self::from)
			.ok_or(())
	}
}

nice_uint!(@traits NicePercent);
nice_uint!(@bytes NicePercent, "1.0_f32", "100.00%");

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
		if 9999 < whole { return self.reset_max(); }

		self.reset_min();
		if let Some(digits) = Digiter::<u16>::new(whole) {
			for (k, v) in NicePercentIdx::DIGITS.into_iter().zip(digits) {
				self.data[k as usize] = v;
			}

			if ! matches!(self.data[1], NiceChar::Digit0) {
				self.from = NicePercentIdx::From01;
			}
		}
	}

	/// # Reset to Minimum.
	const fn reset_min(&mut self) {
		self.data[2] = NiceChar::Digit0;
		self.data[4] = NiceChar::Digit0;
		self.data[5] = NiceChar::Digit0;
		self.from = NicePercentIdx::From02;
	}

	/// # Reset to Maximum.
	const fn reset_max(&mut self) {
		self.data[0] = NiceChar::Digit1;
		self.data[1] = NiceChar::Digit0;
		self.data[2] = NiceChar::Digit0;
		self.data[4] = NiceChar::Digit0;
		self.data[5] = NiceChar::Digit0;
		self.from = NicePercentIdx::From00;
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::BTreeSet;

	#[test]
	fn t_nice_idx() {
		// Most of the usual index tests don't apply to this one, but we can
		// at least verify they proceed downward!
		let mut digits = NicePercentIdx::DIGITS.into_iter().map(|d| d as u8);
		let mut last = digits.next().unwrap();
		for next in digits {
			assert!(
				next < last,
				concat!("BUG: NicePercentIdx::DIGITS are not descending!"),
			);
			last = next;
		}
	}

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 500;

	#[test]
	fn t_nice() {
		const TOTAL: u32 = 10_000;

		// Explicitly check the default, min, and max.
		assert_eq!(NicePercent::default(), NicePercent::from(f32::MIN));
		assert_eq!(NicePercent::MIN, NicePercent::from(f32::MIN));
		assert_eq!(NicePercent::MAX, NicePercent::from(f32::MAX));

		let set: BTreeSet<u32>;
		#[cfg(not(miri))]
		{
			set = (0..TOTAL).collect();
		}

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			set = std::iter::repeat_with(|| rng.u32(0..TOTAL))
				.take(SAMPLE_SIZE)
				.collect();
		}

		let mut last = NicePercent::MAX;
		for i in set {
			let fraction = i as f32 / TOTAL as f32;
			let nice = NicePercent::from(fraction);
			let istr = format!("{:0.02}%", fraction * 100.0);

			assert_eq!(istr, nice.as_str());
			assert_eq!(istr.as_bytes(), nice.as_bytes());
			assert_eq!(istr.len(), nice.len());

			// This should not equal the last value!
			assert_ne!(nice, last);

			// Now it should!
			last.replace(fraction);
			assert_eq!(nice, last);

			// Let's check f64 real quick.
			assert_eq!(
				NicePercent::from(i as f64 / TOTAL as f64),
				nice,
			);
		}

		// Make sure back to zero works.
		last.replace(0.0);
		assert_eq!(last.as_str(), "0.00%");

		// As does back to max.
		last.replace(1.0);
		assert_eq!(last.as_str(), "100.00%");
	}
}
