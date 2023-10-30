#[derive(Clone)]
pub struct OnMaskIter<'a, const WORD_NUM: usize> {
    index: u32,
    parent: &'a Bitmask<WORD_NUM>,
}

impl<'a, const WORD_NUM: usize> OnMaskIter<'a, WORD_NUM> {
    pub fn new(index: u32, parent: &'a Bitmask<WORD_NUM>) -> Self {
        Self { index, parent }
    }
}

impl<'a, const WORD_NUM: usize> Iterator for OnMaskIter<'a, WORD_NUM> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.parent.get_next_bit_on(self.index);
        if self.index != WORD_NUM as u32 * 64 {
            self.index += 1;
            Some(self.index - 1)
        } else {
            None
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy)]
pub struct Bitmask<const WORD_NUM: usize>([u64; WORD_NUM]);

impl<const WORD_NUM: usize> Default for Bitmask<WORD_NUM> {
    fn default() -> Self { Self::new(false) }
}

impl<const WORD_NUM: usize> Bitmask<WORD_NUM> {
    pub fn new(active: bool) -> Self {
        Self { 0: if active { [u64::MAX; WORD_NUM] } else { [0; WORD_NUM] } }
    }

    pub fn get_next_bit_on(&self, start: u32) -> u32 {
        let mut n = start as u64 >> 6;
        if n >= WORD_NUM as u64 { return WORD_NUM as u32 * 64; }
        
        let m = start & 63;
        let mut word = self.0[n as usize];
        if word & (1u64 << m) > 0 { return start; } // simple case: start is active
    
        word &= (u64::MAX >> m) << m; // mask out lower bits
        while word == 0 {
            n += 1;
            if n == WORD_NUM as u64 { break; }
            word = self.0[n as usize]; // find next non-zero word
        }
        
        if word == 0 {
            WORD_NUM as u32 * usize::BITS
        } else {
            (n << 6) as u32 + self.get_lowest_bit_on(word)
        }
    }

    pub fn get_next_bit_off(&self, start: u32) -> u32 {
        let mut n = start as u64 >> 6;
        if n >= WORD_NUM as u64 { return WORD_NUM as u32 * 64; }
        
        let m = start & 63;
        let mut word = !self.0[n as usize];
        if word & (1u64 << m) > 0 { return start; } // simple case: start is active
    
        word &= (u64::MAX >> m) << m; // mask out lower bits
        while word == 0 {
            n += 1;
            if n == WORD_NUM as u64 { break; }
            word = !self.0[n as usize]; // find next non-zero word
        }
        
        if word == 0 {
            WORD_NUM as u32 * usize::BITS
        } else {
            (n << 6) as u32 + self.get_lowest_bit_on(word)
        }
    }
    
    pub fn get_lowest_bit_on(&self, word: u64) -> u32 {
        word.trailing_zeros()
    }

    pub fn is_on(&self) -> bool {
        for word in self.0 { if word != u64::MAX { return false; } }
        true
    }

    pub fn is_off(&self) -> bool {
        for word in self.0 { if word != 0 { return false; } }
        true
    }

    pub fn is_bit_on(&self, index: usize) -> bool {
        0 != (self.0[index >> 6] & (1u64 << (index & 63)))
    }

    pub fn is_bit_off(&self, index: usize) -> bool {
        0 == (self.0[index >> 6] & (1u64 << (index & 63)))
    }

    pub fn set_on(&mut self) {
        for word in self.0.iter_mut() { *word = u64::MAX; }
    }

    pub fn set_off(&mut self) {
        for word in self.0.iter_mut() { *word = 0; }
    }

    pub fn set_bit(&mut self, index: usize, active: bool) {
        if active { self.set_bit_on(index); } else { self.set_bit_off(index); }
    }

    pub fn set_bit_on(&mut self, index: usize) {
        self.0[index >> 6] |= 1u64 << (index & 63);
    }

    pub fn set_bit_off(&mut self, index: usize) {
        self.0[index >> 6] &= !(1u64 << (index & 63));
    }
}