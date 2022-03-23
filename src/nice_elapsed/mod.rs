/*!
# Dactyl: "Nice" Elapsed

Note: this module is "in development". It is subject to change, and may eventually be spun off into its own crate.
*/

use crate::macros;
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
const SIZE: usize = 39;



/// # Helper: Generate Impl
macro_rules! elapsed_from {
	($($type:ty),+) => ($(
		impl From<$type> for NiceElapsed {
			fn from(num: $type) -> Self {
				// Nothing!
				if 0 == num { Self::min() }
				// Hours, minutes, and/or seconds.
				else if num < 86_400 {
					let [h, m, s] = Self::hms(num as u32);
					Self::from_hms(h, m, s)
				}
				// We're into days, which we don't do.
				else { Self::max() }
			}
		}
	)+);
}



#[derive(Clone, Copy)]
/// This is a very simple struct for efficiently converting a given number of
/// seconds (`u32`) into a nice, human-readable Oxford-joined byte string, like
/// `3 hours, 2 minutes, and 1 second`.
///
/// Note: days are unsupported, or more specifically, any value over `23:59:59`
/// (or `86400+` seconds) will return a fixed value of `>1 day`.
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
///
/// ## Note
///
/// This module is "in development". It is subject to change, and may eventually be spun off into its own crate.
pub struct NiceElapsed {
	inner: [u8; SIZE],
	len: usize,
}

macros::as_ref_borrow_cast!(
	NiceElapsed:
		as_bytes [u8],
		as_str str,
);

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
		f.debug_struct("NiceElapsed")
		 .field("inner", &self.inner.to_vec())
		 .field("len", &self.len)
		 .finish()
	}
}

macros::display_str!(as_str NiceElapsed);

impl Eq for NiceElapsed {}

impl From<Duration> for NiceElapsed {
	#[allow(clippy::cast_possible_truncation)] // It fits.
	#[allow(clippy::cast_precision_loss)] // It fits.
	#[allow(clippy::cast_sign_loss)] // It is positive.
	fn from(src: Duration) -> Self {
		let s = src.as_secs();
		if s < 86_400 {
			// Tease out the milliseconds as hundredths.
			let ms: u64 = (src.as_millis() as u64 - s * 1000).wrapping_div(10);

			if 0 < ms && ms < 100 {
				let [h, m, s] = Self::hms(s as u32);
				Self::from_hmsm(h, m, s, ms as u8)
			}
			// Cap precision to seconds.
			else { Self::from(s) }
		}
		// We're into days, which we don't do.
		else { Self::max() }
	}
}

impl From<Instant> for NiceElapsed {
	fn from(src: Instant) -> Self { Self::from(src.elapsed()) }
}

impl From<u32> for NiceElapsed {
	fn from(num: u32) -> Self {
		// Nothing!
		if 0 == num { Self::min() }
		// Hours, and maybe minutes and/or seconds.
		else if num < 86_400 {
			let [h, m, s] = Self::hms(num);
			Self::from_hms(h, m, s)
		}
		// We're into days, which we don't do.
		else { Self::max() }
	}
}

// These all work the same way.
elapsed_from!(usize, u64, u128);

impl Hash for NiceElapsed {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_bytes().hash(state);
	}
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
	pub const fn min() -> Self {
		Self {
			//       0   •    s    e   c    o    n    d    s
			inner: [48, 32, 115, 101, 99, 111, 110, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			len: 9,
		}
	}

	#[must_use]
	/// # Maximum Value
	///
	/// We can save some processing time by hard-coding the maximum value.
	/// Because `NiceElapsed` does not support days, this is equivalent to
	/// `86_400`, which comes out to `>1 day`.
	pub const fn max() -> Self {
		Self {
			//       >   1   •    d   a    y
			inner: [62, 49, 32, 100, 97, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			len: 6,
		}
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
	/// ## Example.
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
	pub fn as_bytes(&self) -> &[u8] { &self.inner[0..self.len] }

	#[must_use]
	#[inline]
	/// # As Str.
	///
	/// Return the nice value as a string slice.
	pub fn as_str(&self) -> &str {
		// Safety: numbers and labels are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}

impl NiceElapsed {
	#[allow(clippy::cast_sign_loss)] // The value is >= 0.
	/// # From HMS.
	///
	/// Build with hours, minutes, and seconds.
	fn from_hms(h: u8, m: u8, s: u8) -> Self {
		let has_h = 0 < h;
		let has_m = 0 < m;
		let has_s = 0 < s;

		let count: u8 = u8::from(has_h) + u8::from(has_m) + u8::from(has_s);

		let mut buf = [0_u8; SIZE];
		let mut end = buf.as_mut_ptr();

		// Hours.
		if has_h {
			end = unsafe { write_u8_advance(end, h) };
			end = ElapsedKind::Hour.write_label(end, h == 1);
			end = ElapsedKind::Hour.write_joiner(end, count, false);
		}

		// Minutes.
		if has_m {
			end = unsafe { write_u8_advance(end, m) };
			end = ElapsedKind::Minute.write_label(end, m == 1);
			end = ElapsedKind::Minute.write_joiner(end, count, has_h);
		}

		// Seconds.
		if has_s {
			end = unsafe { write_u8_advance(end, s) };
			end = ElapsedKind::Second.write_label(end, s == 1);
		}

		// Put it all together!
		Self {
			inner: buf,
			len: unsafe { end.offset_from(buf.as_ptr()) as usize },
		}
	}

	#[allow(clippy::cast_sign_loss)] // The value is >= 0.
	/// # From HMS.ms.
	///
	/// Build with hours, minutes, seconds, and milliseconds (hundredths).
	fn from_hmsm(h: u8, m: u8, s: u8, ms: u8) -> Self {
		let has_h = 0 < h;
		let has_m = 0 < m;

		// How many elements are there?
		let count: u8 = u8::from(has_h) + u8::from(has_m) + 1;

		let mut buf = [0_u8; SIZE];
		let mut end = buf.as_mut_ptr();

		// Hours.
		if has_h {
			end = unsafe { write_u8_advance(end, h) };
			end = ElapsedKind::Hour.write_label(end, h == 1);
			end = ElapsedKind::Hour.write_joiner(end, count, false);
		}

		// Minutes.
		if has_m {
			end = unsafe { write_u8_advance(end, m) };
			end = ElapsedKind::Minute.write_label(end, m == 1);
			end = ElapsedKind::Minute.write_joiner(end, count, has_h);
		}

		// Seconds and milliseconds. These always apply.
		end = unsafe { write_u8_advance(end, s) };
		unsafe { std::ptr::write(end, b'.'); }
		if ms < 10 {
			unsafe {
				std::ptr::write(end.add(1), b'0');
				std::ptr::write(end.add(2), ms + b'0');
				end = end.add(3);
			}
		}
		else {
			end = unsafe { write_u8_advance(end.add(1), ms) };
		}
		end = ElapsedKind::Second.write_label(end, false);

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
/// This abstracts some of the verbosity of formatting, allowing us to
/// instantiate [`NiceElapsed`] with an iterator.
enum ElapsedKind {
	Hour,
	Minute,
	Second,
}

impl ElapsedKind {
	/// # Label.
	///
	/// This is always plural, as singularity can be derived by reducing the
	/// length by one.
	const fn label(self) -> (usize, *const u8) {
		match self {
			Self::Hour => (6, b" hours".as_ptr()),
			Self::Minute => (8, b" minutes".as_ptr()),
			Self::Second => (8, b" seconds".as_ptr()),
		}
	}

	/// # Write Label.
	fn write_label(self, dst: *mut u8, singular: bool) -> *mut u8 {
		let (mut len, label) = self.label();
		if singular { len -= 1; }

		unsafe {
			std::ptr::copy_nonoverlapping(label, dst, len);
			dst.add(len)
		}
	}

	/// # Write Joiner.
	///
	/// This will add commas and/or ands as necessary.
	///
	/// The `any` bool is used to indicate whether or not a value has
	/// previously been printed. This affects the joiner of minutes, as it
	/// varies based on whether or not hours are involved.
	///
	/// Seconds and single-count values never write joiners.
	fn write_joiner(self, dst: *mut u8, count: u8, any: bool) -> *mut u8 {
		match (self, count, any) {
			(Self::Hour, 3, _) => {
				unsafe {
					std::ptr::copy_nonoverlapping(b", ".as_ptr(), dst, 2);
					dst.add(2)
				}
			},
			(Self::Hour, 2, _) | (Self::Minute, 2, false) => {
				unsafe {
					std::ptr::copy_nonoverlapping(b" and ".as_ptr(), dst, 5);
					dst.add(5)
				}
			},
			(Self::Minute, 3, _) => {
				unsafe {
					std::ptr::copy_nonoverlapping(b", and ".as_ptr(), dst, 6);
					dst.add(6)
				}
			},
			_ => { dst },
		}
	}
}



/// # Write u8.
///
/// This will quickly write a `u8` number as a UTF-8 byte slice to the provided
/// pointer.
///
/// ## Safety
///
/// The pointer must have enough space for the value, i.e. 1-2 digits. This
/// isn't a problem in practice given the method calls are all private.
unsafe fn write_u8_advance(buf: *mut u8, num: u8) -> *mut u8 {
	if 9 < num {
		std::ptr::copy_nonoverlapping(crate::double(num as usize), buf, 2);
		buf.add(2)
	}
	else {
		std::ptr::write(buf, num + b'0');
		buf.add(1)
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

		_from(428390, ">1 day");
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

		_from_d(Duration::from_millis(3600000), "1 hour");
		_from_d(Duration::from_millis(3600300), "1 hour and 0.30 seconds");
		_from_d(Duration::from_millis(3660000), "1 hour and 1 minute");
		_from_d(Duration::from_millis(3661000), "1 hour, 1 minute, and 1 second");
		_from_d(Duration::from_millis(3661100), "1 hour, 1 minute, and 1.10 seconds");
		_from_d(Duration::from_millis(37732000), "10 hours, 28 minutes, and 52 seconds");
		_from_d(Duration::from_millis(37732030), "10 hours, 28 minutes, and 52.03 seconds");
		_from_d(Duration::from_millis(37740000), "10 hours and 29 minutes");
		_from_d(Duration::from_millis(37740030), "10 hours, 29 minutes, and 0.03 seconds");

		_from_d(Duration::from_millis(428390000), ">1 day");
		_from_d(Duration::from_millis(428390999), ">1 day");
	}

	fn _from(num: u32, expected: &str) {
		assert_eq!(
			&*NiceElapsed::from(num),
			expected.as_bytes(),
			"{} should be equivalent to {:?}",
			num,
			expected
		);
	}

	fn _from_d(num: Duration, expected: &str) {
		assert_eq!(
			&*NiceElapsed::from(num),
			expected.as_bytes(),
			"{:?} should be equivalent to {:?}",
			num,
			expected
		);
	}
}
