/*!
# Benchmark: `dactyl::traits::BytesToUnsigned`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::BytesToUnsigned;



benches!(
	Bench::new("u8::btou(255)")
		.run(|| u8::btou(b"255")),

	Bench::new("std::str::parse::<u8>(255)")
		.run(|| "255".parse::<u8>()),

	Bench::spacer(),

	Bench::new("u16::btou(65536)")
		.run(|| u16::btou(b"65536")),

	Bench::new("std::str::parse::<u16>(65536)")
		.run(|| "65536".parse::<u16>()),

	Bench::spacer(),

	Bench::new("u32::btou(1844674407)")
		.run(|| u32::btou(b"1844674407")),

	Bench::new("std::str::parse::<u32>(1844674407)")
		.run(|| "1844674407".parse::<u32>()),

	Bench::spacer(),

	Bench::new("u64::btou(18446744073709551615)")
		.run(|| u64::btou(b"18446744073709551615")),

	Bench::new("std::str::parse::<u64>(18446744073709551615)")
		.run(|| "18446744073709551615".parse::<u64>()),

	Bench::spacer(),

	Bench::new("u128::btou(340282366920938463463374607431768211455)")
		.run(|| u128::btou(b"340282366920938463463374607431768211455")),

	Bench::new("std::str::parse::<u128>(340282366920938463463374607431768211455)")
		.run(|| "340282366920938463463374607431768211455".parse::<u128>()),
);
