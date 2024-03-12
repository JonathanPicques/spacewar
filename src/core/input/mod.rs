use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Eq, Pod, Copy, Clone, Default, Zeroable, PartialEq)]
pub struct CoreInput {
    input: u8,
}

impl CoreInput {
    #[inline(always)]
    pub fn set(&mut self, bit: u8) {
        self.input |= bit;
    }

    #[inline(always)]
    pub fn unset(&mut self, bit: u8) {
        self.input &= !bit;
    }

    #[inline(always)]
    pub fn is_set(self, bit: u8) -> bool {
        self.input & bit != 0
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.input == 0
    }
}
