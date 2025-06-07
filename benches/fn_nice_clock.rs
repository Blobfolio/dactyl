/*!
# Benchmark: `dactyl::nice_clock`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceClock;

benches!(
	Bench::new("dactyl::NiceClock::from(0_u32)")
		.run(|| NiceClock::from(0_u32)),

	Bench::new("dactyl::NiceClock::from(60_u32)")
		.run(|| NiceClock::from(60_u32)),

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
