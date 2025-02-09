// This example exists to ensure that code generated by nutype macro
// can compile in no_std environment.
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use nutype::nutype;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// Integer
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 6),
    sanitize(with = |x| x),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        FromStr,
        AsRef,
        Deref,
        TryFrom,
        Into,
        Hash,
        Borrow,
        Display,
        Default,
    ),
    default = 4
)]
struct GermanTaxClass(i64);

// Float
#[nutype(
    validate(greater_or_equal = 0.0, less_or_equal = 1024.0, finite),
    sanitize(with = |x| x),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        FromStr,
        AsRef,
        Deref,
        TryFrom,
        Into,
        Borrow,
        Display,
        Default,
    ),
    default = 0.0
)]
struct Width(f64);

// NOTE: strings are not working yet with no_std

// Any other type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Into, From, Deref, Borrow, Hash
))]
pub struct Location(Point);

#[nutype(
    validate(less_or_equal = 100),
    derive(Serialize, Deserialize)
)]
pub struct Percentage(u8);
