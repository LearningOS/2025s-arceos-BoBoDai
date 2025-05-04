#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println_with_color;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println_with_color!(32,"[WithColor]: Hello, Arceos!");
}
