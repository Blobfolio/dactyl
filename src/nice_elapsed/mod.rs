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
			fn as_ref(&self) -> &$ty { self.$cast() }
		}
		impl ::std::borrow::Borrow<$ty> for NiceElapsed {
			fn borrow(&self) -> &$ty { self.$cast() }
		}
	)+);
}

/// # Helper: Generate Impl
macro_rules! elapsed_from {
	($($type:ty),+) => ($(
		impl From<$type> for NiceElapsed {
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
	inner: [u8; SIZE],
	len: usize,
}

as_ref_borrow_cast!(as_bytes [u8], as_str str);

impl Default for NiceElapsed {
	#[inline]
	fn default() -> Self {
		Self {
			inner: [0; SIZE],
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
	#[allow(clippy::cast_possible_truncation)] // It fits.
	#[allow(clippy::cast_precision_loss)] // It fits.
	#[allow(clippy::cast_sign_loss)] // It is positive.
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
	fn from(src: Instant) -> Self { Self::from(src.elapsed()) }
}

impl From<u32> for NiceElapsed {
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
			//       0   •    s    e   c    o    n    d    s
			inner: [48, 32, 115, 101, 99, 111, 110, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			len: 9,
		}
	}

	#[deprecated(since = "0.4.3", note = "NiceElapsed now supports days; this method is now moot")]
	#[must_use]
	/// # Maximum Value
	///
	/// This returns a value that prints as `>1 day`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(NiceElapsed::max().as_str(), ">1 day");
	/// ```
	pub const fn max() -> Self {
		Self {
			//       >   1   •    d   a    y
			inner: [62, 49, 32, 100, 97, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			len: 6,
		}
	}

	#[allow(clippy::integer_division)] // It's fine.
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
				((num / 86_400) as u16, Self::hms(num % 86_400))
			};

		(d, h, m, s)
	}

	#[allow(clippy::cast_possible_truncation)] // Size is previously asserted.
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

	#[allow(unsafe_code)]
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
		// Safety: numbers and labels are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}

impl NiceElapsed {
	#[allow(clippy::cast_possible_truncation)] // We're checking first.
	#[allow(clippy::cast_sign_loss)] // Values are unsigned.
	#[allow(clippy::similar_names)] // It's that or the names become tedious.
	#[allow(unsafe_code)]
	/// # From DHMS.ms.
	///
	/// Build with days, hours, minutes, seconds, and milliseconds (hundredths).
	fn from_parts(d: u16, h: u8, m: u8, s: u8, ms: u8) -> Self {
		let has_d = 0 < d;
		let has_h = 0 < h;
		let has_m = 0 < m;
		let has_ms = 0 < ms;
		let has_s = has_ms || 0 < s;

		// How many elements are there?
		let total: u8 =
			u8::from(has_d) +
			u8::from(has_h) +
			u8::from(has_m) +
			u8::from(has_s);

		debug_assert!(
			0 < total,
			"BUG: NiceElapsed::from_parts should always have a part!"
		);

		let mut buf = [0_u8; SIZE];
		let mut end = buf.as_mut_ptr();
		let mut idx: u8 = 0;

		// Days.
		if has_d {
			idx += 1;

			// If this fits within a u8 range, it is much cheaper to cast down.
			if d <= 255 {
				end = ElapsedKind::Day.write(end, d as u8, idx, total);
			}
			// Otherwise we need to invoke NiceU16 to handle commas, etc.
			else {
				let tmp = NiceU16::from(d);
				let len = tmp.len();
				unsafe {
					std::ptr::copy_nonoverlapping(
						tmp.as_bytes().as_ptr(),
						end,
						len
					);
					end = end.add(len);
				}
				end = ElapsedKind::Day.write_label(end, d == 1);
			}
		}

		// Hours.
		if has_h {
			idx += 1;
			end = ElapsedKind::Hour.write(end, h, idx, total);
		}

		// Minutes.
		if has_m {
			idx += 1;
			end = ElapsedKind::Minute.write(end, m, idx, total);
		}

		// Seconds and/or milliseconds.
		if has_s {
			idx += 1;
			end = write_joiner(end, idx, total);
			end = unsafe { write_u8_advance(end, s, false) };

			if has_ms {
				unsafe {
					std::ptr::write(end, b'.');
					end = write_u8_advance(end.add(1), ms, true);
				}
			}

			end = ElapsedKind::Second.write_label(end, s == 1 && ms == 0);
		}

		// Put it all together!
		Self {
			inner: buf,
			len: unsafe { end.offset_from(buf.as_ptr()) as usize },
		}
	}
}



#[derive(Debug, Copy, Clone)]
/// # Unit Helpers.
///
/// This abstracts some of the verbosity of formatting.
enum ElapsedKind {
	Day,
	Hour,
	Minute,
	Second,
}

impl ElapsedKind {
	/// # Label.
	///
	/// Return the plural label with a leading space.
	const fn label(self) -> &'static [u8] {
		match self {
			Self::Day => b" days",
			Self::Hour => b" hours",
			Self::Minute => b" minutes",
			Self::Second => b" seconds",
		}
	}

	#[allow(unsafe_code)]
	/// # Write Joiner, Value, Label.
	fn write(self, mut dst: *mut u8, val: u8, idx: u8, total: u8) -> *mut u8 {
		dst = write_joiner(dst, idx, total);
		dst = unsafe { write_u8_advance(dst, val, false) };
		self.write_label(dst, val == 1)
	}

	#[allow(unsafe_code)]
	/// # Write Label.
	const fn write_label(self, dst: *mut u8, singular: bool) -> *mut u8 {
		let label = self.label();
		let len =
			if singular { label.len() - 1 }
			else { label.len() };

		unsafe {
			std::ptr::copy_nonoverlapping(label.as_ptr(), dst, len);
			dst.add(len)
		}
	}
}



#[allow(unsafe_code)]
/// # Write u8.
///
/// This will quickly write a `u8` number as a UTF-8 byte slice to the provided
/// pointer, and return a new pointer advanced to the next position (after
/// however many digits were written).
///
/// If `two == true`, a leading zero will be printed for single-digit values.
/// In practice, this only applies when writing milliseconds.
///
/// ## Safety
///
/// The pointer must have enough space for the value, i.e. 1-2 digits. This
/// isn't a problem in practice given the method calls are all private.
unsafe fn write_u8_advance(buf: *mut u8, num: u8, two: bool) -> *mut u8 {
	debug_assert!(num < 100, "BUG: write_u8_advance should always be under 100.");

	// Two digits.
	if two || 9 < num {
		std::ptr::copy_nonoverlapping(crate::double_ptr(num as usize), buf, 2);
		buf.add(2)
	}
	// One digit.
	else {
		std::ptr::write(buf, num + b'0');
		buf.add(1)
	}
}

#[allow(unsafe_code)]
/// # Write Joiner.
///
/// This will add commas and/or ands as necessary, based on how many entries
/// there are, and where we're at in that list.
const fn write_joiner(dst: *mut u8, idx: u8, total: u8) -> *mut u8 {
	// No joiner ever needed.
	if total < 2 || idx < 2 { dst }
	// We're at the end.
	else if idx == total {
		// Two items need a naked "and" between them.
		if 2 == total {
			unsafe {
				std::ptr::copy_nonoverlapping(b" and ".as_ptr(), dst, 5);
				dst.add(5)
			}
		}
		// More than two items need a "comma-and" between them.
		else {
			unsafe {
				std::ptr::copy_nonoverlapping(b", and ".as_ptr(), dst, 6);
				dst.add(6)
			}
		}
	}
	// We just need a comma.
	else if 2 < total {
		unsafe {
			std::ptr::copy_nonoverlapping(b", ".as_ptr(), dst, 2);
			dst.add(2)
		}
	}
	// No joiner needed this time.
	else { dst }
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
		_from(428390, "4 days, 22 hours, 59 minutes, and 50 seconds");
		_from(878428390, "10,166 days, 23 hours, 53 minutes, and 10 seconds");
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
		_from_d(Duration::from_millis(2101000), "35 minutes and 1 second");
		_from_d(Duration::from_millis(2101050), "35 minutes and 1.05 seconds");
		_from_d(Duration::from_millis(2121000), "35 minutes and 21 seconds");
		_from_d(Duration::from_millis(2121820), "35 minutes and 21.82 seconds");
		_from_d(Duration::from_nanos(2121999999999), "35 minutes and 21.99 seconds");

		_from_d(Duration::from_millis(3600000), "1 hour");
		_from_d(Duration::from_millis(3600300), "1 hour and 0.30 seconds");
		_from_d(Duration::from_millis(3660000), "1 hour and 1 minute");
		_from_d(Duration::from_millis(3661000), "1 hour, 1 minute, and 1 second");
		_from_d(Duration::from_millis(3661100), "1 hour, 1 minute, and 1.10 seconds");
		_from_d(Duration::from_millis(37732000), "10 hours, 28 minutes, and 52 seconds");
		_from_d(Duration::from_millis(37732030), "10 hours, 28 minutes, and 52.03 seconds");
		_from_d(Duration::from_millis(37740000), "10 hours and 29 minutes");
		_from_d(Duration::from_millis(37740030), "10 hours, 29 minutes, and 0.03 seconds");

		_from_d(Duration::from_millis(428390000), "4 days, 22 hours, 59 minutes, and 50 seconds");
		_from_d(Duration::from_millis(428390999), "4 days, 22 hours, 59 minutes, and 50.99 seconds");
		_from_d(Duration::from_millis(878428390999), "10,166 days, 23 hours, 53 minutes, and 10.99 seconds");
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
