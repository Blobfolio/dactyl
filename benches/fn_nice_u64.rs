/*!
# Benchmark: `dactyl::nice_u64`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceU64;

benches!(
	Bench::new("dactyl::NiceU64::from(0)")
		.run(|| NiceU64::from(0_u64)),

	Bench::new("dactyl::NiceU64::from(6_489_320_013)")
		.run(|| NiceU64::from(6_489_320_013_u64)),

	Bench::new("dactyl::NiceU64::from(42_489_320_013)")
		.run(|| NiceU64::from(42_489_320_013_u64)),

	Bench::new("dactyl::NiceU64::from(1_999_999_999_999)")
		.run(|| NiceU64::from(1_999_999_999_999_u64)),

	Bench::new("dactyl::NiceU64::from(u64::MAX)")
		.run(|| NiceU64::from(u64::MAX)),

	Bench::new("dactyl::NiceU64::with_separator(1_999_999_999_999, b'_')")
		.run(|| NiceU64::with_separator(1_999_999_999_999, b'_')),
);
