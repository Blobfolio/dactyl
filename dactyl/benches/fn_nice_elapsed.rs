/*!
# Benchmark: `dactyl::nice_elapsed`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceElapsed;
use std::time::Duration;

benches!(
	Bench::new("dactyl::NiceElapsed", "hms(10)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::hms(10_u32)),

	Bench::new("dactyl::NiceElapsed", "hms(113)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::hms(113_u32)),

	Bench::new("dactyl::NiceElapsed", "hms(10502)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::hms(10502_u32)),

	Bench::new("dactyl::NiceElapsed", "from(1)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(1_u32)),

	Bench::new("dactyl::NiceElapsed", "from(50)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(50_u32)),

	Bench::new("dactyl::NiceElapsed", "from(100)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(100_u32)),

	Bench::new("dactyl::NiceElapsed", "from(2121)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(2121_u32)),

	Bench::new("dactyl::NiceElapsed", "from(37732)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(37732_u32)),

	Bench::new("dactyl::NiceElapsed", "from(428390)")
		.timed(Duration::from_secs(1))
		.with(|| NiceElapsed::from(428390_u32))
);
