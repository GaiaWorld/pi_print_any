#![feature(specialization)]
//! output without the trait bounds (using specialization to
//! find the right impl anyway)
//! 
//! output value for type of impl Debug, output type name for unimplDebug.
//! 
//! # for example:
//! ```
//! #[derive(Debug)]
//! struct A(usize);
//! struct B(usize);
//! fn main() {
//! 	println!("{:?}", A(1)); // output: A(1)
//! 	println!("{:?}", B(1)); // output: `pi_print_any::B`
//! }
//! ```
//!
//! **NOTE**: This uses experimental Rust features and is therefore
//! by itself experimental and unstable, and has all the problems of
//! `feature(specialization)`.
//! 
use std::fmt;
use std::any::type_name;

/// output a message by `println!`, and then each value's debug representation (if it has one)
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate pi_print_any;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         println_any!("println_any: {:?}", x);
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! println_any {
    ($message:expr, $($value:expr),*) => {{
        out_any!(println, $message, $($value),*)
    }};
}

/// output a message by `print!`, and then each value's debug representation (if it has one)
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate pi_print_any;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         print_any!("print_any: {:?}", x);
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! print_any {
    ($message:expr, $($value:expr),*) => {{
        out_any!(print, $message, $($value),*)
    }};
}

/// output a message by a specified way, and then each value's debug representation (if it has one)
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate pi_print_any;
///
/// fn main() { 
/// 	out_any!(print,"out_any: {:?}", 1); // output by print
/// 	out_any!(println,"out_any: {:?}", 1); // output by println
/// 	out_any!(log::info,"out_any: {:?}", 1); // output by log::info
/// }
/// ```
#[macro_export]
macro_rules! out_any {
    ($call:path, $message:expr, $($value:expr),*) => {{
        $call!($message, $($crate::new_debugit($value)),*)
    }};
}

/// This type always implements `Debug`. Uses specialization to use
/// the inner value's Debug (which it should basically always have).
///
/// Otherwise, falls back to print the type name.
///
/// # Examples
///
/// ```
/// use debugit::DebugIt as D;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         println!("starting with {:?}", D(&x));
///     }
/// }
/// ```
#[derive(Copy, Clone)]
pub struct DebugIt<T> {
    value: T
}

/// This type always implements `Debug`. Uses specialization to use
/// the inner value's Debug (which it should basically always have).
///
/// Otherwise, falls back to print the type name.
///
/// # Examples
///
/// ```
/// use debugit::DebugIt as D;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         println!("starting with {:?}", D(&x));
///     }
/// }
/// ```
pub fn new_debugit<T>(value: T) -> DebugIt<T> {
    DebugIt { value }
}

impl<T> fmt::Debug for DebugIt<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", type_name::<T>())
    }
}

impl<T> fmt::Debug for DebugIt<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::new_debugit;

    #[test]
    fn it_works() {
        fn debug_no_bound<T>(x: T, s: &str) {
            assert_eq!(&format!("{:?}", new_debugit(&x)), s);
        }

        debug_no_bound(1, "1");
        debug_no_bound((), "()");

		// struct A (usize);
		// out_any!(println, "{:?}, {:?}", A(1), 1);
		// println_any!("{:?}, {:?}", A(1), 1);
		// print_any!("{:?}, {:?}", A(1), 1);
    }
}
