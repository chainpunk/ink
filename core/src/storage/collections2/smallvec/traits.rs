// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
    Iter,
    SmallVec,
};
use crate::storage::{
    KeyPtr,
    LazyArray,
    LazyArrayLength,
    PullForward,
    PushForward,
    SaturatingStorage,
    StorageFootprint,
    StorageFootprintOf,
};
use core::{
    iter::{
        Extend,
        FromIterator,
    },
    ops::Add,
};
use typenum::{
    Add1,
    Unsigned,
};

impl<T, N> core::ops::Index<u32> for SmallVec<T, N>
where
    T: StorageFootprint + PullForward,
    N: LazyArrayLength<T>,
{
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T, N> core::ops::IndexMut<u32> for SmallVec<T, N>
where
    T: StorageFootprint + SaturatingStorage + PullForward,
    N: LazyArrayLength<T>,
{
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<'a, T: 'a, N> IntoIterator for &'a SmallVec<T, N>
where
    T: StorageFootprint + PullForward,
    N: LazyArrayLength<T>,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, N> Extend<T> for SmallVec<T, N>
where
    T: StorageFootprint + SaturatingStorage,
    N: LazyArrayLength<T>,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.push(item)
        }
    }
}

impl<T, N> FromIterator<T> for SmallVec<T, N>
where
    T: StorageFootprint + SaturatingStorage,
    N: LazyArrayLength<T>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut vec = SmallVec::new();
        vec.extend(iter);
        vec
    }
}

impl<T, N> StorageFootprint for SmallVec<T, N>
where
    T: StorageFootprint + PullForward,
    N: LazyArrayLength<T>,
    LazyArray<T, N>: StorageFootprint,
    StorageFootprintOf<LazyArray<T, N>>: Add<typenum::B1>,
    Add1<StorageFootprintOf<LazyArray<T, N>>>: Unsigned,
{
    type Value = Add1<StorageFootprintOf<LazyArray<T, N>>>;
}

impl<T, N> PullForward for SmallVec<T, N>
where
    N: LazyArrayLength<T>,
    LazyArray<T, N>: PullForward,
{
    fn pull_forward(ptr: &mut KeyPtr) -> Self {
        Self {
            len: PullForward::pull_forward(ptr),
            elems: PullForward::pull_forward(ptr),
        }
    }
}

impl<T, N> PushForward for SmallVec<T, N>
where
    LazyArray<T, N>: PushForward,
    N: LazyArrayLength<T>,
{
    fn push_forward(&self, ptr: &mut KeyPtr) {
        PushForward::push_forward(&self.len(), ptr);
        PushForward::push_forward(&self.elems, ptr);
    }
}

impl<T, N> core::cmp::PartialEq for SmallVec<T, N>
where
    T: PartialEq + StorageFootprint + PullForward,
    N: LazyArrayLength<T>,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false
        }
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T, N> core::cmp::Eq for SmallVec<T, N>
where
    T: Eq + StorageFootprint + PullForward,
    N: LazyArrayLength<T>,
{
}