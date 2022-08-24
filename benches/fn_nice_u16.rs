/*!
# Benchmark: `dactyl::nice_u16`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceU16;

benches!(
	Bench::new("dactyl::NiceU16::from(0)")
		.run(|| NiceU16::from(0_u16)),

	Bench::new("dactyl::NiceU16::from(18)")
		.run(|| NiceU16::from(18_u16)),

	Bench::new("dactyl::NiceU16::from(101)")
		.run(|| NiceU16::from(101_u16)),

	Bench::new("dactyl::NiceU16::from(1_620)")
		.run(|| NiceU16::from(1_620_u16)),

	Bench::new("dactyl::NiceU16::from(40_999)")
		.run(|| NiceU16::from(40_999_u16)),

	Bench::new("dactyl::NiceU16::from(u16::MAX)")
		.run(|| NiceU16::from(u16::MAX)),

	Bench::new("dactyl::NiceU16::with_separator(40_999, b'_')")
		.run(|| NiceU16::with_separator(40_999_u16, b'_')),

	Bench::spacer(),

	Bench::new("String::from::<dactyl::NiceU16>()")
		.run_seeded(NiceU16::from(40_999_u16), String::from),

	Bench::new("dactyl::NiceU16::to_string()")
		.run_seeded(NiceU16::from(40_999_u16), |c| c.to_string()),
);
