/*!
# Benchmark: `dactyl::traits::BytesToSigned`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::BytesToSigned;



benches!(
	Bench::new("i8::btoi(127)")
		.run(|| i8::btoi(b"127")),

	Bench::new("std::str::parse::<i8>(127)")
		.run(|| "127".parse::<i8>()),

	Bench::new("i8::btoi(-127)")
		.run(|| i8::btoi(b"-127")),

	Bench::new("std::str::parse::<i8>(-127)")
		.run(|| "-127".parse::<i8>()),

	Bench::spacer(),

	Bench::new("i16::btoi(32767)")
		.run(|| i16::btoi(b"32767")),

	Bench::new("std::str::parse::<i16>(32767)")
		.run(|| "32767".parse::<i16>()),

	Bench::spacer(),

	Bench::new("i32::btoi(2147483647)")
		.run(|| i32::btoi(b"2147483647")),

	Bench::new("std::str::parse::<i32>(2147483647)")
		.run(|| "2147483647".parse::<i32>()),

	Bench::spacer(),

	Bench::new("i64::btoi(9223372036854775807)")
		.run(|| i64::btoi(b"9223372036854775807")),

	Bench::new("std::str::parse::<i64>(9223372036854775807)")
		.run(|| "9223372036854775807".parse::<i64>()),

	Bench::spacer(),

	Bench::new("i128::btoi(170141183460469231731687303715884105727)")
		.run(|| i128::btoi(b"170141183460469231731687303715884105727")),

	Bench::new("std::str::parse::<i128>(170141183460469231731687303715884105727)")
		.run(|| "170141183460469231731687303715884105727".parse::<i128>()),
);
