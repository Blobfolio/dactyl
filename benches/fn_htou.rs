/*!
# Benchmark: `dactyl::traits::HexToUnsigned`
*/

use brunch::{
	Bench,
	benches,
};
use dactyl::traits::HexToUnsigned;



benches!(
	Bench::new("u8::htou(b)")
		.run(|| u8::htou(b"b")),

	Bench::new("u8::from_str_radix(b, 16)")
		.run(|| u8::from_str_radix("b", 16)),

	Bench::new("u8::htou(ff)")
		.run(|| u8::htou(b"ff")),

	Bench::new("u8::from_str_radix(ff, 16)")
		.run(|| u8::from_str_radix("ff", 16)),

	Bench::spacer(),

	Bench::new("u16::htou(303)")
		.run(|| u16::htou(b"303")),

	Bench::new("u16::from_str_radix(303, 16)")
		.run(|| u16::from_str_radix("303", 16)),

	Bench::new("u16::htou(ffff)")
		.run(|| u16::htou(b"ffff")),

	Bench::new("u16::from_str_radix(ffff, 16)")
		.run(|| u16::from_str_radix("ffff", 16)),

	Bench::spacer(),

	Bench::new("u32::htou(ab0321)")
		.run(|| u32::htou(b"ab0321")),

	Bench::new("u32::from_str_radix(ab0321, 16)")
		.run(|| u32::from_str_radix("ab0321", 16)),

	Bench::new("u32::htou(ffffffff)")
		.run(|| u32::htou(b"ffffffff")),

	Bench::new("u32::from_str_radix(ffffffff, 16)")
		.run(|| u32::from_str_radix("ffffffff", 16)),

	Bench::spacer(),

	Bench::new("u64::htou(b3a73ce2ff2)")
		.run(|| u64::htou(b"b3a73ce2ff2")),

	Bench::new("u64::from_str_radix(b3a73ce2ff2, 16)")
		.run(|| u64::from_str_radix("b3a73ce2ff2", 16)),

	Bench::new("u64::htou(ffffffffffffffff)")
		.run(|| u64::htou(b"ffffffffffffffff")),

	Bench::new("u64::from_str_radix(ffffffffffffffff, 16)")
		.run(|| u64::from_str_radix("ffffffffffffffff", 16)),

	Bench::spacer(),

	Bench::new("u128::htou(b993c7dff2ff2183388)")
		.run(|| u128::htou(b"b993c7dff2ff2183388")),

	Bench::new("u128::from_str_radix(b993c7dff2ff2183388, 16)")
		.run(|| u128::from_str_radix("b993c7dff2ff2183388", 16)),

	Bench::new("u128::htou(ffffffffffffffffffffffffffffffff)")
		.run(|| u128::htou(b"ffffffffffffffffffffffffffffffff")),

	Bench::new("u128::from_str_radix(ffffffffffffffffffffffffffffffff, 16)")
		.run(|| u128::from_str_radix("ffffffffffffffffffffffffffffffff", 16)),
);
