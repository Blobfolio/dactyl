/*!
# Benchmark: `dactyl::nice_u8`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceU8;
use std::time::Duration;

benches!(
	Bench::new("dactyl::NiceU8", "from(0)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU8::from(0_u8)),

	Bench::new("dactyl::NiceU8", "from(18)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU8::from(18_u8)),

	Bench::new("dactyl::NiceU8", "from(101)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU8::from(101_u8)),

	Bench::new("dactyl::NiceU8", "from(u8::MAX)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU8::from(u8::MAX))
);
