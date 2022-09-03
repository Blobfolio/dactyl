/*!
# Benchmark: `dactyl::nice_u8`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceU8;

benches!(
	Bench::new("dactyl::NiceU8::from(0)")
		.run(|| NiceU8::from(0_u8)),

	Bench::new("dactyl::NiceU8::from(18)")
		.run(|| NiceU8::from(18_u8)),

	Bench::new("dactyl::NiceU8::from(101)")
		.run(|| NiceU8::from(101_u8)),

	Bench::new("dactyl::NiceU8::from(u8::MAX)")
		.run(|| NiceU8::from(u8::MAX))
);
