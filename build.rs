/*!
# Dactyl: Build Script.

This is used to pre-compile all of the integer-to-integer SaturatingFrom
implementations because they're an utter nightmare without some degree of
automation.

But don't worry, it's still a nightmare. Haha.
*/

use std::{
	env,
	fmt::{
		self,
		Write,
	},
	fs::File,
	io::Write as ByteWrite,
	path::PathBuf,
};



/// # Min/Max Trait.
///
/// This trait lets us identify primitives and their lower/upper bounds, nice
/// and easy.
trait NumberExt: Copy {
	const MIN_NUMBER: Self;
	const MAX_NUMBER: Self;
}

macro_rules! numext {
	($($ty:ty),+) => ($(
		impl NumberExt for $ty {
			const MIN_NUMBER: Self = Self::MIN;
			const MAX_NUMBER: Self = Self::MAX;
		}
	)+);
}

numext! { u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 }



#[derive(Clone, Copy)]
/// # A Number.
///
/// This enum levels the playing field between integer values of different
/// types by upcasting everything to 128-bit.
///
/// It also ensures that when printed, `_` separators are added to the right
/// place to keep the linter happy.
enum AnyNum {
	Unsigned(u128),
	Signed(i128),
}

macro_rules! into_any {
	($($from:ty),+) => ($(
		impl From<$from> for AnyNum {
			#[allow(unused_comparisons)] // Needed for signed types.
			fn from(src: $from) -> Self {
				if src < 0 { Self::Signed(src as i128) }
				else { Self::Unsigned(src as u128) }
			}
		}
	)+);
}

into_any! { u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 }

impl fmt::Display for AnyNum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Stringify the number, unless it's too small for punctuation, in
		// which case we can just return it directly.
		let mut out = match self {
			Self::Unsigned(n) =>
				if *n < 1000 { return <u128 as fmt::Display>::fmt(n, f); }
				else { n.to_string() },
			Self::Signed(n) =>
				if (-999..1000).contains(n) { return <i128 as fmt::Display>::fmt(n, f); }
				else { n.to_string() },
		};

		// Add _ delimiters every three places starting from the end.
		let last = if out.starts_with('-') { 4 } else { 3 };
		let mut idx = out.len();
		while idx > last {
			idx -= 3;
			out.insert(idx, '_');
		}

		// Done!
		f.write_str(&out)
	}
}

impl AnyNum {
	/// # Inner as Signed.
	///
	/// Return the inner value as an `i128` regardless of sign.
	const fn signed_inner(self) -> i128 {
		match self {
			Self::Unsigned(n) =>n as i128,
			Self::Signed(n) => n,
		}
	}

	/// # Inner as Unsigned.
	///
	/// Return the inner value as a `u128` regardless of sign.
	const fn unsigned_inner(self) -> u128 {
		match self {
			Self::Unsigned(n) => n,
			Self::Signed(n) => n as u128,
		}
	}
}



#[derive(Clone, Copy)]
/// # Two Numeric Types.
///
/// This struct is used to write the `saturating_from` body for any two given
/// types.
struct AnyTwo<TO, FROM>(TO, FROM)
where TO: NumberExt + Into<AnyNum>, FROM: NumberExt + Into<AnyNum>;

impl<TO, FROM> fmt::Display for AnyTwo<TO, FROM>
where TO: NumberExt + Into<AnyNum>, FROM: NumberExt + Into<AnyNum> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Minimum clamp, if any.
		let to: AnyNum = TO::MIN_NUMBER.into();
		let from: AnyNum = FROM::MIN_NUMBER.into();
		let min = (from.signed_inner() < to.signed_inner()).then_some(to);

		// Maximum clamp, if any.
		let to: AnyNum = TO::MAX_NUMBER.into();
		let from: AnyNum = FROM::MAX_NUMBER.into();
		let max = (to.unsigned_inner() < from.unsigned_inner()).then_some(to);

		// Write the conditions!
		match (min, max) {
			(Some(min), Some(max)) => writeln!(
				f,
				"\t\tif src <= {min} {{ {min} }}
		else if src >= {max} {{ {max} }}
		else {{ src as Self }}"
			),
			(Some(min), None) => writeln!(
				f,
				"\t\tif src <= {min} {{ {min} }}
		else {{ src as Self }}"
			),
			(None, Some(max)) => writeln!(
				f,
				"\t\tif src >= {max} {{ {max} }}
		else {{ src as Self }}"
			),
			(None, None) => f.write_str("\t\tsrc as Self\n"),
		}
	}
}



/// # Helper: Write Basic From/To Implementations.
macro_rules! wrt {
	// Conversion.
	($out:ident, $to:ty as $alias:ty, $($from:ty),+) => ($(
		writeln!(
			&mut $out,
			"impl SaturatingFrom<{from}> for {to} {{
	#[inline]
	/// # Saturating From `{from}`.
	///
	/// This method will saturate and recast a `{from}` into a `{to}`, clamping to `{to}::MIN` and `{to}::MAX` as necessary.
	fn saturating_from(src: {from}) -> Self {{
{body}\t}}
}}",
			from=stringify!($from),
			to=stringify!($to),
			body=AnyTwo::<$alias, $from>(0, 0),
		).unwrap();
	)+);
	// Shorthand (including passthrough).
	($out:ident, $to:ty, $($from:ty),+) => (
		// Passthrough Implementation.
		$out.push_str(concat!(
			"impl SaturatingFrom<Self> for ", stringify!($to), " {
	#[inline]
	/// # Saturating From `Self`.
	///
	/// This implementation is provided for consistency; the value is simply passed through.
	fn saturating_from(src: Self) -> Self { src }
}\n"));

		// Handle conversions.
		wrt!($out, $to as $to, $($from),+);
	);
}



/// # Main.
fn main() {
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

	// Make sure our formatting looks right.
	assert_eq!(AnyNum::from(12345_u32).to_string(),   "12_345", "Bug: Number formatting is wrong!");
	assert_eq!(AnyNum::from(-12345_i32).to_string(), "-12_345", "Bug: Number formatting is wrong!");

	// Compile and write the impls!
	let data = build_impls();
	File::create(out_path("dactyl-saturation.rs"))
		.and_then(|mut f| f.write_all(data.as_bytes()).and_then(|_| f.flush()))
		.expect("Unable to save drive data.");
}

/// # Build Impls.
///
/// Generate "code" corresponding to all of the integer-to-integer
/// SaturatingFrom implementations, and return it as a string.
///
/// This would be fairly compact were it not for Rust's sized types, which
/// require cfg-gated module wrappers.
///
/// TODO: if it ever becomes possible for a bulid script to share pointer
/// widths with the target (rather than always using the host), clean up the
/// sized crap. Haha.
fn build_impls() -> String {
	let mut out = String::with_capacity(32_768);
	let mut tmp = String::with_capacity(4096);

	// Into Unsigned.
	wrt!(out, u8,        u16, u32, u64, u128, i8, i16, i32, i64, i128);
	wrt!(out, u16,   u8,      u32, u64, u128, i8, i16, i32, i64, i128);
	wrt!(out, u32,   u8, u16,      u64, u128, i8, i16, i32, i64, i128);
	wrt!(out, u64,   u8, u16, u32,      u128, i8, i16, i32, i64, i128);
	wrt!(out, u128,  u8, u16, u32, u64,       i8, i16, i32, i64, i128);

	// Into Signed.
	wrt!(out, i8,    u8, u16, u32, u64, u128,     i16, i32, i64, i128);
	wrt!(out, i16,   u8, u16, u32, u64, u128, i8,      i32, i64, i128);
	wrt!(out, i32,   u8, u16, u32, u64, u128, i8, i16,      i64, i128);
	wrt!(out, i64,   u8, u16, u32, u64, u128, i8, i16, i32,      i128);
	wrt!(out, i128,  u8, u16, u32, u64, u128, i8, i16, i32, i64      );

	// Write cfg-gated modules containing all of the sized implementations for
	// a given pointer width. Thankfully we only have to enumerate the into
	// impls; generics can be used for the equivalent froms.
	macro_rules! sized {
		($unsigned:ty, $signed:ty) => (
			writeln!(
				&mut out,
				"
#[cfg(target_pointer_width = \"{}\")]
mod sized {{
	use super::SaturatingFrom;

	impl<T: SaturatingFrom<{unsigned}>> SaturatingFrom<usize> for T {{
		#[inline]
		/// # Saturating From `usize`
		///
		/// This blanket implementation uses `{unsigned}` as a go-between, since it is equivalent to `usize`.
		fn saturating_from(src: usize) -> T {{
			T::saturating_from(src as {unsigned})
		}}
	}}

	impl<T: SaturatingFrom<{signed}>> SaturatingFrom<isize> for T {{
		#[inline]
		/// # Saturating From `isize`
		///
		/// This blanket implementation uses `{signed}` as a go-between, since it is equivalent to `isize`.
		fn saturating_from(src: isize) -> T {{
			T::saturating_from(src as {signed})
		}}
	}}",
				<$unsigned>::BITS,
				unsigned=stringify!($unsigned),
				signed=stringify!($signed),
			).unwrap();

			// Write all of the into implementations for our sized types into
			// a separate buffer, then iterate over that so we can tweak the
			// indentation.
			tmp.truncate(0);
			wrt!(tmp, usize as $unsigned, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
			wrt!(tmp, isize as $signed,   u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
			for line in tmp.lines() {
				out.push('\t');
				out.push_str(line);
				out.push('\n');
			}

			// Close off the module.
			out.push_str("}\n");
		);
	}

	// Actually write the sized modules.
	sized!(u16, i16);
	sized!(u32, i32);
	sized!(u64, i64);

	// Done!
	out
}

/// # Out path.
///
/// This generates a (file/dir) path relative to `OUT_DIR`.
fn out_path(name: &str) -> PathBuf {
	let dir = env::var("OUT_DIR").expect("Missing OUT_DIR.");
	let mut out = std::fs::canonicalize(dir).expect("Missing OUT_DIR.");
	out.push(name);
	out
}
