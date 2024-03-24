//!
//! Helper traits for const generic bounds
//!
//! ```rust
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//!
//! use bit_bounds::{IsPowerOf2, usize::*};
//!
//! struct Buffer<const N: usize> {
//!   inner: [usize; N],
//! }
//!
//! impl<const N: usize> Buffer<N>
//! where
//!   Int<N>: IsPowerOf2
//! {
//!   pub const fn new() -> Self {
//!     Buffer { inner: [0; N] }
//!   }
//! }
//!
//! fn extract_index<const N: usize>(counter: usize) -> usize
//! where
//!   Int<N>: IsPowerOf2,
//!   Int<N>: BitsAllClear<{ (u32::MAX as usize) << 32 }>,
//! {
//!   (counter >> 32) & (N - 1)
//! }
//! ```

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![no_std]

use const_assert::{Assert, IsTrue};

pub trait IsPowerOf2 {}

macro_rules! define_bit_bounds {
  ($ty:ident) => {
    pub mod $ty {
      use super::*;
      pub trait BitsAllSet<const M: $ty> {}
      pub trait BitsAnySet<const M: $ty> {}
      pub trait BitsAllClear<const M: $ty> {}
      pub trait BitsAnyClear<const M: $ty> {}

      pub struct Int<const N: $ty>;
      impl<const N: $ty> IsPowerOf2 for Int<N> where Assert<{ N == N.next_power_of_two() }>: IsTrue {}
      impl<const N: $ty, const M: $ty> BitsAllSet<M> for Int<N> where Assert<{ N & M == M }>: IsTrue {}
      impl<const N: $ty, const M: $ty> BitsAnySet<M> for Int<N> where Assert<{ N & M != 0 }>: IsTrue {}
      impl<const N: $ty, const M: $ty> BitsAllClear<M> for Int<N> where Assert<{ (N ^ M) & M == M }>: IsTrue {}
      impl<const N: $ty, const M: $ty> BitsAnyClear<M> for Int<N> where Assert<{ N & M != M }>: IsTrue {}
    }
  };
}

define_bit_bounds!(u8);
define_bit_bounds!(u16);
define_bit_bounds!(u32);
define_bit_bounds!(u64);
define_bit_bounds!(u128);
define_bit_bounds!(usize);
