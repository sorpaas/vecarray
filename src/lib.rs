#![cfg_attr(not(feature = "std"), no_std)]

//! Generic array backed by Vec.

extern crate alloc;

use typenum::Unsigned;
use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;
use core::convert::TryFrom;
use alloc::vec::Vec;
#[cfg(feature = "serde")]
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

impl<T, N: Unsigned> AsRef<[T]> for VecArray<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

pub enum TryFromError {
    InvalidLength,
}

impl<T, N: Unsigned> TryFrom<Vec<T>> for VecArray<T, N> {
    type Error = TryFromError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != N::to_usize() {
            return Err(TryFromError::InvalidLength)
        }

        Ok(Self(value, PhantomData))
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize, N: Unsigned> Serialize for VecArray<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>, N: Unsigned> Deserialize<'de> for VecArray<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        if vec.len() != N::to_usize() {
            return Err(<D::Error as serde::de::Error>::custom("invalid length"))
        }

        Ok(Self(vec, PhantomData))
    }
}
