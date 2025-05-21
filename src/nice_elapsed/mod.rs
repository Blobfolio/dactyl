/*!
# Dactyl: "Nice" Elapsed
*/

pub(super) mod clock;

use crate::{
	Digiter,
	NiceU16,
	traits::SaturatingFrom,
};
use std::{
	fmt,
	hash,
	ops::Deref,
	time::{
		Duration,
		Instant,
	},
};



/// # Array Size.
const SIZE: usize = 52;

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
/// For a more clock-like output, see [`NiceClock`](crate::NiceClock).
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

impl AsRef<str> for NiceElapsed {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl ::std::borrow::Borrow<str> for NiceElapsed {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

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
		<str as fmt::Display>::fmt(self.as_str(), f)
	}
}

impl Eq for NiceElapsed {}

impl From<Duration> for NiceElapsed {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
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

impl hash::Hash for NiceElapsed {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) { state.write(self.as_bytes()); }
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

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
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

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
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
	pub fn as_bytes(&self) -> &[u8] { &self.inner[..self.len] }

	#[expect(unsafe_code, reason = "Content is ASCII.")]
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
	/// # From Parts.
	///
	/// Construct from days, hours, minutes, and/or seconds.
	fn from_parts(d: u16, h: u8, m: u8, s: u8, ms: u8) -> Self {
		let mut parts = [Part::Day(d); 4];
		let mut len = usize::from(d != 0);
		if h != 0 {
			parts[len] = Part::Hour(h);
			len += 1;
		}
		if m != 0 {
			parts[len] = Part::Minute(m);
			len += 1;
		}
		if s != 0 || ms != 0 {
			parts[len] = Part::Second(s, ms);
			len += 1;
		}
		Self::from_parts_slice(&parts[..len])
	}

	/// # From Parts.
	///
	/// Construct from a sorted slice of non-zero time parts.
	fn from_parts_slice(parts: &[Part]) -> Self {
		/// # Write Glue.
		fn write(glue: &[u8], slice: &mut [u8]) -> usize {
			slice[..glue.len()].copy_from_slice(glue);
			glue.len()
		}

		let mut inner = [b' '; SIZE];
		match parts {
			[] => Self::min(),
			[a] => {
				let len = a.write_to_inner(&mut inner);
				Self { inner, len }
			},
			[a, b] => {
				let mut len = a.write_to_inner(&mut inner);
				len += write(b" and ", &mut inner[len..]);
				len += b.write_to_inner(&mut inner[len..]);
				Self { inner, len }
			},
			[rest @ .., b] => {
				let mut len = 0;
				for a in rest {
					len += a.write_to_inner(&mut inner[len..]);
					len += write(b", ", &mut inner[len..]);
				}
				len += write(b"and ", &mut inner[len..]);
				len += b.write_to_inner(&mut inner[len..]);
				Self { inner, len }
			},
		}
	}
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Time Part.
enum Part {
	/// # Day(s).
	Day(u16),

	/// # Hour(s).
	Hour(u8),

	/// # Minute(s).
	Minute(u8),

	/// # Second(s).
	Second(u8, u8),
}

impl Part {
	/// # Write to Slice.
	///
	/// Write the number and unit to the beginning of a slice, returning the
	/// length written.
	fn write_to_inner(self, slice: &mut [u8]) -> usize {
		/// # Do the Write.
		fn write(num: &[u8], label: &[u8], slice: &mut [u8]) -> usize {
			slice[num.len()..num.len() + label.len()].copy_from_slice(label);
			slice[..num.len()].copy_from_slice(num);
			num.len() + label.len()
		}

		match self {
			Self::Day(n) =>
				if n == 1 { write(b"1", b" day", slice) }
				else {
					let tmp = NiceU16::from(n);
					write(tmp.as_bytes(), b" days", slice)
				},
			Self::Hour(n) =>
				if n == 1 { write(b"1", b" hour", slice) }
				else {
					let tmp = Digiter(n).double();
					if tmp[0] == b'0' { write(&[tmp[1]], b" hours", slice) }
					else { write(tmp.as_slice(), b" hours", slice) }
				},
			Self::Minute(n) =>
				if n == 1 { write(b"1", b" minute", slice) }
				else {
					let tmp = Digiter(n).double();
					if tmp[0] == b'0' { write(&[tmp[1]], b" minutes", slice) }
					else { write(tmp.as_slice(), b" minutes", slice) }
				},
			Self::Second(n, 0) =>
				if n == 1 { write(b"1", b" second", slice) }
				else {
					let tmp = Digiter(n).double();
					if tmp[0] == b'0' { write(&[tmp[1]], b" seconds", slice) }
					else { write(tmp.as_slice(), b" seconds", slice) }
				},
			Self::Second(s, ms) => {
				let a = Digiter(s).double();
				let b = Digiter(ms).double();
				let tmp = [a[0], a[1], b'.', b[0], b[1]];
				if a[0] == b'0' { write(&tmp[1..], b" seconds", slice) }
				else { write(tmp.as_slice(), b" seconds", slice) }
			},
		}
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
