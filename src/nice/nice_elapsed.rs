/*!
# Dactyl: Nice Elapsed.
*/

use crate::{
	NiceU16,
	traits::SaturatingFrom,
};
use std::{
	cmp::Ordering,
	time::{
		Duration,
		Instant,
	},
};
use super::{
	nice_uint,
	NiceChar,
};



#[derive(Clone, Copy)]
/// # Nice Elapsed.
///
/// This struct is used to efficiently convert some number of seconds (`u32`)
/// into a human-readable Oxford-joined list of parts, like
/// `"3 hours, 2 minutes, and 1 second"`.
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
	data: [NiceChar; 56],

	/// # Actual Length.
	len: usize,
}

nice_uint!(@traits NiceElapsed);

impl From<Duration> for NiceElapsed {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	fn from(src: Duration) -> Self {
		let s = src.as_secs();
		let ms =
			(src.as_millis() - u128::from(s) * 1000) // Only ms.
			.wrapping_div(10) // Truncate to max 2 digits (from possible 3).
			as u8; // 0-99 fits u8.

		// Nothing.
		if s == 0 && ms == 0 { Self::MIN }
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
		if 0 == num { Self::MIN }
		// Something.
		else {
			let (d, h, m, s) = Self::dhms(num);
			Self::from_parts(d, h, m, s, 0)
		}
	}
}

/// # Helper: Generate Impl
macro_rules! elapsed_from {
	($($ty:ty),+) => ($(
		impl From<$ty> for NiceElapsed {
			#[inline]
			/// This will never fail, however large values will be capped to
			/// [`u32::MAX`] before parsing, so may not reflect all the seconds
			/// you hoped they would.
			fn from(num: $ty) -> Self {
				// Nothing!
				if 0 == num { Self::MIN }
				// Something!
				else { Self::from(u32::saturating_from(num)) }
			}
		}
	)+);
}

// These all work the same way.
elapsed_from!(usize, u64, u128);

impl NiceElapsed {
	/// # Minimum Value
	///
	/// The smallest value [`NiceElapsed`] will render is `0`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	/// assert_eq!(NiceElapsed::MIN.as_str(), "0 seconds");
	/// ```
	pub const MIN: Self = Self {
		data: [
			NiceChar::Digit0,
			NiceChar::Space,
			NiceChar::LowerS, NiceChar::LowerE, NiceChar::LowerC, NiceChar::LowerO, NiceChar::LowerN, NiceChar::LowerD, NiceChar::LowerS,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space, NiceChar::Space,
			NiceChar::Space, NiceChar::Space, NiceChar::Space,
		],
		len: 9,
	};
}

impl NiceElapsed {
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
}

impl NiceElapsed {
	#[must_use]
	#[inline]
	/// # As Byte Slice.
	///
	/// Return the value as a byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	///
	/// assert_eq!(
	///     NiceElapsed::from(62_u32).as_bytes(),
	///     b"1 minute and 2 seconds"
	/// );
	/// ```
	pub const fn as_bytes(&self) -> &[u8] {
		let (out, _) = self.data.split_at(self.len);
		NiceChar::as_bytes(out)
	}

	#[must_use]
	#[inline]
	/// # As String Slice.
	///
	/// Return the value as a string slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	///
	/// assert_eq!(
	///     NiceElapsed::from(61_u32).as_str(),
	///     "1 minute and 1 second"
	/// );
	/// ```
	pub const fn as_str(&self) -> &str {
		let (out, _) = self.data.split_at(self.len);
		NiceChar::as_str(out)
	}

	#[must_use]
	#[inline]
	/// # Is Empty?
	///
	/// No! Haha. But for consistency, this method exists.
	pub const fn is_empty(&self) -> bool { false }

	#[must_use]
	#[inline]
	/// # Length.
	///
	/// Return the length of the byte/string representation of the value.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceElapsed;
	///
	/// let nice = NiceElapsed::from(73_u32);
	/// assert_eq!(
	///     nice.as_str(),
	///     "1 minute and 13 seconds",
	/// );
	/// assert_eq!(nice.len(), nice.as_str().len());
	/// assert_eq!(nice.len(), 23);
	/// ```
	pub const fn len(&self) -> usize { self.len }
}

impl NiceElapsed {
	/// # From Parts.
	///
	/// Construct the instance given the number of days, hours, etc.
	fn from_parts(d: u16, h: u8, m: u8, s: u8, ms: u8) -> Self {
		// We'll need a buffer whether or not we have any time parts to write,
		// but if we don't, MIN is returned, so we might as well start with
		// that!
		let mut out = Self::MIN;

		// What do we have?
		let has_d = match d.cmp(&1) {
			Ordering::Less => None,
			Ordering::Equal => Some(Unit::Day),
			Ordering::Greater => Some(Unit::Days),
		};
		let has_h = match h.cmp(&1) {
			Ordering::Less => None,
			Ordering::Equal => Some(Unit::Hour),
			Ordering::Greater => Some(Unit::Hours),
		};
		let has_m = match m.cmp(&1) {
			Ordering::Less => None,
			Ordering::Equal => Some(Unit::Minute),
			Ordering::Greater => Some(Unit::Minutes),
		};
		let has_s = match (s + ms * 2).cmp(&1) {
			Ordering::Less => None,
			Ordering::Equal => Some(Unit::Second),
			Ordering::Greater => Some(Unit::Seconds),
		};

		let total =
			u8::from(has_d.is_some()) +
			u8::from(has_h.is_some()) +
			u8::from(has_m.is_some()) +
			u8::from(has_s.is_some());
		if total == 0 { return out; }

		// Progress helpers.
		out.len = 0;
		let mut done = 0;

		// Days.
		if let Some(label) = has_d {
			let nice = NiceU16::from(d);
			let tmp = nice.as_bytes_raw();
			out.data[..tmp.len()].copy_from_slice(tmp);
			out.len += tmp.len();

			done += 1;
			let label = label.as_nice_chars(Glue::from_pos(1, total));
			out.data[out.len..out.len + label.len()].copy_from_slice(label);
			out.len += label.len();
		}

		// Hours.
		if let Some(label) = has_h {
			if 9 < h {
				out.data[out.len] = NiceChar::from_digit_u8(h / 10);
				out.data[out.len + 1] = NiceChar::from_digit_u8(h);
				out.len += 2;
			}
			else {
				out.data[out.len] = NiceChar::from_digit_u8(h);
				out.len += 1;
			}

			done += 1;
			let label = label.as_nice_chars(Glue::from_pos(done, total));
			out.data[out.len..out.len + label.len()].copy_from_slice(label);
			out.len += label.len();
		}

		// Minutes.
		if let Some(label) = has_m {
			if 9 < m {
				out.data[out.len] = NiceChar::from_digit_u8(m / 10);
				out.data[out.len + 1] = NiceChar::from_digit_u8(m);
				out.len += 2;
			}
			else {
				out.data[out.len] = NiceChar::from_digit_u8(m);
				out.len += 1;
			}

			done += 1;
			let label = label.as_nice_chars(Glue::from_pos(done, total));
			out.data[out.len..out.len + label.len()].copy_from_slice(label);
			out.len += label.len();
		}

		// Seconds and/or milliseconds.
		if let Some(label) = has_s {
			if 9 < s {
				out.data[out.len] = NiceChar::from_digit_u8(s / 10);
				out.data[out.len + 1] = NiceChar::from_digit_u8(s);
				out.len += 2;
			}
			else {
				out.data[out.len] = NiceChar::from_digit_u8(s);
				out.len += 1;
			}

			// Milliseconds too?
			if ms != 0 {
				out.data[out.len..out.len + 3].copy_from_slice(&[
					NiceChar::Period,
					NiceChar::from_digit_u8(ms / 10),
					NiceChar::from_digit_u8(ms),
				]);
				out.len += 3;
			}

			let label = label.as_nice_chars(None);
			out.data[out.len..out.len + label.len()].copy_from_slice(label);
			out.len += label.len();
		}

		// Done!
		out
	}
}



#[expect(clippy::missing_docs_in_private_items, reason = "Self-evident.")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Units.
///
/// This enum is used to identify time part labels. The singular variations
/// are only used if the value equals exactly `1`.
enum Unit {
	Day,
	Days,
	Hour,
	Hours,
	Minute,
	Minutes,
	Second,
	Seconds,
}

impl Unit {
	/// # As `NiceChar` Slice.
	///
	/// This returns a static slice for the label's characters — with leading
	/// space — and if there's glue, that too.
	const fn as_nice_chars(self, glue: Option<Glue>) -> &'static [NiceChar] {
		match self {
			Self::Day => match glue {
				None =>                 &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY],
				Some(Glue::Comma) =>    &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::Comma, NiceChar::Space],
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
			},
			Self::Days => match glue {
				None =>                 &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::LowerS],
				Some(Glue::Comma) =>    &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::LowerS, NiceChar::Comma, NiceChar::Space],
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::LowerS, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerD, NiceChar::LowerA, NiceChar::LowerY, NiceChar::LowerS, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
			},

			Self::Hour => match glue {
				None =>                 &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR],
				Some(Glue::Comma) =>    &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::Comma, NiceChar::Space],
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
			},
			Self::Hours => match glue {
				None =>                 &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::LowerS],
				Some(Glue::Comma) =>    &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::LowerS, NiceChar::Comma, NiceChar::Space],
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::LowerS, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerH, NiceChar::LowerO, NiceChar::LowerU, NiceChar::LowerR, NiceChar::LowerS, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
			},

			// If there are minutes, they'll either be penultimate or last, so
			// can't end with just a comma.
			Self::Minute => match glue {
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				_ =>                    &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE],
			},
			Self::Minutes => match glue {
				Some(Glue::CommaAnd) => &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE, NiceChar::LowerS, NiceChar::Comma, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				Some(Glue::And) =>      &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE, NiceChar::LowerS, NiceChar::Space, NiceChar::LowerA, NiceChar::LowerN, NiceChar::LowerD, NiceChar::Space],
				_ =>                    &[NiceChar::Space, NiceChar::LowerM, NiceChar::LowerI, NiceChar::LowerN, NiceChar::LowerU, NiceChar::LowerT, NiceChar::LowerE, NiceChar::LowerS],
			},

			// If there are seconds, they'll always be at the end so need no
			// trailing glue.
			Self::Second =>  &[NiceChar::Space, NiceChar::LowerS, NiceChar::LowerE, NiceChar::LowerC, NiceChar::LowerO, NiceChar::LowerN, NiceChar::LowerD],
			Self::Seconds => &[NiceChar::Space, NiceChar::LowerS, NiceChar::LowerE, NiceChar::LowerC, NiceChar::LowerO, NiceChar::LowerN, NiceChar::LowerD, NiceChar::LowerS],
		}
	}
}



#[expect(clippy::missing_docs_in_private_items, reason = "Self-evident.")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Glue.
///
/// This enum is used to identify the type of "glue" used to join time parts
/// in the output.
enum Glue {
	Comma,
	CommaAnd,
	And,
}

impl Glue {
	/// # From Position.
	///
	/// Determine the type of glue to inject between time parts, if any.
	///
	/// In short, if there are only two parts and we've just written one, it
	/// will be an " and ".
	///
	/// If there are more than two parts, it will be a ", " for items in the
	/// middle, and a ", and " before the last part.
	///
	/// Note: done will never be more than total, and neither will ever be
	/// zero.
	const fn from_pos(done: u8, total: u8) -> Option<Self> {
		match total.checked_sub(done) {
			// Done.
			Some(0) | None => None,
			// One more to go; break out the conjunction!
			Some(1) =>
				if total == 2 { Some(Self::And) }
				else { Some(Self::CommaAnd) },
			// N more to go; just chuck a comma on the end.
			Some(_) => Some(Self::Comma),
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_from() {
		from_(0, "0 seconds");
		from_(1, "1 second");
		from_(50, "50 seconds");

		from_(60, "1 minute");
		from_(61, "1 minute and 1 second");
		from_(100, "1 minute and 40 seconds");
		from_(2101, "35 minutes and 1 second");
		from_(2121, "35 minutes and 21 seconds");

		from_(3600, "1 hour");
		from_(3601, "1 hour and 1 second");
		from_(3602, "1 hour and 2 seconds");
		from_(3660, "1 hour and 1 minute");
		from_(3661, "1 hour, 1 minute, and 1 second");
		from_(3662, "1 hour, 1 minute, and 2 seconds");
		from_(3720, "1 hour and 2 minutes");
		from_(3721, "1 hour, 2 minutes, and 1 second");
		from_(3723, "1 hour, 2 minutes, and 3 seconds");
		from_(36001, "10 hours and 1 second");
		from_(36015, "10 hours and 15 seconds");
		from_(36060, "10 hours and 1 minute");
		from_(37732, "10 hours, 28 minutes, and 52 seconds");
		from_(37740, "10 hours and 29 minutes");

		from_(86400, "1 day");
		from_(86401, "1 day and 1 second");
		from_(86461, "1 day, 1 minute, and 1 second");
		from_(428_390, "4 days, 22 hours, 59 minutes, and 50 seconds");
		from_(878_428_390, "10,166 days, 23 hours, 53 minutes, and 10 seconds");
		from_(u32::MAX, "49,710 days, 6 hours, 28 minutes, and 15 seconds");
	}

	#[test]
	fn t_from_duration() {
		from_d_(Duration::from_millis(0), "0 seconds");
		from_d_(Duration::from_millis(1), "0 seconds");
		from_d_(Duration::from_millis(10), "0.01 seconds");
		from_d_(Duration::from_millis(100), "0.10 seconds");
		from_d_(Duration::from_millis(1000), "1 second");
		from_d_(Duration::from_millis(50000), "50 seconds");
		from_d_(Duration::from_millis(50020), "50.02 seconds");

		from_d_(Duration::from_millis(60000), "1 minute");
		from_d_(Duration::from_millis(60001), "1 minute");
		from_d_(Duration::from_millis(60340), "1 minute and 0.34 seconds");
		from_d_(Duration::from_millis(61000), "1 minute and 1 second");
		from_d_(Duration::from_millis(61999), "1 minute and 1.99 seconds");
		from_d_(Duration::from_millis(2_101_000), "35 minutes and 1 second");
		from_d_(Duration::from_millis(2_101_050), "35 minutes and 1.05 seconds");
		from_d_(Duration::from_millis(2_121_000), "35 minutes and 21 seconds");
		from_d_(Duration::from_millis(2_121_820), "35 minutes and 21.82 seconds");
		from_d_(Duration::from_nanos(2_121_999_999_999), "35 minutes and 21.99 seconds");

		from_d_(Duration::from_millis(3_600_000), "1 hour");
		from_d_(Duration::from_millis(3_600_300), "1 hour and 0.30 seconds");
		from_d_(Duration::from_millis(3_660_000), "1 hour and 1 minute");
		from_d_(Duration::from_millis(3_661_000), "1 hour, 1 minute, and 1 second");
		from_d_(Duration::from_millis(3_661_100), "1 hour, 1 minute, and 1.10 seconds");
		from_d_(Duration::from_millis(37_732_000), "10 hours, 28 minutes, and 52 seconds");
		from_d_(Duration::from_millis(37_732_030), "10 hours, 28 minutes, and 52.03 seconds");
		from_d_(Duration::from_millis(37_740_000), "10 hours and 29 minutes");
		from_d_(Duration::from_millis(37_740_030), "10 hours, 29 minutes, and 0.03 seconds");

		from_d_(Duration::from_millis(428_390_000), "4 days, 22 hours, 59 minutes, and 50 seconds");
		from_d_(Duration::from_millis(428_390_999), "4 days, 22 hours, 59 minutes, and 50.99 seconds");
		from_d_(Duration::from_millis(878_428_390_999), "10,166 days, 23 hours, 53 minutes, and 10.99 seconds");
	}

	fn from_(num: u32, expected: &str) {
		assert_eq!(
			NiceElapsed::from(num).as_bytes(),
			expected.as_bytes(),
			"{} should be equivalent to {:?}, not {:?}",
			num,
			expected,
			NiceElapsed::from(num).as_str(),
		);
	}

	fn from_d_(num: Duration, expected: &str) {
		assert_eq!(
			NiceElapsed::from(num).as_bytes(),
			expected.as_bytes(),
			"{:?} should be equivalent to {:?}, not {:?}",
			num,
			expected,
			NiceElapsed::from(num).as_str(),
		);
	}
}
