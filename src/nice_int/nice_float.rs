/*!
# Dactyl: Nice Float.
*/

use crate::{
	Digiter,
	NiceWrapper,
};



/// # Total Buffer Size.
///
/// 1 sign + 18446744073709551615 + 6 commas + 1 decimal + 8 fractionals = 36 bytes.
const SIZE: usize = 36;

/// # Min Overflow From.
const MIN_OVERFLOW_FROM: usize = SIZE - 29;

/// # Max Overflow From.
const MAX_OVERFLOW_FROM: usize = SIZE - 28;

/// # Index for Dot.
const IDX_DOT: usize = 27; // 36 - 8 - 1.

/// # Precision Multiplier.
const PRECISION: u32 = 100_000_000;

/// # (Top) Digit Indices.
const INDICES: [usize; 20] = [26, 25, 24, 22, 21, 20, 18, 17, 16, 14, 13, 12, 10, 9, 8, 6, 5, 4, 2, 1];

/// # Generate Inner Buffer.
macro_rules! inner {
	($sep:expr) => ([b' ', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', $sep, b'0', b'0', b'0', b'.', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0']);
}



/// `NiceFloat` provides a quick way to convert an `f32` or `f64` (up to
/// the absolute equivalent of `u64::MAX`) into a formatted byte string for
/// e.g. printing.
///
/// Commas are added for every (integer) thousand; decimals are
/// rounded up to the nearest eight digits using a tie-to-even strategy.
///
/// Absolute values larger than `u64::MAX` will print as either
/// `> 18,446,744,073,709,551,615` or `< -18,446,744,073,709,551,615`.
///
/// Unlike the other `Nice*` helpers, this one supports negative values! It
/// also contains special handling for NaN and infinity, and comes with float-
/// specific formatting helpers like [`NiceFloat::compact_str`] and
/// [`NiceFloat::precise_str`].
///
/// That's it!
///
/// ## Examples
///
/// ```
/// use dactyl::NiceFloat;
///
/// let nice = NiceFloat::from(1234.5678_f64);
/// assert_eq!(nice.as_str(), "1,234.56780000");
/// assert_eq!(nice.compact_str(), "1,234.5678");
/// assert_eq!(nice.precise_str(2), "1,234.56");
/// ```
///
/// Rust floats are _imprecise_, so you may see some fractional weirdness.
/// Our previous example, lowered to 32 bits, illustrates the point:
///
/// ```
/// use dactyl::NiceFloat;
///
/// let nice = NiceFloat::from(1234.5678_f32);
/// assert_eq!(nice.as_str(), "1,234.56774902");        // .xxx8 == .xxx74092?
/// assert_eq!(1234.5678_f32.to_string(), "1234.5677"); // std::fmt is wrong too.
/// ```
///
/// ## Traits
///
/// Rustdoc doesn't do a good job at documenting type alias implementations, but
/// `NiceFloat` has a bunch, including:
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
///
/// You can instantiate a `NiceFloat` with:
///
/// * `From<f64>`
/// * `From<Option<f64>>`
/// * `From<f32>`
/// * `From<Option<f32>>`
/// * `From<FloatKind>`
///
/// When converting from a `None`, the result will be equivalent to zero.
pub type NiceFloat = NiceWrapper<SIZE>;

impl Default for NiceFloat {
	#[inline]
	fn default() -> Self { Self::ZERO }
}

impl From<f32> for NiceFloat {
	#[inline]
	fn from(num: f32) -> Self { Self::from(FloatKind::from32(num)) }
}

impl From<f64> for NiceFloat {
	#[inline]
	fn from(num: f64) -> Self { Self::from(FloatKind::from64(num)) }
}

impl From<FloatKind> for NiceFloat {
	#[inline]
	fn from(kind: FloatKind) -> Self {
		match kind {
			FloatKind::NaN => Self::NAN,
			FloatKind::Zero => Self::ZERO,
			FloatKind::Normal(top, bottom, neg) => {
				let mut out = Self::ZERO;
				out.parse(top, neg, bottom);
				out
			},
			FloatKind::Overflow(neg) => Self::overflow(neg),
			FloatKind::Infinity => Self::INFINITY,
		}
	}
}

impl NiceFloat {
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
	pub const INFINITY: Self = Self {
		inner: [
			b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
			b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
			b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
			b'0', b'0', b'0', 226, 136, 158,
		],
		from: SIZE - 3,
	};

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
	pub const NAN: Self = Self {
		inner: *b"000000000000000000000000000000000NaN",
		from: SIZE - 3,
	};

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
	pub const ZERO: Self = Self {
		inner: inner!(b','),
		from: IDX_DOT - 1,
	};

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
	/// use dactyl::NiceFloat;
	///
	/// assert_eq!(NiceFloat::from(f64::MAX).as_str(), "> 18,446,744,073,709,551,615");
	/// assert_eq!(NiceFloat::from(-f64::MAX).as_str(), "< -18,446,744,073,709,551,615");
	/// ```
	pub const fn overflow(neg: bool) -> Self {
		if neg {
			Self {
				inner: *b"0000000< -18,446,744,073,709,551,615",
				from: MIN_OVERFLOW_FROM,
			}
		}
		else {
			Self {
				inner: *b"00000000> 18,446,744,073,709,551,615",
				from: MAX_OVERFLOW_FROM,
			}
		}
	}

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
	/// use dactyl::NiceFloat;
	///
	/// assert_eq!(NiceFloat::from(1234.5678_f64).as_str(), "1,234.56780000");
	/// assert_eq!(
	///     NiceFloat::with_separator(1234.5678_f64, b'.', b',').as_str(),
	///     "1.234,56780000",
	/// );
	///
	/// // The punctuation is also honored for "special" values:
	/// assert_eq!(
	///     NiceFloat::with_separator(0_f64, b'.', b',').as_str(),
	///     "0,00000000",
	/// );
	/// assert_eq!(
	///     NiceFloat::with_separator(f64::MAX, b'.', b',').as_str(),
	///     "> 18.446.744.073.709.551.615",
	/// );
	/// ```
	///
	/// ## Panics
	///
	/// This method will panic if the separator is invalid ASCII.
	pub fn with_separator(num: f64, sep: u8, point: u8) -> Self {
		assert!(sep.is_ascii(), "Invalid separator.");
		assert!(point.is_ascii(), "Invalid decimal point.");

		match FloatKind::from64(num) {
			FloatKind::NaN => Self::NAN,
			FloatKind::Zero => {
				let mut out = Self::ZERO;
				out.inner[IDX_DOT] = point;
				out
			},
			FloatKind::Normal(top, bottom, neg) => {
				let mut out = Self {
					inner: inner!(sep),
					from: IDX_DOT - 1,
				};
				out.inner[IDX_DOT] = point;
				out.parse(top, neg, bottom);
				out
			},
			FloatKind::Overflow(neg) => {
				let mut out = Self::overflow(neg);
				if sep != b',' {
					for b in &mut out.inner {
						if b','.eq(b) { *b = sep; }
					}
				}
				out
			},
			FloatKind::Infinity => Self::INFINITY,
		}
	}
}

impl NiceFloat {
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
		let mut out = self.as_bytes();
		if self.from < IDX_DOT {
			let mut idx: u8 = 0;
			while let [rest @ .., last] = out {
				if idx == 8 {
					out = rest;
					break;
				}
				else if *last == b'0' { out = rest; }
				else { break; }
				idx += 1;
			}
		}
		out
	}

	#[expect(unsafe_code, reason = "Content is UTF-8.")]
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
		debug_assert!(
			std::str::from_utf8(self.compact_bytes()).is_ok(),
			"BUG: NiceFloat is not UTF-8?!",
		);

		// Safety: values are always ASCII, except for NiceFloat::INFINITY.
		unsafe { std::str::from_utf8_unchecked(self.compact_bytes()) }
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
		let mut out = self.as_bytes();
		if 9 < out.len() && precision < 8 && self.has_dot() {
			if precision == 0 {
				(out, _) = out.split_at(out.len() - 9);
			}
			else {
				(out, _) = out.split_at(out.len() - (8 - precision));
			}
		}
		out
	}

	#[expect(unsafe_code, reason = "Content is UTF-8.")]
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
		debug_assert!(
			std::str::from_utf8(self.precise_bytes(precision)).is_ok(),
			"BUG: NiceFloat is not UTF-8?!",
		);

		// Safety: values are always ASCII, except for NiceFloat::INFINITY.
		unsafe { std::str::from_utf8_unchecked(self.precise_bytes(precision)) }
	}
}

impl NiceFloat {
	/// # Has Dot?
	///
	/// This would be easy if we didn't allow customization, but, well, here we
	/// are. Haha.
	const fn has_dot(&self) -> bool {
		self.from < IDX_DOT &&
		! (
			self.from == MIN_OVERFLOW_FROM &&
			self.inner[MIN_OVERFLOW_FROM] == b'<'
		) &&
		! (
			self.from == MAX_OVERFLOW_FROM &&
			self.inner[MAX_OVERFLOW_FROM] == b'>'
		)
	}

	#[inline]
	/// # Parse Parts.
	fn parse(&mut self, top: u64, neg: bool, bottom: u32) {
		// Write the top.
		if let Some(digits) = Digiter::<u64>::new(top) {
			self.from = INDICES[digits.len() - 1];
			let Ok(indices) = self.inner.get_disjoint_mut(INDICES) else { unreachable!(); };
			for (d, v) in digits.zip(indices) { *v = d; }

			// Negative?
			if neg {
				self.from -= 1;
				self.inner[self.from] = b'-';
			}
		}

		// Write the bottom.
		if let Some(mut digits) = Digiter::<u32>::new(bottom) {
			// Drain extra digits if our bottom is too big for our precision.
			if let Some(diff) = digits.len().checked_sub(8) {
				for _ in 0..diff {
					let _ = digits.next();
				}
			}

			for (d, v) in digits.zip(self.inner[IDX_DOT + 1..].iter_mut().rev()) {
				*v = d;
			}
		}
	}
}



#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
/// # Float Type.
///
/// This enum provides basic float classification. It is used by [`NiceFloat`]
/// for formatting, but may be useful in other contexts too. Enjoy!
///
/// ## Examples
///
/// ```
/// use dactyl::FloatKind;
///
/// // Weird things.
/// assert_eq!(FloatKind::from(f64::NAN), FloatKind::NaN);
/// assert_eq!(FloatKind::from(f64::INFINITY), FloatKind::Infinity);
/// assert_eq!(FloatKind::from(f64::NEG_INFINITY), FloatKind::Infinity);
///
/// // Really big or small values can't be parsed out.
/// assert_eq!(FloatKind::from(f64::MIN), FloatKind::Overflow(true));
/// assert_eq!(FloatKind::from(f64::MAX), FloatKind::Overflow(false));
///
/// // Normal things.
/// assert_eq!(FloatKind::from(0_f32), FloatKind::Zero);
/// assert_eq!(FloatKind::from(123.456_f64), FloatKind::Normal(123, 45600000, false));
/// assert_eq!(FloatKind::from(-123.456_f64), FloatKind::Normal(123, 45600000, true));
/// ```
///
/// As mentioned elsewhere, Rust floats are _imprecise_. Any imprecision within
/// the original float will come through in the parsed `FloatKind`.
///
/// ```
/// use dactyl::FloatKind;
///
/// // This is right, but wrong. Haha.
/// assert_eq!(
///     FloatKind::from(1234.5678_f32),
///     FloatKind::Normal(1234, 56774902, false),
/// );
pub enum FloatKind {
	/// # Not a Number.
	NaN,

	#[default]
	/// # Zero.
	///
	/// This does not differentiate between positive and negative zero; they're
	/// just zero…
	Zero,

	/// # Normal.
	///
	/// This holds the integer and fractional parts of the float, along with a
	/// bool indicating whether or not it was negative.
	///
	/// The integer range must fit within `u64`; larger (absolute) values will
	/// fall back to [`FloatKind::Overflow`].
	///
	/// The fractional range holds up to eight digits, rounding on the ninth
	/// using a tie-to-even strategy.
	Normal(u64, u32, bool),

	/// # Overflow.
	///
	/// The value is normal, but is too big to be nicely split. The bool
	/// indicates whether or not the value is negative.
	Overflow(bool),

	/// # Infinity.
	///
	/// This does not differentiate between positive and negative infinity; the
	/// point is the numbers go on and on and on…
	Infinity,
}

impl From<f32> for FloatKind {
	#[inline]
	fn from(num: f32) -> Self { Self::from32(num) }
}

impl From<f64> for FloatKind {
	#[inline]
	fn from(num: f64) -> Self { Self::from64(num) }
}

impl FloatKind {
	#[must_use]
	/// # From `f32`.
	const fn from32(num: f32) -> Self {
		if num.is_nan() { Self::NaN }
		else if num.is_infinite() { Self::Infinity }
		else { parse_finite_f32(num) }
	}

	#[must_use]
	/// # From `f64`.
	const fn from64(num: f64) -> Self {
		if num.is_nan() { Self::NaN }
		else if num.is_infinite() { Self::Infinity }
		else { parse_finite_f64(num) }
	}
}



#[expect(clippy::integer_division, reason = "We want this.")]
/// # Parse Finite `f32`
///
/// This parses a float (that is not NaN or infinite) into the appropriate
/// [`FloatKind`].
///
/// This is essentially the same thing [`std::time::Duration`] does when
/// instantiating from fractional seconds.
const fn parse_finite_f32(num: f32) -> FloatKind {
	/// # Minimum Exponent.
	const MIN_EXP: i16 = 1 - (1 << 8) / 2;

	/// # Mantissa Mask.
	const MANT_MASK: u32 = (1 << 23) - 1;

	/// # Exponent Mask.
	const EXP_MASK: u32 = (1 << 8) - 1;

	let bits = num.abs().to_bits();
	let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
	let exp = ((bits >> 23) & EXP_MASK) as i16 + MIN_EXP;

	let (top, bottom) =
		// Zero enough.
		if exp < -31 { (0, 0) }
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
		else { return FloatKind::Overflow(num.is_sign_negative()); };

	// Done!
	if top == 0 && bottom == 0 { FloatKind::Zero }
	else { FloatKind::Normal(top, bottom, num.is_sign_negative()) }
}

#[expect(clippy::integer_division, reason = "We want this.")]
/// # Parse Finite `f64`
///
/// This parses a float (that is not NaN or infinite) into the appropriate
/// [`FloatKind`].
///
/// This is essentially the same thing [`std::time::Duration`] does when
/// instantiating from fractional seconds.
const fn parse_finite_f64(num: f64) -> FloatKind {
	/// # Minimum Exponent.
	const MIN_EXP: i16 = 1 - (1 << 11) / 2;

	/// # Mantissa Mask.
	const MANT_MASK: u64 = (1 << 52) - 1;

	/// # Exponent Mask.
	const EXP_MASK: u64 = (1 << 11) - 1;

	let bits = num.abs().to_bits();
	let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
	let exp = ((bits >> 52) & EXP_MASK) as i16 + MIN_EXP;

	let (top, bottom) =
		// Zero enough.
		if exp < -31 { (0, 0) }
		// Just a fraction (probably).
		else if exp < 0 {
			let bottom = round_tie_even(52 + 44, (mant as u128) << (44 + exp));

			if bottom == PRECISION { (1, 0) }
			else { (0, bottom) }
		}
		// Both parts (probably).
		else if exp < 52 {
			let top = mant >> (52 - exp);
			let bottom = round_tie_even(52, ((mant << exp) & MANT_MASK) as u128);

			if bottom == PRECISION { (top + 1, 0) }
			else { (top, bottom) }
		}
		// Just an integer.
		else if exp < 64 {
			let top = mant << (exp - 52);
			(top, 0)
		}
		// Too big.
		else { return FloatKind::Overflow(num.is_sign_negative()); };

	// Done!
	if top == 0 && bottom == 0 { FloatKind::Zero }
	else { FloatKind::Normal(top, bottom, num.is_sign_negative()) }
}



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
	let tmp = PRECISION as u128 * tmp;
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
	fn t_len() {
		for i in [0_f64, 1.1, 1.55, 123.456, -11_323.03] {
			let nice = NiceFloat::from(i);
			assert_eq!(nice.len(), nice.as_str().len());
			assert_eq!(nice.len(), nice.as_bytes().len());
			assert!(! nice.is_empty());
		}
	}

	#[test]
	fn t_digit_indices() {
		// Find the digit indices.
		let mut idx: Vec<usize> = inner!(b',').into_iter()
			.enumerate()
			.take(IDX_DOT) // Only applies to the top half.
			.filter_map(|(k, v)|
				if v == b'0' { Some(k) }
				else { None }
			)
			.collect();

		// Reverse it to match our constant.
		idx.reverse();

		// Now they should match!
		assert_eq!(INDICES.as_slice(), idx);
	}

	#[test]
	fn t_nice_float() {
		// Some basic numbers.
		assert_eq!(NiceFloat::from(0_f64).as_str(), "0.00000000");
		assert_eq!(NiceFloat::from(-0_f64).as_str(), "0.00000000");
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
		assert_eq!(NiceFloat::with_separator(0_f64, b'0', b'0').compact_str(), "0");
		assert_eq!(NiceFloat::from(0.010_200_3_f64).compact_str(), "0.0102003");
		assert_eq!(NiceFloat::from(0.000_000_01_f64).compact_str(), "0.00000001");
		assert_eq!(NiceFloat::from(0.000_000_001_f64).compact_str(), "0");

		// A few weird ones.
		assert_eq!(NiceFloat::from(f64::NAN).compact_str(), "NaN");
		assert_eq!(NiceFloat::from(f64::INFINITY).compact_str(), "∞");
		assert_eq!(NiceFloat::with_separator(f64::NAN, b'-', b'_').compact_str(), "NaN");
		assert_eq!(NiceFloat::with_separator(f64::INFINITY, b'-', b'_').compact_str(), "∞");
		assert_eq!(NiceFloat::overflow(true).compact_str(), "< -18,446,744,073,709,551,615");
		assert_eq!(NiceFloat::overflow(false).compact_str(), "> 18,446,744,073,709,551,615");
		assert_eq!(NiceFloat::with_separator(f64::MIN, b'!', b'?').compact_str(), "< -18!446!744!073!709!551!615");
		assert_eq!(NiceFloat::with_separator(f64::MAX, b'!', b'?').compact_str(), "> 18!446!744!073!709!551!615");
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
		assert_eq!(NiceFloat::overflow(true).precise_str(3), "< -18,446,744,073,709,551,615");
		assert_eq!(NiceFloat::overflow(false).precise_str(3), "> 18,446,744,073,709,551,615");
		assert_eq!(NiceFloat::with_separator(f64::MIN, b'!', b'?').precise_str(3), "< -18!446!744!073!709!551!615");
		assert_eq!(NiceFloat::with_separator(f64::MAX, b'!', b'?').precise_str(3), "> 18!446!744!073!709!551!615");
	}

	#[test]
	fn t_has_dot() {
		// Basic things should have dots.
		assert!(NiceFloat::from(0_f64).has_dot());
		assert!(NiceFloat::from(1.234_f64).has_dot());
		assert!(NiceFloat::with_separator(1.234_f64, b'!', b'?').has_dot());

		assert!(! NiceFloat::NAN.has_dot());
		assert!(! NiceFloat::INFINITY.has_dot());
		assert!(! NiceFloat::overflow(true).has_dot());
		assert!(! NiceFloat::overflow(false).has_dot());
		assert!(! NiceFloat::with_separator(f64::MIN, b'!', b'?').has_dot());
		assert!(! NiceFloat::with_separator(f64::MAX, b'!', b'?').has_dot());
	}
}
