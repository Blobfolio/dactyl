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
	Bench::new("dactyl::NiceElapsed::hms(10)")
		.run(|| NiceElapsed::hms(10_u32)),

	Bench::new("dactyl::NiceElapsed::hms(113)")
		.run(|| NiceElapsed::hms(113_u32)),

	Bench::new("dactyl::NiceElapsed::hms(10502)")
		.run(|| NiceElapsed::hms(10502_u32)),

	Bench::new("dactyl::NiceElapsed::dhms(10502)")
		.run(|| NiceElapsed::hms(10502_u32)),

	Bench::new("dactyl::NiceElapsed::dhms(269702)")
		.run(|| NiceElapsed::hms(269702_u32)),

	Bench::spacer(),

	Bench::new("dactyl::NiceElapsed::from(1)")
		.run(|| NiceElapsed::from(1_u32)),

	Bench::new("dactyl::NiceElapsed::from(50)")
		.run(|| NiceElapsed::from(50_u32)),

	Bench::new("dactyl::NiceElapsed::from(100)")
		.run(|| NiceElapsed::from(100_u32)),

	Bench::new("dactyl::NiceElapsed::from(2121)")
		.run(|| NiceElapsed::from(2121_u32)),

	Bench::new("dactyl::NiceElapsed::from(37732)")
		.run(|| NiceElapsed::from(37732_u32)),

	Bench::new("dactyl::NiceElapsed::from(Duration(37732030ms))")
		.run(|| NiceElapsed::from(Duration::from_millis(37732030))),

	Bench::new("dactyl::NiceElapsed::from(428390)")
		.run(|| NiceElapsed::from(428390_u32))
);
