/*!
# Benchmark: `dactyl::NoHash`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NoHash;
use std::{
	collections::HashSet,
	time::Duration,
};



/// # Standard Hash.
fn t_hash() -> HashSet<u16> { (u16::MIN..=u16::MAX).collect() }

/// # No Hash.
fn t_nohash() -> HashSet<u16, NoHash> { (u16::MIN..=u16::MAX).collect() }



benches!(
	Bench::new("(u16::MIN..=u16::MAX).collect", "<HashSet<u16>>()")
		.timed(Duration::from_secs(2))
		.with(|| t_hash()),

	Bench::new("(u16::MIN..=u16::MAX).collect", "<HashSet<u16, NoHash>>()")
		.timed(Duration::from_secs(2))
		.with(|| t_nohash()),
);
