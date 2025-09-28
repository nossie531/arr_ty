//! Macros for smart array initialization.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

#![no_std]
#![warn(missing_docs)]

pub mod prelude;

/// Provides a smart notation for array initialization.
///
/// # Examples
///
/// Normal example.
///
/// ```
/// # use arr_ty::prelude::*;
/// let arr = arr_ty!(u32; [0, 1, 2]);
/// assert_eq!(arr, [0u32, 1, 2]);
/// ```
///
/// With initial value example.
///
/// ```
/// # use arr_ty::prelude::*;
/// let arr = arr_ty!(u32; 42; 3);
/// assert_eq!(arr, [42u32, 42, 42]);
/// ```
#[macro_export]
macro_rules! arr_ty {
    ($type:ty; $(,)?) => {
        [] as [$type;0]
    };
    ($type:ty; [$($v:expr),* $(,)?]) => {
        $crate::use_arr([] as [$type;0], [$($v),*])
    };
    ($type:ty; $v:expr; $n:literal) => {
        [$v as $type; $n] as [$type; $n]
    };
}

/// Just returns the passed array with specified element type.
#[doc(hidden)]
#[inline(always)]
pub fn use_arr<T, const N: usize>(_: [T; 0], src: [T; N]) -> [T; N] {
    src
}
