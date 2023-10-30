use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Clone)]
pub struct Leaf2d<T: Default + Clone + Copy + Sync + Send + 'static> {
    data: [T; CHUNK_2D_SIZE],
    active_mask: Bitmask<CHUNK_2D_WORD_NUM>,
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> Default for Leaf2d<T> {
    fn default() -> Self {
        Self { data: [T::default(); CHUNK_2D_SIZE], active_mask: Bitmask::<CHUNK_2D_WORD_NUM>::new(false) }
    }
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> Leaf2d<T> {
    #[inline]
    pub fn new(data: [T; CHUNK_2D_SIZE], active_mask: Bitmask<CHUNK_2D_WORD_NUM>) -> Self {
        Self { data, active_mask}
    }

    #[inline]
    pub fn default_arc_rwlock() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::default()))
    }

    #[inline]
    pub fn new_arc_rwlock(data: [T; CHUNK_2D_SIZE], active_mask: Bitmask<CHUNK_2D_WORD_NUM>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(data, active_mask)))
    }

    #[inline] pub fn data(&self) -> &[T; CHUNK_2D_SIZE] { &self.data }
    #[inline] pub fn active_mask(&self) -> &Bitmask<CHUNK_2D_WORD_NUM> { &self.active_mask }

    #[inline] pub fn get_value(&self, index: usize) -> T { self.data[index] }
    #[inline] pub unsafe fn get_value_unchecked(&self, index: usize) -> T { *self.data.get_unchecked(index) }
    
    #[inline]
    pub fn set_value_on(&mut self, index: usize, value: T) {
        self.data[index] = value;
        self.active_mask.set_bit_on(index);
    }
    
    #[inline]
    pub fn set_value_off(&mut self, index: usize) {
        self.active_mask.set_bit_off(index);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Clone)]
pub struct Leaf3d<T: Default + Clone + Copy + Sync + Send + 'static> {
    data: [T; CHUNK_3D_SIZE],
    active_mask: Bitmask<CHUNK_3D_WORD_NUM>,
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> Default for Leaf3d<T> {
    fn default() -> Self {
        Self { data: [T::default(); CHUNK_3D_SIZE], active_mask: Bitmask::<CHUNK_3D_WORD_NUM>::new(false) }
    }
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> Leaf3d<T> {
    #[inline] pub fn new(data: [T; CHUNK_3D_SIZE], active_mask: Bitmask<CHUNK_3D_WORD_NUM>) -> Self {
        Self { data, active_mask}
    }

    #[inline] pub fn default_arc_rwlock() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::default()))
    }

    #[inline] pub fn new_arc_rwlock(data: [T; CHUNK_3D_SIZE], active_mask: Bitmask<CHUNK_3D_WORD_NUM>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(data, active_mask)))
    }
    
    #[inline] pub fn data(&self) -> &[T; CHUNK_3D_SIZE] { &self.data }
    #[inline] pub fn active_mask(&self) -> &Bitmask<CHUNK_3D_WORD_NUM> { &self.active_mask }

    #[inline] pub fn get_value(&self, index: usize) -> T { self.data[index] }
    #[inline] pub unsafe fn get_value_unchecked(&self, index: usize) -> T { *self.data.get_unchecked(index) }
    
    #[inline]
    pub fn set_value_on(&mut self, index: usize, value: T) {
        self.data[index] = value;
        self.active_mask.set_bit_on(index);
    }
    
    #[inline]
    pub fn set_value_off(&mut self, index: usize) {
        self.active_mask.set_bit_off(index);
    }
}