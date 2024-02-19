use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Eq, Pod, Copy, Clone, Zeroable, PartialEq)]
pub struct CoreInput {
    pub input: u8,
}

impl CoreInput {
    pub fn new() -> Self {
        Self::zeroed()
    }
}

impl CoreInput {
    pub fn set(&mut self, bit: u8) {
        self.input |= bit;
    }

    pub fn is_set(self, bit: u8) -> bool {
        self.input & bit != 0
    }

    pub fn is_empty(self) -> bool {
        self.input == 0
    }
}
