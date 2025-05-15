/*!
# Dactyl: "Nice" Elapsed (Compact)
*/

use crate::NiceElapsed;
use std::{
	fmt,
	num::{
		NonZero,
		NonZeroU32,
	},
	ops::Deref,
	time::{
		Duration,
		Instant,
	},
};



/// # Minute Mask.
///
/// A lie for (computer) children: our h/m/s values can never exceed
/// 23/59/59 respectively, but the compiler doesn't always understand that.
/// This mask gives us a cheap way to let the compiler know these values cannot
/// overflow the ASCII lookup table (max index 99).
const TIME_MASK: u8 = 0b0011_1111;



#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # Nice Clock.
///
/// This struct is used to efficiently convert some number of seconds into an
/// HH:MM:SS-formatted 24-hour clock-like string.
///
/// Counting begins at `00:00:00` and tops out `23:59:59`. Negative and
/// gigantic values are simply saturated to fit.
///
/// If you prefer more of a list-like structure or need support for days, see
/// [`NiceElapsed`].
///
/// ## Examples
///
/// ```
/// use dactyl::NiceClock;
///
/// let mut clock = NiceClock::MIN;
/// assert_eq!(
///     clock.as_str(),
///     "00:00:00",
/// );
///
/// // Update the value in place.
/// clock.replace(99_u32);
/// assert_eq!(
///     clock.as_str(),
///     "00:01:39",
/// );
///
/// // It'll saturate for crazy values.
/// clock.replace(u32::MAX);
/// assert_eq!(
///     clock.as_str(),
///     "23:59:59",
/// );
///
/// // You can get the parts back as numbers too:
/// assert_eq!(clock.hours(), 23);
/// assert_eq!(clock.minutes(), 59);
/// assert_eq!(clock.seconds(), 59);
/// ```
pub struct NiceClock {
	/// # Formatted Data.
	inner: [u8; 8],
}

impl AsRef<[u8]> for NiceClock {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl AsRef<str> for NiceClock {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl ::std::borrow::Borrow<str> for NiceClock {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

impl Default for NiceClock {
	#[inline]
	fn default() -> Self { Self::MIN }
}

impl Deref for NiceClock {
	type Target = [u8];

	#[inline]
	fn deref(&self) -> &Self::Target { self.as_bytes() }
}

impl fmt::Debug for NiceClock {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("NiceClock")
			.field(&self.as_str())
			.finish()
	}
}

impl fmt::Display for NiceClock {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<str as fmt::Display>::fmt(self.as_str(), f)
	}
}

/// # Helper: From Small.
macro_rules! from_small {
	($($ty:ty),+ $(,)?) => ($(
		impl From<$ty> for NiceClock {
			#[inline]
			fn from(num: $ty) -> Self { Self::from(u32::from(num)) }
		}

		impl From<NonZero<$ty>> for NiceClock {
			#[inline]
			fn from(num: NonZero<$ty>) -> Self { Self::from(num.get()) }
		}
	)+);
}

/// # Helper: From Big.
macro_rules! from_big {
	($($ty:ty),+ $(,)?) => ($(
		impl From<$ty> for NiceClock {
			#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
			#[inline]
			fn from(num: $ty) -> Self {
				if num < 86_400 { Self::from(num as u32) }
				else { Self::MAX }
			}
		}

		impl From<NonZero<$ty>> for NiceClock {
			#[inline]
			fn from(num: NonZero<$ty>) -> Self { Self::from(num.get()) }
		}
	)+);
}

/// # Helper: From Signed.
macro_rules! from_signed {
	($($ty:ty, $unsigned:ty),+ $(,)?) => ($(
		impl From<$ty> for NiceClock {
			#[expect(clippy::cast_sign_loss, reason = "False positive.")]
			#[inline]
			fn from(num: $ty) -> Self {
				if num <= 0 { Self::MIN }
				else { Self::from(num as $unsigned) }
			}
		}

		impl From<NonZero<$ty>> for NiceClock {
			#[inline]
			fn from(num: NonZero<$ty>) -> Self { Self::from(num.get()) }
		}
	)+);
}

from_small!(u8, u16);
from_big!(u64, u128, usize);
from_signed!(
	i8, u8,
	i16, u16,
	i32, u32,
	i64, u64,
	i128, u128,
	isize, usize,
);

impl From<Duration> for NiceClock {
	#[inline]
	fn from(src: Duration) -> Self { Self::from(src.as_secs()) }
}

impl From<Instant> for NiceClock {
	#[inline]
	fn from(src: Instant) -> Self { Self::from(src.elapsed()) }
}

impl From<u32> for NiceClock {
	#[inline]
	fn from(num: u32) -> Self {
		let [h, m, s] = NiceElapsed::hms(num);
		let h = crate::double(usize::from(h & TIME_MASK));
		let m = crate::double(usize::from(m & TIME_MASK));
		let s = crate::double(usize::from(s & TIME_MASK));
		Self {
			inner: [h[0], h[1], b':', m[0], m[1], b':', s[0], s[1]],
		}
	}
}

impl From<NonZeroU32> for NiceClock {
	#[inline]
	fn from(num: NonZeroU32) -> Self { Self::from(num.get()) }
}

impl From<NiceClock> for [u8; 8] {
	#[inline]
	fn from(num: NiceClock) -> Self { num.inner }
}

impl NiceClock {
	/// # Minimum Value.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::MIN.as_str(),
	///     "00:00:00",
	/// );
	///
	/// assert_eq!(
	///     NiceClock::from(0_u32).as_str(),
	///     "00:00:00",
	/// );
	/// ```
	pub const MIN: Self = Self {
		inner: *b"00:00:00",
	};

	/// # Maximum Value.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::MAX.as_str(),
	///     "23:59:59"
	/// );
	///
	/// assert_eq!(
	///     NiceClock::from(u32::MAX).as_str(),
	///     "23:59:59",
	/// );
	/// ```
	pub const MAX: Self = Self {
		inner: *b"23:59:59",
	};
}

impl NiceClock {
	#[inline]
	/// # Replace.
	///
	/// Update the clock time, in place.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// let mut clock = NiceClock::from(1_u32);
	/// assert_eq!(clock.as_str(), "00:00:01");
	///
	/// clock.replace(2);
	/// assert_eq!(clock.as_str(), "00:00:02");
	/// ```
	pub fn replace(&mut self, num: u32) {
		let [h, m, s] = NiceElapsed::hms(num);
		let h = crate::double(usize::from(h & TIME_MASK));
		let m = crate::double(usize::from(m & TIME_MASK));
		let s = crate::double(usize::from(s & TIME_MASK));
		self.inner[0] = h[0];
		self.inner[1] = h[1];
		self.inner[3] = m[0];
		self.inner[4] = m[1];
		self.inner[6] = s[0];
		self.inner[7] = s[1];
	}
}

impl NiceClock {
	#[must_use]
	/// # As Bytes.
	///
	/// Return the formatted value as a byte slice.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::from(90_u32).as_bytes(),
	///     b"00:01:30",
	/// );
	/// ```
	pub const fn as_bytes(&self) -> &[u8] { self.inner.as_slice() }

	#[expect(unsafe_code, reason = "For performance.")]
	#[must_use]
	/// # As String.
	///
	/// Return the formatted value as a string slice.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::from(125_u32).as_str(),
	///     "00:02:05",
	/// );
	/// ```
	pub const fn as_str(&self) -> &str {
		// Safety: all bytes are ASCII.
		unsafe { std::str::from_utf8_unchecked(self.inner.as_slice()) }
	}

	#[must_use]
	/// # Hours.
	///
	/// Return the hours part as a number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// let clock = NiceClock::from(12345_usize);
	/// assert_eq!(
	///     clock.as_str(),
	///     "03:25:45",
	/// );
	/// assert_eq!(clock.hours(), 3);
	/// ```
	pub const fn hours(&self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.inner[0] - b'0') * 10 + (self.inner[1] - b'0')
	}

	#[must_use]
	/// # Minutes.
	///
	/// Return the minutes part as a number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// let clock = NiceClock::from(12345_usize);
	/// assert_eq!(
	///     clock.as_str(),
	///     "03:25:45",
	/// );
	/// assert_eq!(clock.minutes(), 25);
	/// ```
	pub const fn minutes(&self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.inner[3] - b'0') * 10 + (self.inner[4] - b'0')
	}

	#[must_use]
	/// # Seconds.
	///
	/// Return the seconds part as a number.
	///
	/// ## Examples.
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// let clock = NiceClock::from(12345_usize);
	/// assert_eq!(
	///     clock.as_str(),
	///     "03:25:45",
	/// );
	/// assert_eq!(clock.seconds(), 45);
	/// ```
	pub const fn seconds(&self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.inner[6] - b'0') * 10 + (self.inner[7] - b'0')
	}
}



#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn t_nice_clock() {
		let mut last = NiceClock::MIN;
		for h in 0..24_u32 {
			for m in 0..60_u32 {
				for s in 0..60_u32 {
					let total = s + m * 60 + h * 60 * 60;
					let clock = NiceClock::from(total);
					assert_eq!(clock.as_str(), format!("{h:02}:{m:02}:{s:02}"));

					// Check replacements too.
					if total == 0 { assert_eq!(last, clock); }
					else { assert_ne!(last, clock); }
					last.replace(total);
					assert_eq!(last, clock); // Should be the same now.

					// Check the bigger types.
					assert_eq!(clock, NiceClock::from(u64::from(total)));
					assert_eq!(clock, NiceClock::from(u128::from(total)));
				}
			}
		}

		// Check big saturating.
		assert_eq!(last, NiceClock::MAX);
		assert_eq!(last, NiceClock::from(i128::MAX));
		assert_eq!(last, NiceClock::from(i32::MAX));
		assert_eq!(last, NiceClock::from(i64::MAX));
		assert_eq!(last, NiceClock::from(isize::MAX));
		assert_eq!(last, NiceClock::from(u128::MAX));
		assert_eq!(last, NiceClock::from(u32::MAX));
		assert_eq!(last, NiceClock::from(u64::MAX));
		assert_eq!(last, NiceClock::from(usize::MAX));

		// Check negative saturating.
		last.replace(0);
		assert_eq!(last, NiceClock::MIN);
		assert_eq!(last, NiceClock::from(i8::MIN));
		assert_eq!(last, NiceClock::from(i16::MIN));
		assert_eq!(last, NiceClock::from(i32::MIN));
		assert_eq!(last, NiceClock::from(i64::MIN));
		assert_eq!(last, NiceClock::from(i128::MIN));
		assert_eq!(last, NiceClock::from(isize::MIN));
	}
}
