/*!
# Benchmark: `dactyl::nice_u32`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::{
	NiceSeparator,
	NiceU32,
};

benches!(
	Bench::new("dactyl::NiceU32::from(0)")
		.run(|| NiceU32::from(0_u32)),

	Bench::new("dactyl::NiceU32::from(100_020)")
		.run(|| NiceU32::from(100_020_u32)),

	Bench::new("dactyl::NiceU32::from(6_330_004)")
		.run(|| NiceU32::from(6_330_004_u32)),

	Bench::new("dactyl::NiceU32::from(57_444_000)")
		.run(|| NiceU32::from(57_444_000_u32)),

	Bench::new("dactyl::NiceU32::from(777_804_132)")
		.run(|| NiceU32::from(777_804_132_u32)),

	Bench::new("dactyl::NiceU32::from(u32::MAX)")
		.run(|| NiceU32::from(u32::MAX)),

	Bench::spacer(),

	Bench::new("dactyl::NiceU32::with_separator(777_804_132, b'_')")
		.run(|| NiceU32::with_separator(777_804_132_u32, NiceSeparator::Underscore)),
);
