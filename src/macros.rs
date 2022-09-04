/*!
# Dactyl: Macros
*/

#[macro_export(local_inner_macros)]
/// # "Total" Float Comparison.
///
/// This macro allows you to compare two floats via [`f32::total_cmp`]/[`f64::total_cmp`]
/// using [`PartialEq`](std::cmp::PartialEq)/[`PartialOrd`](std::cmp::PartialOrd)-like syntax.
///
/// The floats can be idents, literals, and/or parenthesized expressions.
/// Mixing and matching is allowed, so long as both sides resolve to the
/// same type (`f32` or `f64`).
///
/// The following comparison operators are supported:
/// * `<`
/// * `<=`
/// * `==`
/// * `>=`
/// * `>`
/// * `!=`
///
/// ## Examples
///
/// ```
/// use dactyl::total_cmp;
///
/// let a: f64 = 1.0;
/// let b: f64 = 2.0;
/// let c: f64 = 3.0;
///
/// // Less than.
/// assert!(total_cmp!(a < b));
/// assert!(total_cmp!(-3.0_f64 < a)); // Literals are fine too.
/// assert!(! total_cmp!(c < a));      // Nope!
///
/// // Less than or equal to.
/// assert!(total_cmp!(a <= a));
/// assert!(total_cmp!(b <= c));
/// assert!(! total_cmp!(c <= b)); // Nope!
///
/// // Equal.
/// assert!(total_cmp!(a == a));
/// assert!(total_cmp!(b == b));
///
/// // Not equal.
/// assert!(total_cmp!(a != c));
/// assert!(! total_cmp!(a != a)); // Nope!
///
/// // Greater than or equal to.
/// assert!(total_cmp!(a >= a));
/// assert!(total_cmp!(c >= b));
/// assert!(! total_cmp!(b >= c)); // Nope!
///
/// // Greater than.
/// assert!(total_cmp!(b > a));
/// assert!(total_cmp!(c > b));
/// assert!(! total_cmp!(b > b));  // Nope!
/// ```
///
/// Expressions — or anything Rust doesn't consider to be an ident or
/// literal — work the same way, but need to be wrapped in parentheses
/// first:
///
/// ```
/// use dactyl::total_cmp;
///
/// assert!(total_cmp!((f64::NEG_INFINITY) < 0_f64));
/// assert!(total_cmp!((1_f64 + 2_f64) == 3_f64));
/// assert!(total_cmp!(1_f64 > (2_f64.mul_add(2.0, -3.5))));
/// ```
///
/// Total comparison doesn't _always_ agree with partial comparison. Refer
/// to [`f64::total_cmp`] for more details, but here's one obvious
/// difference:
///
/// ```
/// use dactyl::total_cmp;
///
/// assert!(total_cmp!(-0_f32 < 0_f32)); // Total cmp honors the negative.
/// assert_eq!(-0_f32, 0_f32);           // Partial cmp ignores it.
/// ```
macro_rules! total_cmp {
	($a:ident   $op:tt $b:ident) =>   ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	($a:ident   $op:tt $b:literal) => ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	($a:ident   $op:tt ($b:expr)) =>  ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	($a:literal $op:tt $b:ident) =>   ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	($a:literal $op:tt $b:literal) => ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	($a:literal $op:tt ($b:expr)) =>  ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	(($a:expr)  $op:tt $b:ident) =>   ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	(($a:expr)  $op:tt $b:literal) => ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
	(($a:expr)  $op:tt ($b:expr)) =>  ( $crate::_total_cmp!($op $a.total_cmp(&$b)) );
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
/// # Actual Comparison.
macro_rules! _total_cmp {
	(<  $expr:expr) => ( ::std::matches!($expr, ::std::cmp::Ordering::Less) );
	(<= $expr:expr) => (
		::std::matches!($expr, ::std::cmp::Ordering::Less | ::std::cmp::Ordering::Equal)
	);
	(== $expr:expr) => ( ::std::matches!($expr, ::std::cmp::Ordering::Equal) );
	(!= $expr:expr) => (
		::std::matches!($expr, ::std::cmp::Ordering::Less | ::std::cmp::Ordering::Greater)
	);
	(>= $expr:expr) => (
		::std::matches!($expr, ::std::cmp::Ordering::Equal | ::std::cmp::Ordering::Greater)
	);
	(>  $expr:expr) => ( ::std::matches!($expr, ::std::cmp::Ordering::Greater) );
}
