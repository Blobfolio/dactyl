/*!
# Dactyl: Integer Division
*/



/// # Integer Float Division.
///
/// This trait adds a `div_float` method to primitive integers, enabling
/// division as floats.
///
/// ## Examples
///
/// ```
/// use dactyl::traits::IntDivFloat;
///
/// // Equivalent to 25_f64 / 20_f64.
/// assert_eq!(
///     25_u32.div_float(20_u32),
///     Some(1.25_f64),
/// );
/// ```
pub trait IntDivFloat: Copy {
	/// # Integer to Float Division.
	///
	/// Recast two integers to floats, then divide them and return the result,
	/// or `None` if the operation is invalid or yields `NaN` or infinity.
	fn div_float(self, d: Self) -> Option<f64>;
}

/// # Helper: Implement Trait.
macro_rules! intdiv {
	($($ty:ty),+) => ($(
		impl IntDivFloat for $ty {
			#[inline]
			/// # Integer to Float Division.
			///
			/// Recast two integers to floats, then divide them and return the
			/// result, or `None` if the operation is invalid or yields `NaN`
			/// or infinity.
			///
			/// ## Examples
			///
			/// ```
			/// use dactyl::traits::IntDivFloat;
			///
			/// // Equivalent to 20_f64 / 16_f64.
			/// assert_eq!(
			#[doc = concat!("    20_", stringify!($ty), ".div_float(16),")]
			///     Some(1.25_f64),
			/// );
			///
			/// // Division by zero is still a no-no.
			#[doc = concat!("assert!(20_", stringify!($ty), ".div_float(0).is_none());")]
			/// ```
			fn div_float(self, d: Self) -> Option<f64> {
				let res = self as f64 / d as f64;
				if res.is_finite() { Some(res) }
				else { None }
			}
		}
	)+);
}

intdiv! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }



#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn t_div_float() {
		macro_rules! t_div {
			($($ty:ty),+) => ($(
				let e: $ty = 3;
				let d: $ty = 4;

				// Easy stuff.
				assert_eq!(e.div_float(d), Some(0.75));
				assert_eq!(e.div_float(e), Some(1.0));

				// 1.3333333333…
				let Some(long) = d.div_float(e) else {
					panic!(
						concat!("{}_", stringify!($ty), " / {}_", stringify!($ty), " failed."),
						d,
						e,
					);
				};
				assert!(
					long > 1.0 && long < 2.0,
					concat!("{}_", stringify!($ty), " / {}_", stringify!($ty), " came out weird: {}"),
					d,
					e,
					long,
				);

				// Can't divide by zero!
				assert_eq!(e.div_float(0), None);
			)+);
		}

		// Make sure we actually implemented all of these. Haha.
		t_div! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
	}
}
