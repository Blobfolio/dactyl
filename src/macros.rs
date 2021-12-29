/*!
# Dactyl - Macros
*/

/// # Helper: `AsRef` and `Borrow`.
macro_rules! as_ref_borrow_cast {
	($parent:ty: $($cast:ident $ty:ty),+ $(,)?) => ($(
		impl AsRef<$ty> for $parent {
			#[inline]
			fn as_ref(&self) -> &$ty { self.$cast() }
		}

		impl ::std::borrow::Borrow<$ty> for $parent {
			#[inline]
			fn borrow(&self) -> &$ty { self.$cast() }
		}
	)+);
}

/// # Helper: `Display`.
macro_rules! display_str {
	($cast:ident $ty:ty) => (
		impl ::std::fmt::Display for $ty {
			#[inline]
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.write_str(self.$cast())
			}
		}
	);
}

/// # Helper: Recursive `From`.
macro_rules! from_cast {
	($parent:ty: $($cast:ident $ty:ty),+ $(,)?) => ($(
		impl From<$ty> for $parent {
			#[inline]
			fn from(src: $ty) -> Self { Self::from(src.$cast()) }
		}
	)+);
}


pub(super) use {
	as_ref_borrow_cast,
	display_str,
	from_cast,
};
