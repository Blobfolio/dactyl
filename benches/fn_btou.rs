/*!
# Benchmark: `dactyl::btou::parse_u64`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::BytesToUnsigned;
use std::time::Duration;



benches!(
	Bench::new("u8", "btou(128)")
		.timed(Duration::from_secs(1))
		.with(|| u8::btou(b"128")),

	Bench::new("std::str", "parse::<u8>(65536)")
		.timed(Duration::from_secs(1))
		.with(|| "65536".parse::<u8>()),

	Bench::new("u16", "btou(65536)")
		.timed(Duration::from_secs(1))
		.with(|| u16::btou(b"65536")),

	Bench::new("std::str", "parse::<u16>(65536)")
		.timed(Duration::from_secs(1))
		.with(|| "65536".parse::<u16>()),

	Bench::new("u32", "btou(1844674407)")
		.timed(Duration::from_secs(1))
		.with(|| u32::btou(b"1844674407")),

	Bench::new("std::str", "parse::<u32>(1844674407)")
		.timed(Duration::from_secs(1))
		.with(|| "1844674407".parse::<u32>()),

	Bench::new("u64", "btou(18446744073709551615)")
		.timed(Duration::from_secs(1))
		.with(|| u64::btou(b"18446744073709551615")),

	Bench::new("std::str", "parse::<u64>(18446744073709551615)")
		.timed(Duration::from_secs(1))
		.with(|| "18446744073709551615".parse::<u64>()),

	Bench::new("u128", "btou(340282366920938463463374607431768211455)")
		.timed(Duration::from_secs(1))
		.with(|| u128::btou(b"340282366920938463463374607431768211455")),

	Bench::new("std::str", "parse::<u128>(340282366920938463463374607431768211455)")
		.timed(Duration::from_secs(1))
		.with(|| "340282366920938463463374607431768211455".parse::<u128>()),
);
