use crate::*;
use super::*;

use std::{marker::PhantomData, slice::{Iter, IterMut}};

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Clone)]
pub struct FlatChunk2dTable<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static>([T; CHUNK2D_SIZE], PhantomData<Tag>);

impl<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static> Default for FlatChunk2dTable<T, Tag> {
    fn default() -> Self { Self { 0: [T::default(); CHUNK2D_SIZE], 1: PhantomData } }
}

impl<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static> FlatChunk2dTable<T, Tag> {
    #[inline] pub fn new(data: [T; CHUNK2D_SIZE]) -> Self { Self { 0: data, 1: PhantomData } }

    #[inline] pub fn get(&self, index: usize) -> T { self.0[index] }
    #[inline] pub fn get_mut(&mut self, index: usize) -> &mut T { &mut self.0[index] }
    #[inline] pub unsafe fn get_unchecked(&self, index: usize) -> T { *self.0.get_unchecked(index) }
    #[inline] pub unsafe fn get_unckecked_mut(&mut self, index: usize) -> &mut T { self.0.get_unchecked_mut(index) }
    #[inline] pub fn set(&mut self, index: usize, value: T) { self.0[index] = value; }

    #[inline] pub fn get_all(&self) -> &[T; CHUNK2D_SIZE] { &self.0 }
    #[inline] pub fn get_all_mut(&mut self) -> &mut [T; CHUNK2D_SIZE] { &mut self.0 }
    #[inline] pub fn set_all(&mut self, data: [T; CHUNK2D_SIZE]) { self.0 = data }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Clone)]
pub struct FlatChunk3dTable<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static>([T; CHUNK3D_SIZE], PhantomData<Tag>);

impl<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static> Default for FlatChunk3dTable<T, Tag> {
    fn default() -> Self { Self { 0: [T::default(); CHUNK3D_SIZE], 1: PhantomData } }
}

impl<T: Default + Clone + Copy + PartialEq + Eq + Sync + Send + 'static, Tag: Sync + Send + 'static> FlatChunk3dTable<T, Tag> {
    #[inline] pub fn new(data: [T; CHUNK3D_SIZE]) -> Self { Self { 0: data, 1: PhantomData } }
    #[inline] pub fn new_empty() -> Self { Self { 0: [T::default(); CHUNK3D_SIZE], 1: PhantomData } }

    #[inline] pub fn iter(&self) -> Iter<'_, T> { self.0.iter() }
    #[inline] pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.0.iter_mut() }

    #[inline] pub fn get(&self, index: usize) -> T { self.0[index] }
    #[inline] pub fn get_mut(&mut self, index: usize) -> &mut T { &mut self.0[index] }
    #[inline] pub unsafe fn get_unchecked(&self, index: usize) -> T { *self.0.get_unchecked(index) }
    #[inline] pub unsafe fn get_unckecked_mut(&mut self, index: usize) -> &mut T { self.0.get_unchecked_mut(index) }
    #[inline] pub fn set(&mut self, index: usize, value: T) { self.0[index] = value; }

    #[inline] pub fn get_all(&self) -> &[T; CHUNK3D_SIZE] { &self.0 }
    #[inline] pub fn get_all_mut(&mut self) -> &mut [T; CHUNK3D_SIZE] { &mut self.0 }
    #[inline] pub fn set_all(&mut self, data: [T; CHUNK3D_SIZE]) { self.0 = data }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Clone)]
pub struct ChunkData<T: Default + Clone + Sync + Send + 'static, Tag: Sync + Send + 'static>(T, PhantomData<Tag>);

impl<T: Default + Clone + Sync + Send + 'static, Tag: Sync + Send + 'static> Default for ChunkData<T, Tag> {
    fn default() -> Self { Self { 0: T::default(), 1: PhantomData } }
}

impl<T: Default + Clone + Sync + Send + 'static, Tag: Sync + Send + 'static> ChunkData<T, Tag> {
    #[inline] pub fn new(data: T) -> Self { Self { 0: data, 1: PhantomData } }

    #[inline] pub fn get(&self) -> &T { &self.0 }
    #[inline] pub fn get_mut(&mut self) -> &mut T { &mut self.0 }
    #[inline] pub fn set(&mut self, value: T) { self.0 = value; }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Reflect)]
pub enum ChunkDataType {
    Matter,
    Temperature,
}

pub struct ChunkMatter;
pub struct ChunkTemperature;