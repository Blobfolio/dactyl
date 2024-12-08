/*!
# Benchmark: `dactyl::nice_elapsed`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::{
	NiceClock,
	NiceElapsed,
};
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
		.run(|| NiceElapsed::from(428390_u32)),

	Bench::new("dactyl::NiceElapsed::from(428390_u64)")
		.run(|| NiceElapsed::from(428390_u64)),

	Bench::spacer(),

	Bench::new("dactyl::NiceClock::from(12345_u32)")
		.run(|| NiceClock::from(12345_u32)),

	Bench::spacer(),

	Bench::new("nice_clock_range_from").run(|| {
		let mut len: u8 = 0;
		for i in 0..86400_u32 {
			let last = NiceClock::from(i);
			len = len.wrapping_add(last.seconds());
		}
		len
	}),

	Bench::new("nice_clock_range_replace").run(|| {
		let mut len: u8 = 0;
		let mut last = NiceClock::from(0_u32);
		len = len.wrapping_add(last.seconds());
		for i in 1..86400_u32 {
			last.replace(i);
			len = len.wrapping_add(last.seconds());
		}
		len
	}),
);
