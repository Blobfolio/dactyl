/*!
# Dactyl: Nice Float.
*/

use crate::NiceSeparator;
use super::{
	Digiter,
	nice_uint,
	NiceChar,
};



/// # Helper: Buffer w/ Separators.
macro_rules! data {
	($comma:expr, $dot:expr) => (
		[
			NiceChar::Space, // Reserved for sign.
			NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$comma,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			$dot,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
		]
	);

	// Default separators.
	() => ( data!(NiceChar::Comma, NiceChar::Period) );
}



#[expect(dead_code, reason = "For readability.")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
/// # `NiceFloat` Indices.
enum NiceFloatIdx {
	From00 =  0_u8, // ±
	From01 =  1_u8, // 1
	From02 =  2_u8, // 8
	From03 =  3_u8, // ,
	From04 =  4_u8, // 4
	From05 =  5_u8, // 4
	From06 =  6_u8, // 6
	From07 =  7_u8, // ,
	From08 =  8_u8, // 7
	From09 =  9_u8, // 4
	From10 = 10_u8, // 4
	From11 = 11_u8, // ,
	From12 = 12_u8, // 0
	From13 = 13_u8, // 7
	From14 = 14_u8, // 3
	From15 = 15_u8, // ,
	From16 = 16_u8, // 7
	From17 = 17_u8, // 0
	From18 = 18_u8, // 9
	From19 = 19_u8, // ,
	From20 = 20_u8, // 5
	From21 = 21_u8, // 5
	From22 = 22_u8, // 1
	From23 = 23_u8, // ,
	From24 = 24_u8, // 6
	From25 = 25_u8, // 1
	From26 = 26_u8, // 5
	From27 = 27_u8, // .
	From28 = 28_u8, // 0
	From29 = 29_u8, // 0
	From30 = 30_u8, // 0
	From31 = 31_u8, // 0
	From32 = 32_u8, // 0
	From33 = 33_u8, // 0
	From34 = 34_u8, // 0
	From35 = 35_u8, // 0
}

impl NiceFloatIdx {
	/// # Top Digit Indices (Reverse Order).
	const TOP: [Self; 20] = [
		Self::From26, Self::From25, Self::From24, // ,
		Self::From22, Self::From21, Self::From20, // ,
		Self::From18, Self::From17, Self::From16, // ,
		Self::From14, Self::From13, Self::From12, // ,
		Self::From10, Self::From09, Self::From08, // ,
		Self::From06, Self::From05, Self::From04, // ,
		Self::From02, Self::From01,
	];

	/// # Last (Top).
	const LAST: Self = Self::From26;

	/// # Bottom Starts.
	const BOTTOM_START: Self = Self::From28;

	/// # Length.
	const LEN: usize = 36;

	/// # Precision Multiplier.
	const PRECISION: u32 = 100_000_000;

	/// # Minus One (Saturating).
	const fn previous(self) -> Self {
		match self {
			Self::From00 | Self::From01 => Self::From00,
			Self::From02 => Self::From01,
			Self::From03 => Self::From02,
			Self::From04 => Self::From03,
			Self::From05 => Self::From04,
			Self::From06 => Self::From05,
			Self::From07 => Self::From06,
			Self::From08 => Self::From07,
			Self::From09 => Self::From08,
			Self::From10 => Self::From09,
			Self::From11 => Self::From10,
			Self::From12 => Self::From11,
			Self::From13 => Self::From12,
			Self::From14 => Self::From13,
			Self::From15 => Self::From14,
			Self::From16 => Self::From15,
			Self::From17 => Self::From16,
			Self::From18 => Self::From17,
			Self::From19 => Self::From18,
			Self::From20 => Self::From19,
			Self::From21 => Self::From20,
			Self::From22 => Self::From21,
			Self::From23 => Self::From22,
			Self::From24 => Self::From23,
			Self::From25 => Self::From24,
			Self::From26 => Self::From25,
			Self::From27 => Self::From26,
			Self::From28 => Self::From27,
			Self::From29 => Self::From28,
			Self::From30 => Self::From29,
			Self::From31 => Self::From30,
			Self::From32 => Self::From31,
			Self::From33 => Self::From32,
			Self::From34 => Self::From33,
			Self::From35 => Self::From34,
		}
	}
}



#[derive(Clone, Copy)]
/// # Nice Float.
///
/// This struct can be used to quickly and efficiently stringify a float.
///
/// ## Examples
///
/// ```
/// use dactyl::NiceFloat;
///
/// assert_eq!(
///     NiceFloat::from(1234.5501234501_f64).as_str(),
///     "1,234.55012345", // Decimals only go to eight places.
/// );
/// ```
pub struct NiceFloat(FloatKind);

nice_uint!(@traits NiceFloat);

impl From<f32> for NiceFloat {
	#[inline]
	fn from(num: f32) -> Self {
		match FloatKind::from32(num, NiceSeparator::Comma) {
			Ok(kind) => Self(kind),
			Err((top, bottom, neg)) => {
				let mut out = NiceInner::ZERO;
				out.parse(top, bottom, neg);
				Self(FloatKind::Normal(out))
			}
		}
	}
}

impl From<f64> for NiceFloat {
	#[inline]
	fn from(num: f64) -> Self {
		match FloatKind::from64(num, NiceSeparator::Comma) {
			Ok(kind) => Self(kind),
			Err((top, bottom, neg)) => {
				let mut out = NiceInner::ZERO;
				out.parse(top, bottom, neg);
				Self(FloatKind::Normal(out))
			}
		}
	}
}

impl From<Result<f64, f64>> for NiceFloat {
	#[inline]
	fn from(src: Result<f64, f64>) -> Self {
		match src {
			Ok(f) | Err(f) => Self::from(f)
		}
	}
}

impl NiceFloat {
	/// # "Minimum".
	///
	/// To be clear, zero isn't the actual "minimum" value, but this constant
	/// is required by the trait macro for the `Default` impl.
	///
	/// It isn't public or used anywhere else, so we can pretend for its sake.
	const MIN: Self = Self::ZERO;

	/// # Infinity.
	///
	/// A value representing infinity. Note that no distinction is made between
	/// positive and negative varieties.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// assert_eq!(NiceFloat::INFINITY.as_str(), "∞");
	/// assert_eq!(NiceFloat::from(f64::INFINITY).as_str(), "∞");
	/// assert_eq!(NiceFloat::from(f64::NEG_INFINITY).as_str(), "∞");
	/// ```
	pub const INFINITY: Self = Self(FloatKind::Infinity);

	/// # NaN.
	///
	/// A value representing a Not-a-Number.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// assert_eq!(NiceFloat::NAN.as_str(), "NaN");
	/// assert_eq!(NiceFloat::from(f64::NAN).as_str(), "NaN");
	/// ```
	pub const NAN: Self = Self(FloatKind::NaN);

	/// # Zero.
	///
	/// A value representing zero.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// assert_eq!(NiceFloat::ZERO.as_str(), "0.00000000");
	/// assert_eq!(NiceFloat::from(0_f64).as_str(), "0.00000000");
	/// ```
	pub const ZERO: Self = Self(FloatKind::Normal(NiceInner::ZERO));

	#[must_use]
	#[inline]
	/// # Overflow.
	///
	/// This is used for values with integer components that do not fit within
	/// the `u64` range.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::{NiceFloat, NiceSeparator};
	///
	/// assert_eq!(
	///     NiceFloat::overflow(false, NiceSeparator::Comma).as_str(),
	///     "> 18,446,744,073,709,551,615",
	/// );
	/// assert_eq!(
	///     NiceFloat::overflow(true, NiceSeparator::Comma).as_str(),
	///     "< -18,446,744,073,709,551,615",
	/// );
	///
	/// // The same, triggered manually.
	/// assert_eq!(
	///     NiceFloat::from(f64::MAX).as_str(),
	///     "> 18,446,744,073,709,551,615",
	/// );
	/// assert_eq!(
	///     NiceFloat::from(-f64::MAX).as_str(),
	///     "< -18,446,744,073,709,551,615",
	/// );
	/// ```
	pub const fn overflow(neg: bool, sep: NiceSeparator) -> Self {
		Self(FloatKind::Overflow(NiceInner::overflow(neg, sep)))
	}
}

impl NiceFloat {
	#[must_use]
	/// # As Byte Slice.
	///
	/// Return the value as a byte slice.
	pub const fn as_bytes(&self) -> &[u8] {
		match self.0 {
			FloatKind::NaN => b"NaN",
			FloatKind::Normal(ref n) | FloatKind::Overflow(ref n) => n.as_bytes(),
			FloatKind::Infinity => &[226, 136, 158], // ∞
		}
	}

	#[must_use]
	/// # As String Slice.
	///
	/// Return the value as a string slice.
	pub const fn as_str(&self) -> &str {
		match self.0 {
			FloatKind::NaN => "NaN",
			FloatKind::Normal(ref n) | FloatKind::Overflow(ref n) => n.as_str(),
			FloatKind::Infinity => "∞",
		}
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
	pub const fn len(&self) -> usize {
		match self.0 {
			FloatKind::NaN | FloatKind::Infinity => 3,
			FloatKind::Normal(ref n) | FloatKind::Overflow(ref n) => n.len(),
		}
	}
}

impl NiceFloat {
	/// # Compact Raw.
	///
	/// This reslices a normal backing buffer for use by
	/// [`NiceFloat::compact_bytes`] and [`NiceFloat::comact_str`].
	const fn compact_raw(&self) -> Option<&[NiceChar]> {
		if let FloatKind::Normal(ref n) = self.0 {
			let (_, mut out) = n.data.split_at(n.from as usize);
			if 9 < out.len() {
				let mut trimmed = 0;
				while let [ rest @ .., last ] = out {
					if trimmed == 8 {
						out = rest;
						break;
					}
					else if matches!(*last, NiceChar::Digit0) {
						out = rest;
						trimmed += 1;
					}
					else { break; }
				}

				return Some(out);
			}
		}

		None
	}

	#[must_use]
	/// # Compact Bytes.
	///
	/// This returns a byte slice without trailing decimal zeroes. If the
	/// value has no fractional component at all, it will just return the
	/// integer portion.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// let nice = NiceFloat::from(12345.678_f64);
	/// assert_eq!(nice.as_bytes(), b"12,345.67800000");
	/// assert_eq!(nice.compact_bytes(), b"12,345.678"); // Some zeroes to trim.
	///
	/// let nice = NiceFloat::from(12340.0);
	/// assert_eq!(nice.as_bytes(), b"12,340.00000000");
	/// assert_eq!(nice.compact_bytes(), b"12,340"); // No fraction at all.
	///
	/// let nice = NiceFloat::from(12345.6783333333_f64);
	/// assert_eq!(nice.as_bytes(), b"12,345.67833333");
	/// assert_eq!(nice.compact_bytes(), b"12,345.67833333"); // Nothing to trim.
	/// ```
	pub const fn compact_bytes(&self) -> &[u8] {
		if let Some(chunk) = self.compact_raw() { NiceChar::as_bytes(chunk) }
		else { self.as_bytes() }
	}

	#[inline]
	#[must_use]
	/// # Compact String.
	///
	/// This returns a string slice without trailing decimal zeroes. If the
	/// value has no fractional component at all, it will just return the
	/// integer portion.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// let nice = NiceFloat::from(12345.678_f64);
	/// assert_eq!(nice.as_str(), "12,345.67800000");
	/// assert_eq!(nice.compact_str(), "12,345.678"); // Some zeroes to trim.
	///
	/// let nice = NiceFloat::from(12345.0);
	/// assert_eq!(nice.as_str(), "12,345.00000000");
	/// assert_eq!(nice.compact_str(), "12,345"); // No fraction at all.
	///
	/// let nice = NiceFloat::from(12345.6783333333_f64);
	/// assert_eq!(nice.as_str(), "12,345.67833333");
	/// assert_eq!(nice.compact_str(), "12,345.67833333"); // Nothing to trim.
	/// ```
	pub const fn compact_str(&self) -> &str {
		if let Some(chunk) = self.compact_raw() { NiceChar::as_str(chunk) }
		else { self.as_str() }
	}

	/// # Precise Raw.
	///
	/// This reslices a normal backing buffer for use by
	/// [`NiceFloat::precise_bytes`] and [`NiceFloat::precise_str`].
	const fn precise_raw(&self, precision: usize) -> Option<&[NiceChar]> {
		if let FloatKind::Normal(ref n) = self.0 {
			let (_, mut out) = n.data.split_at(n.from as usize);
			if 9 < out.len() && precision < 8 {
				if precision == 0 {
					(out, _) = out.split_at(out.len() - 9);
				}
				else {
					(out, _) = out.split_at(out.len() - (8 - precision));
				}

				return Some(out);
			}
		}

		None
	}

	#[inline]
	#[must_use]
	/// # Precise Bytes.
	///
	/// This truncates the fractional part to the desired number of places, and
	/// returns the corresponding byte slice.
	///
	/// If the precision is zero, only the integer portion will be returned.
	/// Precisions `>= 8` are meaningless, and return the equivalent of
	/// [`NiceFloat::as_bytes`].
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// let nice = NiceFloat::from(12345.678_f64);
	/// assert_eq!(nice.precise_bytes(0), b"12,345");
	/// assert_eq!(nice.precise_bytes(1), b"12,345.6");
	/// assert_eq!(nice.precise_bytes(2), b"12,345.67");
	/// assert_eq!(nice.precise_bytes(3), b"12,345.678");
	/// assert_eq!(nice.precise_bytes(4), b"12,345.6780");
	/// assert_eq!(nice.precise_bytes(5), b"12,345.67800");
	/// assert_eq!(nice.precise_bytes(6), b"12,345.678000");
	/// assert_eq!(nice.precise_bytes(7), b"12,345.6780000");
	/// assert_eq!(nice.precise_bytes(8), b"12,345.67800000");
	///
	/// // This has no effect on weird floats.
	/// assert_eq!(NiceFloat::NAN.precise_bytes(8), b"NaN");
	/// assert_eq!(NiceFloat::INFINITY.precise_bytes(8), "∞".as_bytes());
	/// ```
	pub const fn precise_bytes(&self, precision: usize) -> &[u8] {
		if let Some(chunk) = self.precise_raw(precision) {
			NiceChar::as_bytes(chunk)
		}
		else { self.as_bytes() }
	}

	#[inline]
	#[must_use]
	/// # Precise String.
	///
	/// This truncates the fractional part to the desired number of places, and
	/// returns the corresponding string slice.
	///
	/// If the precision is zero, only the integer portion will be returned.
	/// Precisions `>= 8` are meaningless, and return the equivalent of
	/// [`NiceFloat::as_str`].
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceFloat;
	///
	/// let nice = NiceFloat::from(12345.678_f64);
	/// assert_eq!(nice.precise_str(0), "12,345");
	/// assert_eq!(nice.precise_str(1), "12,345.6");
	/// assert_eq!(nice.precise_str(2), "12,345.67");
	/// assert_eq!(nice.precise_str(3), "12,345.678");
	/// assert_eq!(nice.precise_str(4), "12,345.6780");
	/// assert_eq!(nice.precise_str(5), "12,345.67800");
	/// assert_eq!(nice.precise_str(6), "12,345.678000");
	/// assert_eq!(nice.precise_str(7), "12,345.6780000");
	/// assert_eq!(nice.precise_str(8), "12,345.67800000");
	///
	/// // This has no effect on weird floats.
	/// assert_eq!(NiceFloat::NAN.precise_str(8), "NaN");
	/// assert_eq!(NiceFloat::INFINITY.precise_str(8), "∞");
	/// ```
	pub const fn precise_str(&self, precision: usize) -> &str {
		if let Some(chunk) = self.precise_raw(precision) {
			NiceChar::as_str(chunk)
		}
		else { self.as_str() }
	}
}

impl NiceFloat {
	#[must_use]
	/// # New Instance w/ Custom Separator.
	///
	/// Create a new instance, defining any arbitrary ASCII byte as the
	/// thousands separator, and another for the decimal point.
	///
	/// If you're good with American commas/periods, just use
	/// [`NiceFloat::from`] instead; it's faster.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::{NiceFloat, NiceSeparator};
	///
	/// // The default is commas for thousands, a period for top and bottom.
	/// assert_eq!(
	///     NiceFloat::from(1234.5678_f64).as_str(),
	///     "1,234.56780000",
	/// );
	///
	/// // Some places prefer the opposite.
	/// assert_eq!(
	///     NiceFloat::with_separator(
	///         1234.5678_f64,
	///         NiceSeparator::Period,
	///         NiceSeparator::Comma,
	///     ).as_str(),
	///     "1.234,56780000",
	/// );
	///
	/// // The punctuation is also honored for "special" values:
	/// assert_eq!(
	///     NiceFloat::with_separator(
	///         0_f64,
	///         NiceSeparator::Comma,
	///         NiceSeparator::Space,
	///     ).as_str(),
	///     "0 00000000",
	/// );
	/// assert_eq!(
	///     NiceFloat::with_separator(
	///         f64::MAX,
	///         NiceSeparator::Underscore,
	///         NiceSeparator::Period,
	///     ).as_str(),
	///     "> 18_446_744_073_709_551_615",
	/// );
	/// ```
	pub fn with_separator(num: f64, sep: NiceSeparator, dot: NiceSeparator) -> Self {
		match FloatKind::from64(num, sep) {
			Ok(kind) => Self(kind),
			Err((top, bottom, neg)) => {
				let sep = sep.as_nice_char();
				let dot = dot.as_nice_char();
				let mut out = NiceInner {
					data: data!(sep, dot),
					from: NiceFloatIdx::LAST,
				};
				out.parse(top, bottom, neg);
				Self(FloatKind::Normal(out))
			}
		}
	}
}

/// # Helper: Integer Division Methods.
macro_rules! div_int {
	// Generic documentation.
	(@doc_start $ty:expr, $fn:expr) => (
		concat!(
"# Divide Two `", $ty, "` as `f64`.

Recast two integers as floats and divide them, returning the result.

## Examples

```
use dactyl::NiceFloat;

assert_eq!(
    NiceFloat::", $fn, "(0, 13),
    Ok(0.0),
);
assert_eq!(
    NiceFloat::", $fn, "(13, 13),
    Ok(1.0),
    \"testing 13 / 13", $fn, "\"
);
assert_eq!(
    NiceFloat::", $fn, "(20, 16),
    Ok(1.25),
);
```

[`Result::Err`] is used to draw attention to weird/lossy values, such as
what happens when dividing by zero.

```
# use dactyl::NiceFloat;
assert!(
    NiceFloat::", $fn, "(0, 0).is_err_and(|e| e.is_nan()),
);
assert!(
    NiceFloat::", $fn, "(5, 0).is_err_and(|e| e.is_infinite()),
);
```
",
		)
	);

	// Generic documentation bottom.
	(@doc_end $fn:expr) => (
		concat!("
If you ultimately need a [`NiceFloat`], the result can be converted as usual.

```
# use dactyl::NiceFloat;
// Conditional niceness.
if let Ok(nice) = NiceFloat::", $fn, "(3, 100).map(NiceFloat::from) {
	assert_eq!(nice.as_str(), \"0.03000000\");
}

// Unconditional niceness.
assert_eq!(
    NiceFloat::from(NiceFloat::", $fn, "(3, 100)).as_str(),
    \"0.03000000\", // Result was good.
);
assert_eq!(
    NiceFloat::from(NiceFloat::", $fn, "(3, 0)).as_str(),
    \"∞\", // Result was infinite!
);
```

## Errors

The result is returned either way, but will come back as an error if
`NaN`, infinite, or the integer portion lost precision.
")
	);

	// Docs w/ middle.
	(@docs $ty:expr, $fn:expr, $mid:expr) => (
		concat!(
			div_int!(@doc_start $ty, $fn),
			$mid,
			div_int!(@doc_end $fn),
		)
	);

	// Docs w/o middle.
	(@docs $ty:expr, $fn:expr) => (
		concat!(
			div_int!(@doc_start $ty, $fn),
			div_int!(@doc_end $fn),
		)
	);

	// Small types that shouldn't need anything special.
	(@small $ty:ty, $fn:tt) => (
		#[doc = div_int!(@docs stringify!($ty), stringify!($fn))]
		pub const fn $fn(e: $ty, d: $ty) -> Result<f64, f64> {
			// Rule out stupid.
			if d == 0 {
				return Err(if e == 0 { f64::NAN } else { f64::INFINITY });
			}
			if e == 0 { return Ok(0.0); }

			let out = e as f64 / d as f64;
			if out.is_finite() { Ok(out) }
			else { Err(out) }
		}
	);

	// Big and unsigned.
	(@big $ty:ty, $fn:ident, $gcd:ident) => (
		#[doc = div_int!(@docs
			stringify!($ty),
			stringify!($fn),
			concat!(
				"
Integers with more than 15-16 digits — as can happen with `", stringify!($ty), "` —
can also be problematic.

```
# use dactyl::NiceFloat;
assert_eq!(
    NiceFloat::", stringify!($fn), "(9_223_372_036_854_775_806, 1),
    Err(9_223_372_036_854_776_000.0), // Almost!
);
```
")
		)]
		pub const fn $fn(mut e: $ty, mut d: $ty) -> Result<f64, f64> {
			// Rule out stupid.
			if d == 0 {
				return Err(if e == 0 { f64::NAN } else { f64::INFINITY });
			}
			if e == 0 { return Ok(0.0); }

			// Shrink the numbers to give them a better chance.
			if let Some(gcd) = $gcd(e, d) {
				e /= gcd;
				d /= gcd;
			}

			// Avoid pointless division.
			if d == 1 {
				let out = e as f64;
				if out.is_finite() && e == (out as $ty) {
					return Ok(out);
				}
				return Err(out);
			}

			// Try it and see what happens.
			let out = e as f64 / d as f64;
			if
				out.is_finite() &&
				e.wrapping_div(d).abs_diff(out as $ty) <= 1
			{
				Ok(out)
			}
			else { Err(out) }
		}
	);

	// Big and signed.
	(@big $ty:ty, $fn:ident, $gcd:ident, $tyu:ty, $fnu:ident) => (
		#[doc = div_int!(@docs
			stringify!($ty),
			stringify!($fn),
			concat!(
				"
Integers with more than 15-16 digits — as can happen with `", stringify!($ty), "` —
can also be problematic.

```
# use dactyl::NiceFloat;
assert_eq!(
    NiceFloat::", stringify!($fn), "(9_223_372_036_854_775_806, 1),
    Err(9_223_372_036_854_776_000.0), // Almost!
);
assert_eq!(
    NiceFloat::", stringify!($fn), "(-9_223_372_036_854_775_806, 1),
    Err(-9_223_372_036_854_776_000.0), // Almost!
);
```
")
		)]
		pub const fn $fn(mut e: $ty, mut d: $ty) -> Result<f64, f64> {
			// Rule out stupid.
			match (e.signum(), d.signum()) {
				(n, 0) => return
					if n == 0 { Err(f64::NAN) }
					else if n == -1 { Err(f64::NEG_INFINITY) }
					else { Err(f64::INFINITY) },

				(0, _) => return Ok(0.0),

				// If the result is going to be positive anyway, defer to the
				// unsigned sister method since it's safer.
				(-1, -1) | (1, 1) => return Self::$fnu(e.unsigned_abs(), d.unsigned_abs()),

				_ => {},
			}

			// Try to reduce.
			if let Some(gcd) = $gcd(e, d) {
				e /= gcd;
				d /= gcd;
			}

			// Avoid pointless division.
			if d.abs() == 1 {
				let out =
					if d == 1 { e as f64 }
					else if let Some(e) = e.checked_neg() { e as f64 }
					else { e as f64 * -1.0 };

				if out.is_finite() && e == (out as $ty) { return Ok(out); }
				return Err(out);
			}

			// Try it and see what happens.
			let out = e as f64 / d as f64;
			if
				out.is_finite() &&
				e.wrapping_div(d).unsigned_abs().abs_diff(out.abs() as $tyu) <= 1
			{
				Ok(out)
			}
			else { Err(out) }
		}
	);
}

#[expect(
	clippy::cast_precision_loss,
	clippy::cast_possible_truncation,
	reason = "We're trying not to let that happen…"
)]
#[expect(clippy::cast_sign_loss, reason = "False positive.")]
impl NiceFloat {
	div_int!(@small u8,   div_u8);
	div_int!(@small u16,  div_u16);
	div_int!(@small u32,  div_u32);
	div_int!(@big   u64,  div_u64,  gcd_u64);
	div_int!(@big   u128, div_u128, gcd_u128);

	div_int!(@small i8,   div_i8);
	div_int!(@small i16,  div_i16);
	div_int!(@small i32,  div_i32);
	div_int!(@big   i64,  div_i64,  gcd_i64,  u64,  div_u64);
	div_int!(@big   i128, div_i128, gcd_i128, u128, div_u128);

	#[inline]
	#[doc = div_int!(@docs "usize", "div_usize")]
	pub const fn div_usize(e: usize, d: usize) -> Result<f64, f64> {
		Self::div_u64(e as u64, d as u64)
	}

	#[inline]
	#[doc = div_int!(@docs "isize", "div_isize")]
	pub const fn div_isize(e: isize, d: isize) -> Result<f64, f64> {
		Self::div_i64(e as i64, d as i64)
	}
}



#[derive(Clone, Copy, Eq, PartialEq)]
/// # Float Type.
///
/// This enum provides basic float classification, used by [`NiceFloat`].
enum FloatKind {
	/// # Not a Number.
	NaN,

	/// # Overflow.
	///
	/// These values are fixed, but need buffering to accommodate custom
	/// thousands separators. Boo.
	Overflow(NiceInner),

	/// # Normal.
	///
	/// Normal numbers need a buffer.
	Normal(NiceInner),

	/// # Infinity.
	Infinity,
}

impl FloatKind {
	#[inline]
	/// # From `f32`.
	///
	/// Parse a float into a "special" type, or return the components required
	/// for us to parse it as normal.
	const fn from32(num: f32, sep: NiceSeparator) -> Result<Self, (u64, u32, bool)> {
		/// # Minimum Exponent.
		const MIN_EXP: i16 = 1 - (1 << 8) / 2;

		/// # Mantissa Mask.
		const MANT_MASK: u32 = (1 << 23) - 1;

		/// # Exponent Mask.
		const EXP_MASK: u32 = (1 << 8) - 1;

		const {
			assert!(f32::MANTISSA_DIGITS - 1 == 23, "Bug: wrong f32 mantissa count.");
		}

		// Quick weirdness checks.
		if num.is_nan() { return Ok(Self::NaN); }
		if num.is_infinite() { return Ok(Self::Infinity); }

		let bits = num.abs().to_bits();
		let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
		let exp = ((bits >> 23) & EXP_MASK) as i16 + MIN_EXP;

		let (top, bottom) =
			// Zero enough.
			if exp < -31 { return Err((0, 0, false)); }
			// Just a fraction.
			else if exp < 0 {
				let t = (mant as u64) << (41 + exp);
				(0, round_tie_even(23 + 41, t as u128))
			}
			// Both parts.
			else if exp < 23 {
				let top = (mant >> (23 - exp)) as u64;
				let bottom = round_tie_even(23, ((mant << exp) & MANT_MASK) as u128);
				(top, bottom)
			}
			// Just an integer.
			else if exp < 64 {
				let top = (mant as u64) << (exp - 23);
				(top, 0)
			}
			// Too big.
			else {
				return Ok(Self::Overflow(NiceInner::overflow(
					num.is_sign_negative(),
					sep,
				)));
			};

		// Done!
		Err((top, bottom, num.is_sign_negative()))
	}

	#[inline]
	/// # From `f64`.
	///
	/// Parse a float into a "special" type, or return the components required
	/// for us to parse it as normal.
	const fn from64(num: f64, sep: NiceSeparator) -> Result<Self, (u64, u32, bool)> {
		/// # Minimum Exponent.
		const MIN_EXP: i16 = 1 - (1 << 11) / 2;

		/// # Mantissa Mask.
		const MANT_MASK: u64 = (1 << 52) - 1;

		/// # Exponent Mask.
		const EXP_MASK: u64 = (1 << 11) - 1;

		const {
			assert!(f64::MANTISSA_DIGITS - 1 == 52, "Bug: wrong f64 mantissa count.");
		}

		// Quick weirdness checks.
		if num.is_nan() { return Ok(Self::NaN); }
		if num.is_infinite() { return Ok(Self::Infinity); }

		let bits = num.abs().to_bits();
		let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
		let exp = ((bits >> 52) & EXP_MASK) as i16 + MIN_EXP;

		let (top, bottom) =
			// Zero enough.
			if exp < -31 { return Err((0, 0, false)); }
			// Just a fraction (probably).
			else if exp < 0 {
				let bottom = round_tie_even(52 + 44, (mant as u128) << (44 + exp));

				if bottom == NiceFloatIdx::PRECISION { (1, 0) }
				else { (0, bottom) }
			}
			// Both parts (probably).
			else if exp < 52 {
				let top = mant >> (52 - exp);
				let bottom = round_tie_even(52, ((mant << exp) & MANT_MASK) as u128);

				if bottom == NiceFloatIdx::PRECISION { (top + 1, 0) }
				else { (top, bottom) }
			}
			// Just an integer.
			else if exp < 64 {
				let top = mant << (exp - 52);
				(top, 0)
			}
			// Too big.
			else {
				return Ok(Self::Overflow(NiceInner::overflow(
					num.is_sign_negative(),
					sep,
				)));
			};

		// Done!
		Err((top, bottom, num.is_sign_negative()))
	}
}



#[derive(Debug, Clone, Copy)]
/// # Nice (Buffered) Float.
///
/// This essentially does what all the other top-level `Nice*` structs do, but
/// is relegated to a private inner wrapper here because floats are a mess.
struct NiceInner {
	/// # String Buffer.
	data: [NiceChar; NiceFloatIdx::LEN],

	/// # Starting Position.
	///
	/// Data is written right to left.
	from: NiceFloatIdx,
}

impl Eq for NiceInner {}

impl PartialEq for NiceInner {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool { self.as_bytes() == rhs.as_bytes() }
}

impl NiceInner {
	/// # Zero.
	const ZERO: Self = Self {
		data: data!(),
		from: NiceFloatIdx::From26,
	};
}

impl NiceInner {
	#[must_use]
	/// # As Byte Slice.
	///
	/// Return the value as a byte slice.
	const fn as_bytes(&self) -> &[u8] {
		let (_, used) = self.data.split_at(self.from as usize);
		NiceChar::as_bytes(used)
	}

	#[must_use]
	/// # As String Slice.
	///
	/// Return the value as a string slice.
	const fn as_str(&self) -> &str {
		let (_, used) = self.data.split_at(self.from as usize);
		NiceChar::as_str(used)
	}

	#[must_use]
	/// # Length.
	///
	/// Return the length of the nice byte/string representation.
	///
	/// Note this will never be zero.
	const fn len(&self) -> usize {
		self.data.len() - self.from as usize
	}
}

impl NiceInner {
	/// # Overflow.
	const fn overflow(neg: bool, sep: NiceSeparator) -> Self {
		let sep = sep.as_nice_char();

		// Assume positive overflow.
		let mut data = [
			// Unused.
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0, NiceChar::Digit0,
			NiceChar::Gt, NiceChar::Space,
			NiceChar::Digit1, NiceChar::Digit8, sep,
			NiceChar::Digit4, NiceChar::Digit4, NiceChar::Digit6, sep,
			NiceChar::Digit7, NiceChar::Digit4, NiceChar::Digit4, sep,
			NiceChar::Digit0, NiceChar::Digit7, NiceChar::Digit3, sep,
			NiceChar::Digit7, NiceChar::Digit0, NiceChar::Digit9, sep,
			NiceChar::Digit5, NiceChar::Digit5, NiceChar::Digit1, sep,
			NiceChar::Digit6, NiceChar::Digit1, NiceChar::Digit5,
		];

		let from =
			if neg {
				// Patch the start to make it negative.
				data[9] = NiceChar::Dash;
				data[8] = NiceChar::Space;
				data[7] = NiceChar::Lt;
				NiceFloatIdx::From07
			}
			else { NiceFloatIdx::From08 };

		Self { data, from }
	}

	/// # Parse.
	fn parse(&mut self, top: u64, bottom: u32, neg: bool) {
		// Write the top.
		self.from = NiceFloatIdx::LAST;
		if let Some(digits) = Digiter::<u64>::new(top) {
			for (k, v) in NiceFloatIdx::TOP.into_iter().zip(digits) {
				self.data[k as usize] = v;
				self.from = k;
			}
		}

		// Write the minus sign.
		if neg {
			self.from = self.from.previous();
			self.data[self.from as usize] = NiceChar::Dash;
		}

		// Write the bottom.
		if let Some(mut digits) = Digiter::<u32>::new(bottom) {
			// Drain extra digits if our bottom is too big for our precision.
			if let Some(diff) = digits.len().checked_sub(8) {
				for _ in 0..diff { let _ = digits.next(); }
			}

			for (d, v) in digits.zip(self.data[NiceFloatIdx::BOTTOM_START as usize..].iter_mut().rev()) {
				*v = d;
			}
		}
	}
}



/// # Helper: GCD.
///
/// These methods are used to help reduce big-type numbers before
/// floatification, increasing the chances of their being represented
/// correctly.
///
/// The particulars are heavily borrowed from the `num` crate, but in our case
/// values are only returned if greater than one.
macro_rules! gcd {
	// Unsigned.
	(@unsigned $ty:ty, $fn:ident) => (
		/// # Greatest Common Divisor.
		const fn $fn(mut m: $ty, mut n: $ty) -> Option<$ty> {
			// Use Stein's algorithm
			if m == 0 || n == 0 {
				let out = m | n;
				if 1 < out { return Some(out); }
				return None;
			}

			// Find common factors of 2.
			let shift = (m | n).trailing_zeros();

			// Divide n and m by 2 until odd.
			m >>= m.trailing_zeros();
			n >>= n.trailing_zeros();

			while m != n {
				if m > n {
					m -= n;
					m >>= m.trailing_zeros();
				}
				else {
					n -= m;
					n >>= n.trailing_zeros();
				}
			}

			let out = m << shift;
			if 1 < out { Some(out) }
			else { None }
		}
	);

	// Signed.
	(@signed $ty:ty, $fn:ident) => (
		/// # Greatest Common Divisor.
		const fn $fn(mut m: $ty, mut n: $ty) -> Option<$ty> {
			// Use Stein's algorithm
			if m == 0 || n == 0 {
				let out = (m | n).abs();
				if 1 < out { return Some(out); }
				return None;
			}

			// Find common factors of 2.
			let shift = (m | n).trailing_zeros();

			// Positivity is required, but that won't work if the value is MIN.
			if m == <$ty>::MIN || n == <$ty>::MIN {
				// Workaround: .abs() can't infer the type from `out`.
				const ONE: $ty = 1;
				let out = (ONE << shift).abs();
				if 1 < out { return Some(out); }
				return None;
			}
			m = m.abs();
			n = n.abs();

			// Divide n and m by 2 until odd.
			m >>= m.trailing_zeros();
			n >>= n.trailing_zeros();

			while m != n {
				if m > n {
					m -= n;
					m >>= m.trailing_zeros();
				}
				else {
					n -= m;
					n >>= n.trailing_zeros();
				}
			}

			let out = m << shift;
			if 1 < out { Some(out) }
			else { None }
		}
	);
}

gcd!(@unsigned u64,  gcd_u64);
gcd!(@unsigned u128, gcd_u128);
gcd!(@signed   i64,  gcd_i64);
gcd!(@signed   i128, gcd_i128);



#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
/// # Round, Tie to Even.
///
/// Fractions are rounded on the ninth decimal place (to eight places).
/// `..=4` rounds down, `6..` rounds up. On `5` — a tie — rounding
/// heads toward an even value.
///
/// For example, `…25` rounds down to `…2`, while `…35` rounds up to `…4`.
///
/// Of course, this depends on the float having been faithfully stored to begin
/// with. If `…25` got turned into `…2477…` or whatever, _this_ rounding cycle
/// will be working from the wrong numbers.
///
/// Still, better than nothing!
const fn round_tie_even(offset: u128, tmp: u128) -> u32 {
	let tmp = NiceFloatIdx::PRECISION as u128 * tmp;
	let val = (tmp >> offset) as u32;

	let rem_mask = (1 << offset) - 1;
	let rem_msb_mask = 1 << (offset - 1);
	let rem = tmp & rem_mask;
	let is_tie = rem == rem_msb_mask;
	let is_even = (val & 1) == 0;
	let rem_msb = tmp & rem_msb_mask == 0;

	if rem_msb || (is_even && is_tie) { val }
	else { val + 1 }
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_nice() {
		// Some basic numbers.
		assert_eq!(NiceFloat::from(0_f64).as_str(), "0.00000000");
		assert_eq!(NiceFloat::from(-0_f64).as_str(), "0.00000000");
		assert_eq!(NiceFloat::from(-0.03_f64).as_str(), "-0.03000000");
		assert_eq!(NiceFloat::from(-10_f64).as_str(), "-10.00000000");
		assert_eq!(NiceFloat::from(1.03_f64).as_str(), "1.03000000");
		assert_eq!(NiceFloat::from(1.020_202_020_2_f64).as_str(), "1.02020202");
		assert_eq!(NiceFloat::from(-11_323.03_f64).as_str(), "-11,323.03000000");

		// Rounding.
		assert_eq!(NiceFloat::from(0.123_456_789_f64).as_str(), "0.12345679");
		assert_eq!(NiceFloat::from(1.000_000_046_f64).as_str(), "1.00000005");

		// Tie/Even Rounding.
		assert_eq!(NiceFloat::from(1.000_000_035_f64).as_str(), "1.00000004");
		assert_eq!(NiceFloat::from(1.000_000_044_f64).as_str(), "1.00000004");
		assert_eq!(NiceFloat::from(1.000_000_045_f64).as_str(), "1.00000004");
		assert_eq!(NiceFloat::from(1.000_000_055_f64).as_str(), "1.00000006");

		// Weird things.
		assert_eq!(NiceFloat::from(f64::NAN).as_str(), "NaN");
		assert_eq!(NiceFloat::from(f64::INFINITY).as_str(), "∞");
		assert_eq!(NiceFloat::from(f64::NEG_INFINITY).as_str(), "∞");
		assert_eq!(NiceFloat::from(1.0e-308_f64).as_str(), "0.00000000");
	}

	#[test]
	fn t_compact() {
		assert_eq!(NiceFloat::from(0_f64).compact_str(), "0");
		assert_eq!(
			NiceFloat::with_separator(
				0_f64,
				NiceSeparator::Space,
				NiceSeparator::Space,
			).compact_str(),
			"0",
		);
		assert_eq!(NiceFloat::from(0.010_200_3_f64).compact_str(), "0.0102003");
		assert_eq!(NiceFloat::from(0.000_000_01_f64).compact_str(), "0.00000001");
		assert_eq!(NiceFloat::from(0.000_000_001_f64).compact_str(), "0");

		// A few weird ones.
		assert_eq!(NiceFloat::from(f64::NAN).compact_str(), "NaN");
		assert_eq!(NiceFloat::from(f64::INFINITY).compact_str(), "∞");
		assert_eq!(
			NiceFloat::with_separator(
				f64::NAN,
				NiceSeparator::Space,
				NiceSeparator::Space,
			).compact_str(),
			"NaN",
		);
		assert_eq!(
			NiceFloat::with_separator(
				f64::INFINITY,
				NiceSeparator::Space,
				NiceSeparator::Space,
			).compact_str(),
			"∞",
		);
		assert_eq!(
			NiceFloat::overflow(true, NiceSeparator::Comma).compact_str(),
			"< -18,446,744,073,709,551,615",
		);
		assert_eq!(
			NiceFloat::overflow(false, NiceSeparator::Comma).compact_str(),
			"> 18,446,744,073,709,551,615",
		);
		assert_eq!(
			NiceFloat::with_separator(
				f64::MIN,
				NiceSeparator::Underscore,
				NiceSeparator::Period,
			).compact_str(),
			"< -18_446_744_073_709_551_615",
		);
		assert_eq!(
			NiceFloat::with_separator(
				f64::MAX,
				NiceSeparator::Apostrophe,
				NiceSeparator::Period,
			).compact_str(),
			"> 18'446'744'073'709'551'615",
		);
	}

	#[test]
	fn t_precise() {
		// Normal numbers are tested inline, but let's make sure zero works as
		// expected real quick.
		assert_eq!(NiceFloat::from(0_f64).precise_str(1), "0.0");
		assert_eq!(NiceFloat::from(0_f64).precise_str(0), "0");

		// A few weird ones.
		assert_eq!(NiceFloat::NAN.precise_str(3), "NaN");
		assert_eq!(NiceFloat::INFINITY.precise_str(3), "∞");
		assert_eq!(
			NiceFloat::overflow(true, NiceSeparator::Comma).precise_str(3),
			"< -18,446,744,073,709,551,615",
		);
		assert_eq!(
			NiceFloat::overflow(false, NiceSeparator::Comma).precise_str(3),
			"> 18,446,744,073,709,551,615",
		);
		assert_eq!(
			NiceFloat::with_separator(
				f64::MIN,
				NiceSeparator::Dash,
				NiceSeparator::Space,
			).precise_str(3),
			"< -18-446-744-073-709-551-615",
		);
		assert_eq!(
			NiceFloat::with_separator(
				f64::MAX,
				NiceSeparator::Dash,
				NiceSeparator::Space,
			).precise_str(3),
			"> 18-446-744-073-709-551-615",
		);
	}

	/// # Helper: div_int.
	///
	/// Float math is really hard to check programmatically. For this round,
	/// we're checking 5/4 and 4/5 at different scales to make sure they always
	/// come out as expected.
	macro_rules! t_div_int {
		($fnt:ident, $ty:ident, $fn:ident) => (
			#[test]
			fn $fnt() {
				use std::collections::BTreeSet;

				#[cfg(not(miri))]
				const SAMPLE_SIZE: usize = 1_000_000;

				#[cfg(miri)]
				const SAMPLE_SIZE: usize = 500;

				let max_five = <$ty>::MAX.wrapping_div(5);
				let mut rng = fastrand::Rng::new();
				let set = std::iter::repeat_with(|| rng.$ty(1..=max_five))
					.take(SAMPLE_SIZE)
					.chain(std::iter::once(max_five))
					.map(|scale| (5 * scale, 4 * scale))
					.collect::<BTreeSet<_>>();

				// Set has (n5, n4) pairs of all different sizes, so we
				// should get the same result no matter what we do?
				for (five, four) in set {
					assert_eq!(
						NiceFloat::$fn(five, four),
						Ok(1.25),
						"Failed for {five} / {four}",
					);
					assert_eq!(
						NiceFloat::$fn(four, five),
						Ok(0.8),
						"Failed for {four} / {five}",
					);

					// Either by itself should work or overflow.
					let res = NiceFloat::$fn(five, 1);
					assert!(
						res.ok().is_none_or(|n| n as $ty == five),
						"Failed for {five} / 1 ({res:?})",
					);
				}
			}
		);
	}

	t_div_int!(t_div_u8,    u8,    div_u8);
	t_div_int!(t_div_u16,   u16,   div_u16);
	t_div_int!(t_div_u32,   u32,   div_u32);
	t_div_int!(t_div_u64,   u64,   div_u64);
	t_div_int!(t_div_u128,  u128,  div_u128);
	t_div_int!(t_div_usize, usize, div_usize);

	t_div_int!(t_div_i8,    i8,    div_i8);
	t_div_int!(t_div_i16,   i16,   div_i16);
	t_div_int!(t_div_i32,   i32,   div_i32);
	t_div_int!(t_div_i64,   i64,   div_i64);
	t_div_int!(t_div_i128,  i128,  div_i128);
	t_div_int!(t_div_isize, isize, div_isize);

	#[test]
	fn t_div_int_weird() {
		let e =  92_23_372_036_854_775_807_u128;
		let d = 276_70_116_110_564_327_421_u128;
		let Ok(res) = NiceFloat::div_u128(e, d) else {
			panic!("Gigantic u128 division failed.");
		};
		assert!(
			// Precision will vary, but we should see roughly a third.
			format!("{res}").starts_with("0.33333"),
			"Gigantic u128 division doesn't start 0.33333.",
		);
	}
}
