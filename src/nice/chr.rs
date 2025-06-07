/*!
# Dactyl: Characters.
*/



#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[expect(missing_docs, reason = "Self-explanatory.")]
/// # Nice Thousands Separator.
///
/// The variants in this enum are used for thousand separation in the various
/// `NiceU*` structs.
pub enum NiceSeparator {
	Apostrophe = b'\'',
	Comma      = b',',
	Dash       = b'-',
	Period     = b'.',
	Space      = b' ',
	Underscore = b'_',
}

impl NiceSeparator {
	/// # Into Nice Char.
	pub(super) const fn as_nice_char(self) -> NiceChar {
		match self {
			Self::Apostrophe => NiceChar::Apostrophe,
			Self::Comma => NiceChar::Comma,
			Self::Dash => NiceChar::Dash,
			Self::Period => NiceChar::Period,
			Self::Space => NiceChar::Space,
			Self::Underscore => NiceChar::Underscore,
		}
	}
}



/// # Helper: `NiceChar` Definition.
macro_rules! nice_chars {
	($($k:ident $v:literal),+ $(,)*) => (
		#[repr(u8)]
		#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
		/// # Nice Characters.
		///
		/// This enum holds the (relatively) small subset of ASCII characters
		/// any of the `Nice*` structs would ever need to use.
		///
		/// This adds some complication to the data population side of things,
		/// but reduces the "unsafe" footprint to just two methods, both
		/// located here.
		///
		/// TODO: replace this with `AsciiChar` once stable.
		pub(super) enum NiceChar {
			$($k = $v,)+
		}

		impl NiceChar {
			#[inline(always)]
			#[must_use]
			/// # From Digit.
			///
			/// Convert a single digit (`0..=9`) to the corresponding `NiceChar`.
			///
			/// Because the callers are already neck-deep in calculations and
			/// checks, this method saturates the result rather than verifying
			/// the source is actually a single digit.
			pub(super) const fn from_digit(src: u8) -> Self {
				match src {
					0 => Self::Digit0,
					1 => Self::Digit1,
					2 => Self::Digit2,
					3 => Self::Digit3,
					4 => Self::Digit4,
					5 => Self::Digit5,
					6 => Self::Digit6,
					7 => Self::Digit7,
					8 => Self::Digit8,
					_ => Self::Digit9,
				}
			}

			#[expect(unsafe_code, reason = "For transmute.")]
			#[inline(always)]
			#[must_use]
			/// # As Bytes.
			///
			/// Transmute a slice of `NiceChar` into a slice of bytes.
			pub(super) const fn as_bytes(src: &[Self]) -> &[u8] {
				// This check is overly-paranoid, but the compiler should
				// optimize it out.
				const {
					assert!(
						align_of::<&[Self]>() == align_of::<&[u8]>() &&
						size_of::<&[Self]>() == size_of::<&[u8]>(),
						"BUG: NiceChar and u8 have different layouts?!",
					);
				}

				// Safety: `NiceChar` is represented by `u8` so shares the
				// same size and alignment.
				unsafe { std::mem::transmute::<&[Self], &[u8]>(src) }
			}

			#[expect(unsafe_code, reason = "For transmute.")]
			#[inline(always)]
			#[must_use]
			/// # As Str.
			///
			/// Transmute a slice of `NiceChar` into a string slice.
			pub(super) const fn as_str(src: &[Self]) -> &str {
				// Safety: all `NiceChar` variants are valid ASCII, so no
				// matter how they're sliced up, will always yield valid UTF-8
				// sequences.
				unsafe { std::str::from_utf8_unchecked(Self::as_bytes(src)) }
			}
		}
	);
}

nice_chars!(
	Space      b' ',  // NiceSeparator.
	Percent    b'%',  // NicePercent.
	Apostrophe b'\'', // NiceSeparator.
	Comma      b',',  // NiceSeparator.
	Dash       b'-',  // NiceSeparator.
	Period     b'.',  // NiceSeparator.
	Digit0     b'0',
	Digit1     b'1',
	Digit2     b'2',
	Digit3     b'3',
	Digit4     b'4',
	Digit5     b'5',
	Digit6     b'6',
	Digit7     b'7',
	Digit8     b'8',
	Digit9     b'9',
	Colon      b':', // NiceClock.
	Lt         b'<', // NiceFloat (overflow).
	Gt         b'>', // NiceFloat (overflow).
	LowerA     b'a', // NiceElapsed.
	LowerC     b'c', // NiceElapsed.
	LowerD     b'd', // NiceElapsed.
	LowerE     b'e', // NiceElapsed.
	LowerH     b'h', // NiceElapsed.
	LowerI     b'i', // NiceElapsed.
	LowerM     b'm', // NiceElapsed.
	LowerN     b'n', // NiceElapsed.
	LowerO     b'o', // NiceElapsed.
	LowerR     b'r', // NiceElapsed.
	LowerS     b's', // NiceElapsed.
	LowerT     b't', // NiceElapsed.
	LowerU     b'u', // NiceElapsed.
	LowerY     b'y', // NiceElapsed.
	Underscore b'_',  // NiceSeparator.
);
