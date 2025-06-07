/*!
# Dactyl: Nice Number Formatting.
*/

mod chr;
mod digiter;
mod nice_clock;
mod nice_elapsed;
mod nice_float;
mod nice_percent;
mod nice_u16;
mod nice_u32;
mod nice_u64;
mod nice_u8;

use digiter::Digiter;

use chr::NiceChar;
pub use chr::NiceSeparator;
pub use nice_clock::NiceClock;
pub use nice_elapsed::NiceElapsed;
pub use nice_float::NiceFloat;
pub use nice_percent::NicePercent;
pub use nice_u16::NiceU16;
pub use nice_u32::NiceU32;
pub use nice_u64::NiceU64;
pub use nice_u8::NiceU8;



/// # Helper: Number to String (w/ Thousands Separation).
///
/// Convert a series of digit tokens into a thousand-separated string.
macro_rules! nice_str {
	// Three and N Thousands.
	($d1:tt $d2:tt $d3:tt $($a:tt $b:tt $c:tt)+) => (
		concat!(
			stringify!($d1), stringify!($d2), stringify!($d3),
			$( ",", stringify!($a), stringify!($b), stringify!($c) ),+
		)
	);

	// Two and N Thousands.
	($d1:tt $d2:tt $($a:tt $b:tt $c:tt)+) => (
		concat!(
			stringify!($d1), stringify!($d2),
			$( ",", stringify!($a), stringify!($b), stringify!($c) ),+
		)
	);

	// One and N Thousands.
	($d1:tt $($a:tt $b:tt $c:tt)+) => (
		concat!(
			stringify!($d1),
			$( ",", stringify!($a), stringify!($b), stringify!($c) ),+
		)
	);

	// One, Two, or Three (No Thousands).
	($($digit:tt)+) => ( concat!($(stringify!($digit)),+) );
}
use nice_str;



/// # Helper: Number to Array (w/ Thousands Separation).
///
/// Convert a series of digit tokens into a thousand-separated array of
/// matching `NiceChar` variants.
macro_rules! nice_arr {
	// Literals to Digits.
	(@digit 0) => ( NiceChar::Digit0 );
	(@digit 1) => ( NiceChar::Digit1 );
	(@digit 2) => ( NiceChar::Digit2 );
	(@digit 3) => ( NiceChar::Digit3 );
	(@digit 4) => ( NiceChar::Digit4 );
	(@digit 5) => ( NiceChar::Digit5 );
	(@digit 6) => ( NiceChar::Digit6 );
	(@digit 7) => ( NiceChar::Digit7 );
	(@digit 8) => ( NiceChar::Digit8 );
	(@digit 9) => ( NiceChar::Digit9 );

	// Three and N Thousands.
	(@sep $sep:ident $d1:tt $d2:tt $d3:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1), nice_arr!(@digit $d2), nice_arr!(@digit $d3),
			$(
				$sep,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// Two and N Thousands.
	(@sep $sep:ident $d1:tt $d2:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1), nice_arr!(@digit $d2),
			$(
				$sep,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// One and N Thousands.
	(@sep $sep:ident $d1:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1),
			$(
				$sep,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// One, Two, or Three (No Thousands).
	(@sep $sep:ident $($digit:tt)+) => (
		[ $( nice_arr!(@digit $digit) ),+ ]
	);

	// Three and N Thousands.
	(@comma $d1:tt $d2:tt $d3:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1), nice_arr!(@digit $d2), nice_arr!(@digit $d3),
			$(
				NiceChar::Comma,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// Two and N Thousands.
	(@comma $d1:tt $d2:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1), nice_arr!(@digit $d2),
			$(
				NiceChar::Comma,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// One and N Thousands.
	(@comma $d1:tt $($a:tt $b:tt $c:tt)+) => (
		[
			nice_arr!(@digit $d1),
			$(
				NiceChar::Comma,
				nice_arr!(@digit $a),
				nice_arr!(@digit $b),
				nice_arr!(@digit $c)
			),+
		]
	);

	// One, Two, or Three (No Thousands).
	(@comma $($digit:tt)+) => (
		[ $( nice_arr!(@digit $digit) ),+ ]
	);

	// Use Comma If Separator Unspecified.
	($($digit:tt)+) => (
		nice_arr! { @comma $($digit)+ }
	);
}
use nice_arr;



#[cfg(test)]
/// # Helper: Unit Tests.
macro_rules! nice_test {
	($idx:ty) => (
		#[test]
		fn t_nice_idx() {
			assert_eq!(
				<$idx>::LEN,
				<$idx>::LAST as usize + 1,
				concat!("BUG: ", stringify!($idx), "::LAST not one less than LEN!"),
			);
			assert!(
				<$idx>::DIGITS.len() <= <$idx>::LEN,
				concat!("BUG: ", stringify!($idx), "::LEN is fewer than DIGITS!"),
			);
			assert_eq!(
				<$idx>::DIGITS.len(),
				<$idx>::LEN - <$idx>::LEN.wrapping_div(4),
				concat!("BUG: ", stringify!($idx), "::DIGITS has wrong length!"),
			);

			let mut digits = <$idx>::DIGITS.into_iter().map(|d| d as u8);

			let mut last = digits.next().unwrap();
			assert_eq!(
				<$idx>::LAST as u8,
				last,
				concat!("BUG: ", stringify!($idx), "::DIGITS must start with LAST!"),
			);

			for next in digits {
				assert!(
					next < last,
					concat!("BUG: ", stringify!($idx), "::DIGITS are not descending!"),
				);
				last = next;
			}

			assert_eq!(
				last,
				0,
				concat!("BUG: ", stringify!($idx), "::DIGITS doesn't end with zero!"),
			);
		}
	);

	($name:ty, $uint:ident) => (
		#[test]
		fn t_nice() {
			use num_format::{ToFormattedString, Locale};
			use std::collections::BTreeSet;

			#[cfg(not(miri))]
			const SAMPLE_SIZE: usize = 1_000_000;

			#[cfg(miri)]
			const SAMPLE_SIZE: usize = 500;

			// Explicitly check default, min, and max.
			assert_eq!(<$name>::default(), <$name>::from(<$uint>::MIN));
			assert_eq!(<$name>::MIN, <$name>::from(<$uint>::MIN));
			assert_eq!(<$name>::MAX, <$name>::from(<$uint>::MAX));

			// We'll need to collect, dedupe, and order the values in
			// advance to prevent random repetition (that would break our
			// replacement checks).
			let mut rng = fastrand::Rng::new();
			let set = std::iter::repeat_with(|| rng.$uint(..))
				.take(SAMPLE_SIZE)
				.collect::<BTreeSet<_>>();

			let mut last = <$name>::MAX;
			for i in set {
				let istr = i.to_formatted_string(&Locale::en);
				let nice = <$name>::from(i);

				assert_eq!(istr, nice.as_str());
				assert_eq!(istr.as_bytes(), nice.as_bytes());
				assert_eq!(istr.len(), nice.len());

				// This should not equal the last value!
				assert_ne!(nice, last);

				// Now it should!
				last.replace(i);
				assert_eq!(nice, last);
			}

			// Make sure back to zero works.
			last.replace(0);
			assert_eq!(last.as_str(), "0");

			// Try a different separator.
			assert_eq!(
				<$name>::with_separator(<$uint>::MAX, NiceSeparator::Underscore).as_str(),
				<$uint>::MAX.to_formatted_string(&Locale::en)
					.chars()
					.map(|c| if c == ',' { '_' } else { c })
					.collect::<String>(),
			);
		}
	);

	($name:ty, $uint:ident, $idx:ty) => (
		$crate::nice::nice_test!($name, $uint);
		$crate::nice::nice_test!($idx);
	);
}
#[cfg(test)] use nice_test;



/// # Helper: Initialize Nice U* Struct.
///
/// This macro is used to general all or most of a given `NiceU*` struct,
/// including all the different trait impls.
macro_rules! nice_uint {
	// Struct documentation.
	(@doc $name:expr, $ty:expr, $max:expr) => (
		concat!(
			"# Nice `", $ty, "`.\n\n",
			"This struct can be used to quickly and efficiently stringify a `", $ty, "` primitive (with thousands separators).\n\n",
			"## Examples\n\n",
			"```\n",
			"use dactyl::", $name, ";\n\n",
			"assert_eq!(\n",
			"    ", $name, "::from(", $ty, "::MAX).as_str(),\n",
			"    \"", $max, "\",\n",
			");\n",
			"```",
		)
	);

	// Secondary trait impls.
	(@traits $name:ident) => (
		impl AsRef<[u8]> for $name {
			#[inline]
			fn as_ref(&self) -> &[u8] { self.as_bytes() }
		}

		impl AsRef<str> for $name {
			#[inline]
			fn as_ref(&self) -> &str { self.as_str() }
		}

		impl std::borrow::Borrow<str> for $name {
			#[inline]
			fn borrow(&self) -> &str { self.as_str() }
		}

		impl std::fmt::Debug for $name {
			#[inline]
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.debug_tuple(stringify!($name))
					.field(&self.as_str())
					.finish()
			}
		}

		impl std::fmt::Display for $name {
			#[inline]
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				<str as std::fmt::Display>::fmt(self.as_str(), f)
			}
		}

		impl Default for $name {
			#[inline]
			fn default() -> Self { Self::MIN }
		}

		impl Eq for $name {}

		impl From<$name> for String {
			#[inline]
			fn from(src: $name) -> Self { src.as_str().to_owned() }
		}

		impl std::hash::Hash for $name {
			#[inline]
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
				state.write(self.as_bytes());
			}
		}

		impl Ord for $name {
			#[inline]
			fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
				self.as_bytes().cmp(rhs.as_bytes())
			}
		}

		impl PartialEq for $name {
			#[inline]
			fn eq(&self, rhs: &Self) -> bool { self.as_bytes() == rhs.as_bytes() }
		}

		impl PartialOrd for $name {
			#[inline]
			fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
				Some(self.cmp(rhs))
			}
		}
	);

	// As Bytes, As Str, etc.
	(@bytes $name:ident, $from:expr, $max:expr) => (
		impl $name {
			#[must_use]
			/// # As Byte Slice.
			///
			/// Return the value as a byte slice.
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::from(", $from, ").as_bytes(),")]
			#[doc = concat!("    b\"", $max, "\",")]
			/// );
			/// ```
			pub const fn as_bytes(&self) -> &[u8] {
				let (_, used) = self.data.split_at(self.from as usize);
				NiceChar::as_bytes(used)
			}

			#[must_use]
			#[allow(clippy::allow_attributes, dead_code, reason = "Auto-generated.")]
			/// # As Nice Slice.
			///
			/// Return the value as a byte slice.
			pub(super) const fn as_bytes_raw(&self) -> &[NiceChar] {
				let (_, used) = self.data.split_at(self.from as usize);
				used
			}

			#[must_use]
			/// # As String Slice.
			///
			/// Return the value as a string slice.
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::from(", $from, ").as_str(),")]
			#[doc = concat!("    \"", $max, "\",")]
			/// );
			/// ```
			pub const fn as_str(&self) -> &str {
				let (_, used) = self.data.split_at(self.from as usize);
				NiceChar::as_str(used)
			}

			#[must_use]
			/// # Is Empty?
			///
			/// No! Haha. But for consistency, this method exists.
			pub const fn is_empty(&self) -> bool { false }

			#[must_use]
			/// # Length.
			///
			/// Return the length of the nice byte/string representation.
			///
			/// Note this will never be zero.
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			#[doc = concat!("let nice = ", stringify!($name), "::MAX;")]
			/// assert_eq!(
			///     nice.len(),
			///     nice.as_str().len(),
			/// );
			/// ```
			pub const fn len(&self) -> usize {
				self.data.len() - self.from as usize
			}
		}
	);

	// Struct definition.
	($name:ident, $idx:ty, $uint:ty, $nz:ty, [ $($min:tt)+ ], [ $($max:tt)+ ]) => (
		#[derive(Clone, Copy)]
		#[doc = nice_uint!(
			@doc
			stringify!($name),
			stringify!($uint),
			nice_str!($($max)+)
		)]
		pub struct $name {
			/// # String Buffer.
			data: [NiceChar; <$idx>::LEN],

			/// # Starting Position.
			///
			/// Data is written right to left.
			from: $idx,
		}

		nice_uint!(@traits $name);
		nice_uint!(@bytes $name, concat!(stringify!($uint), "::MAX"), nice_str!($($max)+));

		impl From<Option<$uint>> for $name {
			#[inline]
			fn from(src: Option<$uint>) -> Self { src.map_or(Self::MIN, Self::from) }
		}

		impl From<$nz> for $name {
			#[inline]
			fn from(src: $nz) -> Self { Self::from(src.get()) }
		}

		impl From<Option<$nz>> for $name {
			#[inline]
			fn from(src: Option<$nz>) -> Self { src.map_or(Self::MIN, Self::from) }
		}

		impl $name {
			/// # Minimum Value.
			///
			#[doc = concat!("The nice equivalent of `", stringify!($uint), "::MIN`.")]
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::MIN.as_str(),")]
			///     "0",
			/// );
			/// ```
			pub const MIN: Self = Self {
				data: nice_arr!($($min)+),
				from: <$idx>::LAST,
			};

			/// # Maximum Value.
			///
			#[doc = concat!("The nice equivalent of `", stringify!($uint), "::MAX`.")]
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::MAX.as_str(),")]
			#[doc = concat!("    \"", nice_str!($($max)+), "\",")]
			/// );
			/// ```
			pub const MAX: Self = Self {
				data: nice_arr!($($max)+),
				from: <$idx>::From00,
			};
		}
	);

	// Struct definition _and_ construction helpers.
	(@full $name:ident, $idx:ty, $uint:ty, $nz:ty, [ $($min:tt)+ ], [ $($max:tt)+ ]) => (
		nice_uint!($name, $idx, $uint, $nz, [ $($min)+ ], [ $($max)+ ]);

		impl From<$uint> for $name {
			#[inline]
			fn from(src: $uint) -> Self {
				const {
					assert!(
						<$idx>::DIGITS.len() == <$uint>::MAX.ilog10() as usize + 1,
						concat!("BUG: ", stringify!($idx), "::DIGITS has different digit count than ", stringify!($uint), "::MAX."),
					);
				}

				let mut data = nice_arr!($($min)+);
				let mut from = <$idx>::LAST;
				if let Some(digits) = Digiter::<$uint>::new(src) {
					for (k, v) in <$idx>::DIGITS.into_iter().zip(digits) {
						data[k as usize] = v;
						from = k;
					}
				}
				Self { data, from }
			}
		}

		impl $name {
			#[must_use]
			/// # New (w/ Alternative Thousands Separator).
			///
			#[doc = concat!("Nicely stringify a `", stringify!($uint), "` with a specific thousands separator (instead of the default comma).")]
			///
			/// ## Examples
			///
			/// ```
			#[doc = concat!("use dactyl::{NiceSeparator, ", stringify!($name), "};")]
			///
			#[doc = concat!("let num: ", stringify!($uint), " = 54321;")]
			///
			/// // Commas are used by default.
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::from(num).as_str(),")]
			///     "54,321",
			/// );
			///
			/// // Other contexts might prefer, say, underscores…
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($name), "::with_separator(")]
			///         num,
			///         NiceSeparator::Underscore,
			///     ).as_str(),
			///     "54_321",
			/// );
			/// ```
			pub fn with_separator(src: $uint, sep: NiceSeparator) -> Self {
				let sep = sep.as_nice_char();
				let mut data = nice_arr!(@sep sep $($min)+);
				let mut from = <$idx>::LAST;
				if let Some(digits) = Digiter::<$uint>::new(src) {
					for (k, v) in <$idx>::DIGITS.into_iter().zip(digits) {
						data[k as usize] = v;
						from = k;
					}
				}
				Self { data, from }
			}

			/// # Replace w/ New Number.
			///
			/// Reuse the backing storage behind `self` to hold a new nice number.
			///
			/// ## Examples.
			///
			/// ```
			#[doc = concat!("use dactyl::", stringify!($name), ";")]
			///
			#[doc = concat!("let mut num = ", stringify!($name), "::from(1234_", stringify!($uint), ");")]
			/// assert_eq!(num.as_str(), "1,234");
			///
			/// num.replace(1);
			/// assert_eq!(num.as_str(), "1");
			/// ```
			pub fn replace(&mut self, src: $uint) {
				let Some(digits) = Digiter::<$uint>::new(src) else {
					self.data[<$idx>::LAST as usize] = NiceChar::Digit0;
					self.from = <$idx>::LAST;
					return;
				};

				for (k, v) in <$idx>::DIGITS.into_iter().zip(digits) {
					self.data[k as usize] = v;
					self.from = k;
				}
			}
		}
	);
}
use nice_uint;



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice_arr() {
		assert_eq!(
			NiceChar::as_str(nice_arr!(0 1 2).as_slice()),
			"012",
		);
		assert_eq!(
			NiceChar::as_str(nice_arr!(0 1 2 3).as_slice()),
			"0,123",
		);
		assert_eq!(
			NiceChar::as_str(nice_arr!(0 1 2 3 4).as_slice()),
			"01,234",
		);
		assert_eq!(
			NiceChar::as_str(nice_arr!(0 1 2 3 4 5).as_slice()),
			"012,345",
		);
		assert_eq!(
			NiceChar::as_str(nice_arr!(0 1 2 3 4 5 6).as_slice()),
			"0,123,456",
		);
	}

	#[test]
	fn t_nice_str() {
		assert_eq!(
			nice_str!(0 1 2),
			"012",
		);
		assert_eq!(
			nice_str!(0 1 2 3),
			"0,123",
		);
		assert_eq!(
			nice_str!(0 1 2 3 4),
			"01,234",
		);
		assert_eq!(
			nice_str!(0 1 2 3 4 5),
			"012,345",
		);
		assert_eq!(
			nice_str!(0 1 2 3 4 5 6),
			"0,123,456",
		);
	}
}
