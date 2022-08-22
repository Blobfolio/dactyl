/*!
# Benchmark: `dactyl::traits::SaturatingFrom`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::SaturatingFrom;

benches!(
	Bench::new("u8::saturating_from::<u32>(99)")
		.run(|| u8::saturating_from(99_u32)),

	Bench::new("u8::saturating_from::<u32>(16_789)")
		.run(|| u8::saturating_from(16_789_u32)),

	Bench::new("u8::saturating_from::<u32>(u32::MAX)")
		.run(|| u8::saturating_from(u32::MAX))
);
