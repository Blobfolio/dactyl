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
		.run(|| NicePercent::from(0.12345_f32))
);
