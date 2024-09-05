/*!
# Dactyl: "Nice" Elapsed
*/

use crate::{
	NiceU16,
	traits::SaturatingFrom,
};
use std::{
	fmt,
	hash::{
		Hash,
		Hasher,
	},
	ops::Deref,
	time::{
		Duration,
		Instant,
	},
};



/// # Array Size.
const SIZE: usize = 52;

/// # Helper: `AsRef` and `Borrow`.
macro_rules! as_ref_borrow_cast {
	($($cast:ident $ty:ty),+ $(,)?) => ($(
		impl AsRef<$ty> for NiceElapsed {
			#[inline]
			fn as_ref(&self) -> &$ty { self.$cast() }
		}
		impl ::std::borrow::Borrow<$ty> for NiceElapsed {
			#[inline]
			fn borrow(&self) -> &$ty { self.$cast() }
		}
	)+);
}

/// # Helper: Generate Impl
macro_rules! elapsed_from {
	($($type:ty),+) => ($(
		impl From<$type> for NiceElapsed {
			#[inline]
			/// This will never fail, however large values will be capped to
			/// [`u32::MAX`] before parsing, so may not reflect all the seconds
			/// you hoped they would.
			fn from(num: $type) -> Self {
				// Nothing!
				if 0 == num { Self::min() }
				// Something!
				else {
					Self::from(u32::saturating_from(num))
				}
			}
		}
	)+);
}



#[derive(Clone, Copy)]
/// This is a very simple struct for efficiently converting a given number of
/// seconds (`u32`) into a nice, human-readable Oxford-joined byte string, like
/// `3 hours, 2 minutes, and 1 second`.
///
/// The largest unit is days. The smallest unit is seconds, unless created with
/// `From<Duration>` or `From<Instant>`, in which case milliseconds (to two
/// decimal places) will be included, unless zero.
///
/// ## Examples
///
/// ```
/// use dactyl::NiceElapsed;
/// assert_eq!(
///     NiceElapsed::from(61_u32).as_str(),
///     "1 minute and 1 second"
/// );
/// ```
pub struct NiceElapsed {
	/// # Buffer.
	inner: [u8; SIZE],

	/// # Actual Length.
	len: usize,
}

impl AsRef<[u8]> for NiceElapsed {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

as_ref_borrow_cast!(as_str str);

impl Default for NiceElapsed {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [b' '; SIZE],
			len: 0,
		}
	}
}

impl Deref for NiceElapsed {
	type Target = [u8];
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_bytes() }
}

impl fmt::Debug for NiceElapsed {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("NiceElapsed")
			.field(&self.as_str())
			.finish()
	}
}

impl fmt::Display for NiceElapsed {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Eq for NiceElapsed {}

impl From<Duration> for NiceElapsed {
	#[allow(clippy::cast_possible_truncation)] // False positive.
	fn from(src: Duration) -> Self {
		let s = src.as_secs();
		let ms =
			(src.as_millis() - u128::from(s) * 1000) // Only ms.
			.wrapping_div(10) // Truncate to max 2 digits (from possible 3).
			as u8; // 0-99 fits u8.

		// Nothing.
		if s == 0 && ms == 0 { Self::min() }
		// Something.
		else {
			debug_assert!(ms < 100, "BUG: Milliseconds should never be more than two digits.");
			let (d, h, m, s) = Self::dhms(u32::saturating_from(s));
			Self::from_parts(d, h, m, s, ms)
		}
	}
}

impl From<Instant> for NiceElapsed {
	#[inline]
	fn from(src: Instant) -> Self { Self::from(src.elapsed()) }
}

impl From<u32> for NiceElapsed {
	#[inline]
	fn from(num: u32) -> Self {
		// Nothing!
		if 0 == num { Self::min() }
		// Something.
		else {
			let (d, h, m, s) = Self::dhms(num);
			Self::from_parts(d, h, m, s, 0)
		}
	}
}

// These all work the same way.
elapsed_from!(usize, u64, u128);

impl Hash for NiceElapsed {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.as_bytes()); }
}

impl PartialEq for NiceElapsed {
	#[inline]
	fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() }
}

impl NiceElapsed {
	#[must_use]
	#[inline]
	/// # Minimum Value
	///
	/// We can save some processing time by hard-coding the value for `0`,
	/// which comes out to `0 seconds`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(NiceElapsed::min().as_str(), "0 seconds");
	/// ```
	pub const fn min() -> Self {
		Self {
			inner: *b"0 seconds                                           ",
			len: 9,
		}
	}

	#[allow(clippy::cast_possible_truncation)] // False positive.
	#[must_use]
	/// # Time Chunks (with Days).
	///
	/// This works just like [`NiceElapsed::hms`], but counts up days too.
	///
	/// Note that unlike the time units, which have really small caps, days can
	/// reach up to `49,710`, so are returned as a `u16`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(NiceElapsed::dhms(1_123_321), (13_u16, 0_u8, 2_u8, 1_u8));
	/// assert_eq!(NiceElapsed::dhms(3661), (0_u16, 1_u8, 1_u8, 1_u8));
	/// ```
	pub const fn dhms(num: u32) -> (u16, u8, u8, u8) {
		let (d, [h, m, s]) =
			if num < 86_400 {
				(0, Self::hms(num))
			}
			else {
				(num.wrapping_div(86_400) as u16, Self::hms(num % 86_400))
			};

		(d, h, m, s)
	}

	#[allow(clippy::cast_possible_truncation)] // False positive.
	#[must_use]
	/// # Time Chunks.
	///
	/// This method splits seconds into hours, minutes, and seconds. Days are not
	/// supported; the maximum return value is `[23, 59, 59]`.
	///
	/// Given the limited range of digits involved, we're able to use some data
	/// rounding trickery to achieve conversion, bypassing the need for
	/// (relatively) expensive division and remainder calculations.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(NiceElapsed::hms(121), [0_u8, 2_u8, 1_u8]);
	/// ```
	pub const fn hms(mut num: u32) -> [u8; 3] {
		if num < 60 { [0, 0, num as u8] }
		else if num < 86399 {
			let mut buf = [0_u8; 3];

			// There are hours.
			if num >= 3600 {
				buf[0] = ((num * 0x91A3) >> 27) as u8;
				num -= buf[0] as u32 * 3600;
			}

			// There are minutes.
			if num >= 60 {
				buf[1] = ((num * 0x889) >> 17) as u8;
				buf[2] = (num - buf[1] as u32 * 60) as u8;
			}
			// There are seconds.
			else if num > 0 { buf[2] = num as u8; }

			buf
		}
		else { [23, 59, 59] }
	}

	#[must_use]
	#[inline]
	/// # As Bytes.
	///
	/// Return the nice value as a byte string.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(
	///     NiceElapsed::from(61_u32).as_bytes(),
	///     b"1 minute and 1 second"
	/// );
	/// ```
	pub fn as_bytes(&self) -> &[u8] { &self.inner[0..self.len] }

	#[allow(unsafe_code)] // Content is ASCII.
	#[must_use]
	#[inline]
	/// # As Str.
	///
	/// Return the nice value as a string slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(
	///     NiceElapsed::from(61_u32).as_str(),
	///     "1 minute and 1 second"
	/// );
	/// ```
	pub fn as_str(&self) -> &str {
		debug_assert!(self.as_bytes().is_ascii(), "Bug: NiceElapsed is not ASCII.");
		// Safety: numbers and labels are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}

impl NiceElapsed {
	#[allow(clippy::cast_possible_truncation)] // False positive.
	#[allow(clippy::many_single_char_names)]   // Consistency is preferred.
	#[allow(clippy::similar_names)]            // Consistency is preferred.
	/// # From DHMS.ms.
	///
	/// Build with days, hours, minutes, seconds, and milliseconds (hundredths).
	fn from_parts(d: u16, h: u8, m: u8, s: u8, ms: u8) -> Self {
		// Figure out which parts apply.
		let has_d = 0 != d;
		let has_h = 0 != h;
		let has_m = 0 != m;
		let has_ms = 0 != ms;
		let has_s = has_ms || 0 != s;

		// How many sections are there to write?
		let total: u8 =
			u8::from(has_d) +
			u8::from(has_h) +
			u8::from(has_m) +
			u8::from(has_s);

		// This shouldn't hit, but just in case.
		if total == 0 { return Self::min(); }

		let mut inner = [b' '; SIZE];
		let mut len = 0;
		let mut idx: u8 = 0;

		// Days.
		if has_d {
			idx += 1;

			// If the days are small, we can handle the digits like normal.
			if d < 10 {
				inner[0] = d as u8 + b'0';
				len = 1;
			}
			else if d < 100 {
				inner[..2].copy_from_slice(crate::double(d as usize).as_slice());
				len += 2;
			}
			else if d < 1000 {
				inner[..3].copy_from_slice(crate::triple(d as usize).as_slice());
				len += 3;
			}
			// Otherwise we'll need to leverage NiceU16.
			else {
				let tmp = NiceU16::from(d);
				len += tmp.len();
				inner[..len].copy_from_slice(tmp.as_bytes());
			}
			len += LabelKind::Day.write_to_slice(1 == d, idx, total, &mut inner[len..]);
		}

		// Hours.
		if has_h {
			idx += 1;
			len += write_u8_to_slice(h, &mut inner[len..]);
			len += LabelKind::Hour.write_to_slice(1 == h, idx, total, &mut inner[len..]);
		}

		// Minutes.
		if has_m {
			idx += 1;
			len += write_u8_to_slice(m, &mut inner[len..]);
			len += LabelKind::Minute.write_to_slice(1 == m, idx, total, &mut inner[len..]);
		}

		// Seconds.
		if has_s {
			idx += 1;
			len += write_u8_to_slice(s, &mut inner[len..]);

			// They might need milliseconds before the label.
			if has_ms {
				let [b, c] = crate::double(ms as usize);
				inner[len..len + 3].copy_from_slice(&[b'.', b, c]);
				len += 3;
			}

			len += LabelKind::Second.write_to_slice(1 == s && ! has_ms, idx, total, &mut inner[len..]);
		}

		Self { inner, len }
	}
}



#[derive(Debug, Clone, Copy)]
/// # Join Style.
///
/// The labels are written with their joins in one go. These are the different
/// options.
enum JoinKind {
	/// # No Join.
	None,

	/// # And Join.
	And,

	/// # Comma Join.
	Comma,

	/// # Comma/And Join.
	CommaAnd,
}



#[derive(Debug, Copy, Clone)]
/// # Labels.
///
/// This holds the different labels/units for each time part.
enum LabelKind {
	/// # Days.
	Day,

	/// # Hours.
	Hour,

	/// # Minutes.
	Minute,

	/// # Seconds.
	Second,
}

impl LabelKind {
	/// # Write Label to Slice.
	fn write_to_slice(self, singular: bool, idx: u8, total: u8, buf: &mut [u8]) -> usize {
		let join =
			// The last section needs no joiner.
			if idx == total { JoinKind::None }
			// If there are two sections, this must be the first, and simply
			// needs an " and ".
			else if total == 2 { JoinKind::And }
			// If this is the penultimate section (of more than two), we need
			// a comma and an and.
			else if idx + 1 == total { JoinKind::CommaAnd }
			// Otherwise just a comma.
			else { JoinKind::Comma };

		let new =
			if singular { self.as_bytes_singular(join) }
			else { self.as_bytes_plural(join) };

		let len = new.len();
		buf[..len].copy_from_slice(new);
		len
	}

	/// # As Bytes (Singular).
	const fn as_bytes_singular(self, join: JoinKind) -> &'static [u8] {
		match (self, join) {
			(Self::Day, JoinKind::And) => b" day and ",
			(Self::Day, JoinKind::Comma) => b" day, ",
			(Self::Day, _) => b" day",

			(Self::Hour, JoinKind::None) => b" hour",
			(Self::Hour, JoinKind::And) => b" hour and ",
			(Self::Hour, JoinKind::Comma) => b" hour, ",
			(Self::Hour, JoinKind::CommaAnd) => b" hour, and ",

			(Self::Minute, JoinKind::None) => b" minute",
			(Self::Minute, JoinKind::And) => b" minute and ",
			(Self::Minute, JoinKind::Comma) => b" minute, ",
			(Self::Minute, JoinKind::CommaAnd) => b" minute, and ",

			(Self::Second, _) => b" second",
		}
	}

	/// # As Bytes (Plural).
	const fn as_bytes_plural(self, join: JoinKind) -> &'static [u8] {
		match (self, join) {
			(Self::Day, JoinKind::And) => b" days and ",
			(Self::Day, JoinKind::Comma) => b" days, ",
			(Self::Day, _) => b" days",

			(Self::Hour, JoinKind::None) => b" hours",
			(Self::Hour, JoinKind::And) => b" hours and ",
			(Self::Hour, JoinKind::Comma) => b" hours, ",
			(Self::Hour, JoinKind::CommaAnd) => b" hours, and ",

			(Self::Minute, JoinKind::None) => b" minutes",
			(Self::Minute, JoinKind::And) => b" minutes and ",
			(Self::Minute, JoinKind::Comma) => b" minutes, ",
			(Self::Minute, JoinKind::CommaAnd) => b" minutes, and ",

			(Self::Second, _) => b" seconds",
		}
	}
}



#[inline]
/// # Write U8.
///
/// This converts a U8 to ASCII and writes it to the buffer without leading
/// zeroes, returning the length written.
fn write_u8_to_slice(num: u8, slice: &mut [u8]) -> usize {
	if 99 < num {
		slice[..3].copy_from_slice(crate::triple(num as usize).as_slice());
		3
	}
	else if 9 < num {
		slice[..2].copy_from_slice(crate::double(num as usize).as_slice());
		2
	}
	else {
		slice[0] = num + b'0';
		1
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_from() {
		_from(0, "0 seconds");
		_from(1, "1 second");
		_from(50, "50 seconds");

		_from(60, "1 minute");
		_from(61, "1 minute and 1 second");
		_from(100, "1 minute and 40 seconds");
		_from(2101, "35 minutes and 1 second");
		_from(2121, "35 minutes and 21 seconds");

		_from(3600, "1 hour");
		_from(3601, "1 hour and 1 second");
		_from(3602, "1 hour and 2 seconds");
		_from(3660, "1 hour and 1 minute");
		_from(3661, "1 hour, 1 minute, and 1 second");
		_from(3662, "1 hour, 1 minute, and 2 seconds");
		_from(3720, "1 hour and 2 minutes");
		_from(3721, "1 hour, 2 minutes, and 1 second");
		_from(3723, "1 hour, 2 minutes, and 3 seconds");
		_from(36001, "10 hours and 1 second");
		_from(36015, "10 hours and 15 seconds");
		_from(36060, "10 hours and 1 minute");
		_from(37732, "10 hours, 28 minutes, and 52 seconds");
		_from(37740, "10 hours and 29 minutes");

		_from(86400, "1 day");
		_from(86401, "1 day and 1 second");
		_from(86461, "1 day, 1 minute, and 1 second");
		_from(428_390, "4 days, 22 hours, 59 minutes, and 50 seconds");
		_from(878_428_390, "10,166 days, 23 hours, 53 minutes, and 10 seconds");
		_from(u32::MAX, "49,710 days, 6 hours, 28 minutes, and 15 seconds");
	}

	#[test]
	fn t_from_duration() {
		_from_d(Duration::from_millis(0), "0 seconds");
		_from_d(Duration::from_millis(1), "0 seconds");
		_from_d(Duration::from_millis(10), "0.01 seconds");
		_from_d(Duration::from_millis(100), "0.10 seconds");
		_from_d(Duration::from_millis(1000), "1 second");
		_from_d(Duration::from_millis(50000), "50 seconds");
		_from_d(Duration::from_millis(50020), "50.02 seconds");

		_from_d(Duration::from_millis(60000), "1 minute");
		_from_d(Duration::from_millis(60001), "1 minute");
		_from_d(Duration::from_millis(60340), "1 minute and 0.34 seconds");
		_from_d(Duration::from_millis(61000), "1 minute and 1 second");
		_from_d(Duration::from_millis(61999), "1 minute and 1.99 seconds");
		_from_d(Duration::from_millis(2_101_000), "35 minutes and 1 second");
		_from_d(Duration::from_millis(2_101_050), "35 minutes and 1.05 seconds");
		_from_d(Duration::from_millis(2_121_000), "35 minutes and 21 seconds");
		_from_d(Duration::from_millis(2_121_820), "35 minutes and 21.82 seconds");
		_from_d(Duration::from_nanos(2_121_999_999_999), "35 minutes and 21.99 seconds");

		_from_d(Duration::from_millis(3_600_000), "1 hour");
		_from_d(Duration::from_millis(3_600_300), "1 hour and 0.30 seconds");
		_from_d(Duration::from_millis(3_660_000), "1 hour and 1 minute");
		_from_d(Duration::from_millis(3_661_000), "1 hour, 1 minute, and 1 second");
		_from_d(Duration::from_millis(3_661_100), "1 hour, 1 minute, and 1.10 seconds");
		_from_d(Duration::from_millis(37_732_000), "10 hours, 28 minutes, and 52 seconds");
		_from_d(Duration::from_millis(37_732_030), "10 hours, 28 minutes, and 52.03 seconds");
		_from_d(Duration::from_millis(37_740_000), "10 hours and 29 minutes");
		_from_d(Duration::from_millis(37_740_030), "10 hours, 29 minutes, and 0.03 seconds");

		_from_d(Duration::from_millis(428_390_000), "4 days, 22 hours, 59 minutes, and 50 seconds");
		_from_d(Duration::from_millis(428_390_999), "4 days, 22 hours, 59 minutes, and 50.99 seconds");
		_from_d(Duration::from_millis(878_428_390_999), "10,166 days, 23 hours, 53 minutes, and 10.99 seconds");
	}

	fn _from(num: u32, expected: &str) {
		assert_eq!(
			&*NiceElapsed::from(num),
			expected.as_bytes(),
			"{} should be equivalent to {:?}, not {:?}",
			num,
			expected,
			NiceElapsed::from(num).as_str(),
		);
	}

	fn _from_d(num: Duration, expected: &str) {
		assert_eq!(
			&*NiceElapsed::from(num),
			expected.as_bytes(),
			"{:?} should be equivalent to {:?}, not {:?}",
			num,
			expected,
			NiceElapsed::from(num).as_str(),
		);
	}
}
