# Stack trait

[![Crates.io](https://img.shields.io/crates/v/stack-trait)](https://crates.io/crates/stack-trait)
[![Downloads](https://img.shields.io/crates/d/stack-trait.svg)](https://crates.io/crates/stack-trait)
[![Documentation](https://docs.rs/stack-trait/badge.svg)](https://docs.rs/stack-trait)
[![License](https://img.shields.io/crates/l/stack-trait)](https://crates.io/crates/stack-trait)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/stack-trait/status.svg)](https://deps.rs/repo/github/JohnScience/stack-trait)

Stack trait with entry API for the LIFO element.

## Example

```rust
use stack_trait::{Stack, LIFOEntry};

// types are written explicitly for clarity
fn main() {
    let mut stack: Vec<i32> = vec![1, 2, 3];
    // When we push to the stack, we can get an "entry" object
    // that corresponds to the pushed element.
    let mut entry: LIFOEntry<'_, Vec<i32>> = stack.lifo_push(4);
    // This object can be dereferenced to get the shared
    // reference to the element.
    assert_eq!(*entry, 4);
    // We also can dereference LIFO entry mutably to get
    // the mutable reference to the element and change the
    // element.
    *entry = 5;
    // After this we can dereference it immutably again.
    assert_eq!(*entry, 5);
    // However, we must drop the entry before we can use
    // the stack again.
    drop(entry);
    assert_eq!(stack, vec![1, 2, 3, 5]);
    // In this case, we could be bold to use
    // `.lifo_unchecked()` unsafe method but the
    // boundary check at this unwrap should be
    // optimized out by the compiler.
    let entry = stack.lifo().unwrap();
    // In addition, we can pop the element from the stack
    assert_eq!(entry.pop(), 5);
    assert_eq!(stack, vec![1, 2, 3]);
}
```

As demonstrated above, `LIFOEntry<'a,C>` can be coverted to `&'a C`, `&'a mut C` or even `C` at our discretion.

## Notes

At the point of writing, this trait is implemented only for `Vec<T>`. However, having this trait implemented for other types, such as `ArrayVec<T>` is welcome.
