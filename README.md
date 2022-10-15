![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/bit-bounds.svg)](https://crates.io/crates/bit-bounds)
[![Documentation](https://docs.rs/bit-bounds/badge.svg)](https://docs.rs/bit-bounds)

Helper traits for const generic bitwise bounds

```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bit_bounds::{IsPowerOf2, usize::*};

struct Buffer<const N: usize> {
  inner: [usize; N],
}

impl<const N: usize> Buffer<N>
where
  Int<N>: IsPowerOf2
{
  pub const fn new() -> Self {
    Buffer { inner: [0; N] }
  }
}

fn extract_index<const N: usize>(counter: usize) -> usize
where
  Int<N>: IsPowerOf2,
  Int<N>: BitsAllClear<{ (u32::MAX as usize) << 32 }>,
{
  (counter >> 32) & (N - 1)
}
```