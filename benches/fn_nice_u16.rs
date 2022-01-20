/*!
# Benchmark: `dactyl::nice_u16`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NiceU16;
use std::time::Duration;

benches!(
	Bench::new("dactyl::NiceU16", "from(0)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(0_u16)),

	Bench::new("dactyl::NiceU16", "from(18)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(18_u16)),

	Bench::new("dactyl::NiceU16", "from(101)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(101_u16)),

	Bench::new("dactyl::NiceU16", "from(1_620)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(1_620_u16)),

	Bench::new("dactyl::NiceU16", "from(40_999)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(40_999_u16)),

	Bench::new("dactyl::NiceU16", "from(u16::MAX)")
		.timed(Duration::from_secs(1))
		.with(|| NiceU16::from(u16::MAX)),

	Bench::new("dactyl::NiceU16", "with_separator(b'_')")
		.timed(Duration::from_secs(1))
		.with_setup(NiceU16::from(40_999_u16), |c| c.with_separator(b'_')),

	Bench::new("dactyl::NiceU16", "as_string()")
		.timed(Duration::from_secs(1))
		.with_setup(NiceU16::from(40_999_u16), |c| c.as_string()),

	Bench::new("dactyl::NiceU16", "to_string()")
		.timed(Duration::from_secs(1))
		.with_setup(NiceU16::from(40_999_u16), |c| c.to_string()),
);
