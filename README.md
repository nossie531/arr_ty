# arr_ty

Macros for smart array initialization.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate makes Rust array initialization a little smarter.

## ⚠ Attention!

This crate is useless in the latest version of Rust.
`generic_arg_infer` was introduced in Rust 1.89.0 (2025-08-07).
(See rust issue [#85077][ri85077] for more details).

[ri85077]: https://github.com/rust-lang/rust/issues/85077

## Examples

### Use case 1 (number element type)

This is not possible.

```rust, ignore
let arr = [0, 1, 2] as [u32;_];
```

With this crate.

```rust
let arr = arr_ty!(u32; [0, 1, 2]);
```

Without this crate, manual counting approach.

```rust
let arr = [0, 1, 2] as [u32;3];
```

Without this crate, redundant type approach.

```rust
let arr = [0u32, 1u32, 2u32];
```

Without this crate, no unity approach.

```rust
let arr = [0u32, 1, 2];
```

### Use case 2 (dynamic element type)

This is not possible.

```rust, ignore
let arr: [Box<dyn Any>; _] = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```

With this crate.

```rust
let arr = arr_ty!(Box<dyn Any>; [
    Box::new(0),
    Box::new(false),
    Box::new("false")
]);
```

Without this crate, manual counting approach.

```rust
let arr: [Box<dyn Any>; 3] = [
    Box::new(0),
    Box::new(false),
    Box::new("false")
];
```

Without this crate, redundant cast approach.

```rust
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false) as Box<dyn Any>,
    Box::new("false") as Box<dyn Any>
];
```

Without this crate, no unity approach.

```rust
let arr = [
    Box::new(0) as Box<dyn Any>,
    Box::new(false),
    Box::new("false")
];
```

## History

See [CHANGELOG](CHANGELOG.md).
