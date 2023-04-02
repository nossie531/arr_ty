/*! Provider of [`arr_ty`].

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This provides a macro that allows smart initialization of arrays. This is
especially valuable when the type of the array elements is a trait object.

More specifically, `arr_ty` allows the notation of first specifying the
element type and second specifying the value of each element.

# Examples

```
# use std::any::Any;
# use arr_ty::arr_ty;
let arr = arr_ty!(Box<dyn Any>; [
    Box::new(0),
    Box::new(false),
    Box::new("false")
]);

assert_eq!(arr.len(), 3);
assert_eq!(*arr[0].downcast_ref::<i32>().unwrap(), 0);
assert_eq!(*arr[1].downcast_ref::<bool>().unwrap(), false);
assert_eq!(*arr[2].downcast_ref::<&str>().unwrap(), "false");
```

# Target problem

In Rust (1.68.2) as of 2023, as far as the author knows, any way to initialize
an array whose element type is a trait object is somewhat unsmart.

* Redundant...
``` no_run
# use std::any::Any;
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false) as Box<dyn Any>,
    Box::new("false") as Box<dyn Any>
];
```
* Or no unity...
``` no_run
# use std::any::Any;
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false),
    Box::new("false")
];
```
* Or manual counting...
``` no_run
# use std::any::Any;
let arr: [Box<dyn Any>; 3] = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```

In addition, the following codes will result in a compilation error.

* Compiler cannot guess the common type.
``` compile_fail
# use std::any::Any;
let arr = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```
* Compiler cannot guess the number of elements (Pattern 1).
``` compile_fail
# use std::any::Any;
let arr: [Box<dyn Any>; _] = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```
* Compiler cannot guess the number of elements (Pattern 2).
``` compile_fail
# use std::any::Any;
let arr = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
] as [Box<dyn Any>; _];
```
*/

#![no_std]

/// Provides a smart notation for array initialization.
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
pub fn use_arr<T, const N: usize>(_: [T; 0], src: [T; N]) -> [T; N] {
    src
}
