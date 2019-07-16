#![cfg_attr(not(feature = "std"), no_std)]

//! Generic array backed by Vec.

extern crate alloc;

use typenum::Unsigned;
use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;
use alloc::vec::Vec;
#[cfg(all(feature = "serde", feature = "std"))]
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VecArray<T, N: Unsigned>(Vec<T>, PhantomData<N>);

impl<T: Default, N: Unsigned> Default for VecArray<T, N> {
    fn default() -> Self {
        let mut ret = Vec::new();
        ret.resize_with(N::to_usize(), Default::default);
        Self(ret, PhantomData)
    }
}

impl<T, N: Unsigned> Deref for VecArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, N: Unsigned> DerefMut for VecArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(all(feature = "serde", feature = "std"))]
impl<T: Serialize, N: Unsigned> Serialize for VecArray<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(all(feature = "serde", feature = "std"))]
impl<'de, T: Deserialize<'de>, N: Unsigned> Deserialize<'de> for VecArray<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Vec::<T>::deserialize(deserializer)?, PhantomData))
    }
}
