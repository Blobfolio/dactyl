/*!
# Benchmark: `dactyl::NiceU*::replace`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::{
	NiceU32,
	NiceU64,
};

/// # 100k NiceU32: No Reuse
fn niceu32_no_replace() -> usize {
	let mut len = 0;
	for i in 0..=100_000_u32 {
		let last = NiceU32::from(i);
		len += last.len();
	}
	len
}

/// # 100k NiceU32: With Reuse
fn niceu32_replace() -> usize {
	let mut len = 0;
	let mut last = NiceU32::from(0_u32);
	len += last.len();
	for i in 1..=100_000_u32 {
		last.replace(i);
		len += last.len();
	}
	len
}

/// # 100k NiceU64: No Reuse
fn niceu64_no_replace() -> usize {
	let mut len = 0;
	for i in 0..=100_000_u64 {
		let last = NiceU64::from(i);
		len += last.len();
	}
	len
}

/// # 100k NiceU64: With Reuse
fn niceu64_replace() -> usize {
	let mut len = 0;
	let mut last = NiceU64::from(0_u64);
	len += last.len();
	for i in 1..=100_000_u64 {
		last.replace(i);
		len += last.len();
	}
	len
}

benches!(
	Bench::new("niceu32_no_replace(100k)").run(niceu32_no_replace),
	Bench::new("niceu32_replace(100k)").run(niceu32_replace),

	Bench::spacer(),

	Bench::new("niceu64_no_replace(100k)").run(niceu64_no_replace),
	Bench::new("niceu64_replace(100k)").run(niceu64_replace),
);
