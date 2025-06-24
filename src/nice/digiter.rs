/*!
# Dactyl: Digit Iterator.
*/

use super::NiceChar;



#[derive(Debug, Clone, Eq, PartialEq)]
/// # Popping Digiterator.
///
/// This struct is used internally by the library's various `Nice*` structs to
/// help stringify numbers.
///
/// It employs a naive divide-by-ten strategy to "pop" digits off the end one
/// at a time, returning the `NiceChar` equivalent of each.
pub(super) struct Digiter<T>(T);

/// # Helper: Primitive Implementations.
macro_rules! digiter {
	($($ty:ident $conv:ident),+ $(,)?) => ($(
		#[allow(
			dead_code,
			clippy::allow_attributes,
			trivial_numeric_casts,
			reason = "Macro made me do it.",
		)]
		impl Digiter<$ty> {
			#[inline]
			/// # New Instance.
			///
			/// Return a new [`Digiter`] for a given value, unless zero.
			///
			/// This is only necessary for iteration purposes; for one-off
			/// crunching it can instantiated directly to service any number,
			/// including zero.
			pub(super) const fn new(num: $ty) -> Option<Self> {
				if num == 0 { None }
				else { Some(Self(num)) }
			}
		}

		impl Iterator for Digiter<$ty> {
			type Item = NiceChar;

			#[allow(
				clippy::allow_attributes,
				trivial_numeric_casts,
				reason = "Macro made me do it.",
			)]
			#[inline]
			/// # Digit Iteration.
			///
			/// Read and return each digit, right to left.
			///
			/// This will not work if the starting value is zero; `Digiter::new`
			/// should be used for initialization to rule out that possibility.
			fn next(&mut self) -> Option<Self::Item> {
				if self.0 == 0 { None }
				else {
					let next = NiceChar::$conv(self.0);
					self.0 = self.0.wrapping_div(10);
					Some(next)
				}
			}

			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				let len = self.len();
				(len, Some(len))
			}
		}

		impl ExactSizeIterator for Digiter<$ty> {
			#[inline]
			fn len(&self) -> usize {
				// Zero marks the end for the iterator.
				if self.0 == 0 { 0 }
				else { self.0.ilog10() as usize + 1 }
			}
		}

		impl std::iter::FusedIterator for Digiter<$ty> {}
	)+);
}

digiter! {
	 u8 from_digit_u8,
	u16 from_digit_u16,
	u32 from_digit_u32,
	u64 from_digit_u64,
}



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri is way too slow for a million tests!

	/// # Helper: Digiter test for one specific value.
	macro_rules! t_digiter {
		($num:ident, $ty:ty) => (
			// The expected number.
			let expected = $num.to_string();

			// Make sure we can digitize it.
			let Some(iter) = Digiter::<$ty>::new($num) else {
				panic!(
					concat!("Digiter::new failed with {num}_", stringify!($ty)),
					num=expected,
				);
			};

			// Verify the iter's reported length matches.
			assert_eq!(
				iter.len(),
				expected.len(),
				concat!("Digiter::len invalid for {num}_", stringify!($ty)),
				num=expected,
			);

			// Collect the results and reverse, then verify we got it right!
			let mut digits = iter.collect::<Vec<NiceChar>>();
			digits.reverse();
			assert_eq!(
				NiceChar::as_str(digits.as_slice()),
				expected.as_str(),
			);
		);
	}

	#[test]
	fn t_digiter_u8() {
		// Zero is a no.
		assert!(Digiter::<u8>::new(0).is_none());

		// Everything else should be happy!
		for i in 1..=u8::MAX { t_digiter!(i, u8); }
	}

	#[test]
	fn t_digiter_u16() {
		// Zero is a no.
		assert!(Digiter::<u16>::new(0).is_none());

		#[cfg(not(miri))]
		for i in 1..=u16::MAX { t_digiter!(i, u16); }

		#[cfg(miri)]
		{
			let mut rng = fastrand::Rng::new();
			for i in std::iter::repeat_with(|| rng.u16(1..u16::MAX)).take(SAMPLE_SIZE) {
				t_digiter!(i, u16);
			}

			// Explicitly check the max works.
			let i = u16::MAX;
			t_digiter!(i, u16);
		}
	}

	#[test]
	fn t_digiter_u32() {
		// Zero is a no.
		assert!(Digiter::<u32>::new(0).is_none());

		// Testing the full range takes too long.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(1..u32::MAX)).take(SAMPLE_SIZE) {
			t_digiter!(i, u32);
		}

		// Explicitly check the max works.
		let i = u32::MAX;
		t_digiter!(i, u32);
	}

	#[test]
	fn t_digiter_u64() {
		// Zero is a no.
		assert!(Digiter::<u64>::new(0).is_none());

		// Testing the full range takes too long.
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u64(1..u64::MAX)).take(SAMPLE_SIZE) {
			t_digiter!(i, u64);
		}

		// Explicitly check the max works.
		let i = u64::MAX;
		t_digiter!(i, u64);
	}
}
