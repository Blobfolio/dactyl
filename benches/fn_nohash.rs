/*!
# Benchmark: `dactyl::NoHash`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::NoHash;
use std::collections::HashSet;



/// # Standard Hash.
fn t_hash() -> HashSet<u16> { (u16::MIN..=u16::MAX).collect() }

/// # No Hash.
fn t_nohash() -> HashSet<u16, NoHash> { (u16::MIN..=u16::MAX).collect() }



benches!(
	Bench::new("(u16::MIN..=u16::MAX).collect::<HashSet<u16>>()")
		.run(t_hash),

	Bench::new("(u16::MIN..=u16::MAX).collect::<HashSet<u16, NoHash>>()")
		.run(t_nohash),
);
