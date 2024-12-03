/*!
# Benchmark: `dactyl::nice_percent`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NicePercent;

benches!(
	Bench::new("dactyl::NicePercent::from(0)")
		.run(|| NicePercent::from(0_f32)),

	Bench::new("dactyl::NicePercent::from(0.1)")
		.run(|| NicePercent::from(0.1_f32)),

	Bench::new("dactyl::NicePercent::from(0.12)")
		.run(|| NicePercent::from(0.12_f32)),

	Bench::new("dactyl::NicePercent::from(0.123)")
		.run(|| NicePercent::from(0.123_f32)),

	Bench::new("dactyl::NicePercent::from(0.1234)")
		.run(|| NicePercent::from(0.1234_f32)),

	Bench::new("dactyl::NicePercent::from(0.12345)")
		.run(|| NicePercent::from(0.12345_f32)),

	Bench::spacer(),

	Bench::new("nice_percent_no_replace(all)").run(|| {
		let mut len = 0;
		for i in 0..=10_000_u16 {
			let i = f32::from(i) / 10_000.0;
			let last = NicePercent::from(i);
			len += last.len();
		}
		len
	}),
	Bench::new("nice_percent_replace(all)").run(|| {
		let mut len = 0;
		let mut last = NicePercent::from(0.0_f32);
		len += last.len();
		for i in 1..=10_000_u16 {
			let i = f32::from(i) / 10_000.0;
			last.replace(i);
			len += last.len();
		}
		len
	}),
);
