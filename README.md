arr_ty
===

Macros for smart array initialization (best for trait object element types).

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?
This crate makes it a little bit smarter about initializing arrays whose
element type is a trait object (just the author's personal feeling...).

```rust
let arr = arr_ty!(Box<dyn Any>; [
    Box::new(0),
    Box::new(false),
    Box::new("false")
]);
```

Without this crate, as far as the author knows,
Rust (1.68.2) as of 2023 has the following unsmart code.

* Redundant...
```rust
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false) as Box<dyn Any>,
    Box::new("false") as Box<dyn Any>
];
```
* Or no unity...
```rust
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false),
    Box::new("false")
];
```
* Or manual counting...
```rust
let arr: [Box<dyn Any>; 3] = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```

## What's New

Version 0.2 removes the restriction on element types. For example, a tuple
type consisting of a trait object can be used as an element type.

```rust
let arr = arr_ty!((i32, Box<dyn Any>); [
    (0, Box::new(false)),
    (0, Box::new("false"))
]);
