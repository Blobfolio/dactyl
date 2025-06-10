/*!
# Dactyl: Nice Clock.
*/

use std::{
	num::{
		NonZero,
		NonZeroU32,
	},
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
/// # Nice Clock.
///
/// This struct is used to efficiently convert some number of seconds into an
/// HH:MM:SS-formatted 24-hour clock-like string.
///
/// Counting begins at `00:00:00` and tops out `23:59:59`. Negative and
/// gigantic values are simply saturated to fit.
///
/// If you prefer more of a list-like structure or need support for days, see
/// [`NiceElapsed`](crate::NiceElapsed).
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
	data: [NiceChar; 8],
}

nice_uint!(@traits NiceClock);

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
	($($ty:ty),+ $(,)?) => ($(
		impl From<$ty> for NiceClock {
			#[inline]
			fn from(num: $ty) -> Self {
				if num <= 0 { Self::MIN }
				else { Self::from(num.cast_unsigned()) }
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
from_signed!(i8, i16, i32, i64, i128, isize);

impl From<Duration> for NiceClock {
	#[inline]
	fn from(src: Duration) -> Self { Self::from(src.as_secs()) }
}

impl From<Instant> for NiceClock {
	#[inline]
	fn from(src: Instant) -> Self { Self::from(src.elapsed()) }
}

impl From<u32> for NiceClock {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[expect(clippy::cast_lossless, reason = "For performance (inlining).")]
	#[inline]
	fn from(mut num: u32) -> Self {
		// Overflow.
		if 86_399 < num { return Self::MAX; }

		// Hours.
		let h =
			if num >= 3600 {
				let tmp = ((num * 0x91A3) >> 27) as u8;
				num -= tmp as u32 * 3600;
				[
					NiceChar::from_digit(tmp / 10),
					NiceChar::from_digit(tmp % 10),
				]
			}
			else { [NiceChar::Digit0, NiceChar::Digit0] };

		// Minutes.
		let m =
			if num >= 60 {
				let tmp = ((num * 0x889) >> 17) as u8;
				num -= tmp as u32 * 60;
				[
					NiceChar::from_digit(tmp / 10),
					NiceChar::from_digit(tmp % 10),
				]
			}
			else { [NiceChar::Digit0, NiceChar::Digit0] };

		// Seconds (and return).
		Self {
			data: [
				h[0], h[1], NiceChar::Colon,
				m[0], m[1], NiceChar::Colon,
				NiceChar::from_digit((num / 10) as u8),
				NiceChar::from_digit((num % 10) as u8),
			]
		}
	}
}

impl From<NonZeroU32> for NiceClock {
	#[inline]
	fn from(num: NonZeroU32) -> Self { Self::from(num.get()) }
}

impl From<NiceClock> for [u8; 8] {
	#[inline]
	fn from(src: NiceClock) -> Self { src.data.map(|b| b as u8) }
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
		data: [
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Colon,
			NiceChar::Digit0, NiceChar::Digit0, NiceChar::Colon,
			NiceChar::Digit0, NiceChar::Digit0,
		],
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
		data: [
			NiceChar::Digit2, NiceChar::Digit3, NiceChar::Colon,
			NiceChar::Digit5, NiceChar::Digit9, NiceChar::Colon,
			NiceChar::Digit5, NiceChar::Digit9,
		],
	};
}

impl NiceClock {
	#[must_use]
	#[inline]
	/// # As Byte Slice.
	///
	/// Return the value as a byte slice.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::from(90_u32).as_bytes(),
	///     b"00:01:30",
	/// );
	/// ```
	pub const fn as_bytes(&self) -> &[u8] {
		NiceChar::as_bytes(self.data.as_slice())
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
	/// use dactyl::NiceClock;
	///
	/// assert_eq!(
	///     NiceClock::from(125_u32).as_str(),
	///     "00:02:05",
	/// );
	/// ```
	pub const fn as_str(&self) -> &str {
		NiceChar::as_str(self.data.as_slice())
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
	/// The length of the string/byte output is fixed, so this always returns
	/// `8`.
	///
	/// ## Examples
	///
	/// ```
	/// use dactyl::NiceClock;
	///
	/// let nice = NiceClock::default();
	/// assert_eq!(nice.as_str(), "00:00:00");
	/// assert_eq!(nice.len(), nice.as_str().len());
	/// assert_eq!(nice.len(), 8);
	/// ```
	pub const fn len(&self) -> usize { 8 }

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
	pub const fn hours(self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.data[0] as u8 - b'0') * 10 + (self.data[1] as u8 - b'0')
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
	pub const fn minutes(self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.data[3] as u8 - b'0') * 10 + (self.data[4] as u8 - b'0')
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
	pub const fn seconds(self) -> u8 {
		// Working backwards isn't a big deal since we only have two digits to
		// worry about.
		(self.data[6] as u8 - b'0') * 10 + (self.data[7] as u8 - b'0')
	}
}

impl NiceClock {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
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
	pub const fn replace(&mut self, mut num: u32) {
		// Overflow.
		if 86_399 < num {
			self.data = Self::MAX.data;
			return;
		}

		// Hours.
		if num >= 3600 {
			let tmp = ((num * 0x91A3) >> 27) as u8;
			num -= tmp as u32 * 3600;
			self.data[0] = NiceChar::from_digit(tmp / 10);
			self.data[1] = NiceChar::from_digit(tmp % 10);
		}
		else {
			self.data[0] = NiceChar::Digit0;
			self.data[1] = NiceChar::Digit0;
		}

		// Minutes.
		if num >= 60 {
			let tmp = ((num * 0x889) >> 17) as u8;
			num -= tmp as u32 * 60;
			self.data[3] = NiceChar::from_digit(tmp / 10);
			self.data[4] = NiceChar::from_digit(tmp % 10);
		}
		else {
			self.data[3] = NiceChar::Digit0;
			self.data[4] = NiceChar::Digit0;
		}

		// Seconds.
		self.data[6] = NiceChar::from_digit((num / 10) as u8);
		self.data[7] = NiceChar::from_digit((num % 10) as u8);
	}
}



#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn t_nice_clock() {
		// Gather up the possible hour/minute/second combinations.
		let mut set = Vec::new();
		for h in 0..24_u32 {
			for m in 0..60_u32 {
				for s in 0..60_u32 {
					set.push([h, m, s]);
				}
			}
		}

		#[cfg(miri)]
		// Miri is too slow to check everything; let's shuffle and cut the
		// list to a more reasonable size.
		{
			fastrand::shuffle(&mut set);
			set.truncate(500);

			// Make sure the first and last are both present as our tests
			// will get messed up otherwise.
			set.push([0, 0, 0]);
			set.push([23, 59, 59]);
			set.sort();
			set.dedup();
		}

		let mut last = NiceClock::MIN;
		for [h, m, s] in set {
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
