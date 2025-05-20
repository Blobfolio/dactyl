/*!
# Benchmark: `dactyl::nice_float`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceFloat;

benches!(
	Bench::new("dactyl::NiceFloat::from(0.0)")
		.run(|| NiceFloat::from(0_f64)),

	Bench::new("dactyl::NiceFloat::from(NaN)")
		.run(|| NiceFloat::from(f64::NAN)),

	Bench::new("dactyl::NiceFloat::from(12345.6789)")
		.run(|| NiceFloat::from(12_345.678_9_f64)),

	Bench::spacer(),

	Bench::new("dactyl::NiceFloat::from(12345.6789).as_bytes()")
		.run_seeded(NiceFloat::from(12_345.678_9_f64), |n| n.as_bytes().len()),

	Bench::new("dactyl::NiceFloat::from(12345.6789).compact_bytes()")
		.run_seeded(NiceFloat::from(12_345.678_9_f64), |n| n.compact_bytes().len()),

	Bench::new("dactyl::NiceFloat::from(12345.6789).precise_bytes(3)")
		.run_seeded(NiceFloat::from(12_345.678_9_f64), |n| n.precise_bytes(3).len()),
);
