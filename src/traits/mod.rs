/*!
# Dactyl: Traits
*/

mod btoi; // TODO: remove when from_ascii is stable.
mod btou; // TODO: remove when from_ascii is stable.
mod hex;  // TODO: remove when from_ascii_radix is stable.
mod inflect;
mod saturating_from;

pub use btoi::BytesToSigned;
pub use btou::BytesToUnsigned;
pub use hex::{
	HexToSigned,
	HexToUnsigned,
};
pub use inflect::{
	Inflection,
	NiceInflection,
};
pub use saturating_from::SaturatingFrom;
