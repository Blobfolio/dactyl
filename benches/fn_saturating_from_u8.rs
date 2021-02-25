/*!
# Benchmark: `dactyl::traits::SaturatingFrom`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::SaturatingFrom;
use std::time::Duration;

benches!(
	Bench::new("u8::saturating_from<u32>", "(99)")
		.timed(Duration::from_secs(1))
		.with(|| u8::saturating_from(99_u32)),

	Bench::new("u8::saturating_from<u32>", "(16_789)")
		.timed(Duration::from_secs(1))
		.with(|| u8::saturating_from(16_789_u32)),

	Bench::new("u8::saturating_from<u32>", "(u32::MAX)")
		.timed(Duration::from_secs(1))
		.with(|| u8::saturating_from(u32::MAX))
);
