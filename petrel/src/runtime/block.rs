use crate::diagnostic::BlockError;
use std::ptr::NonNull;

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

pub struct Block {
    ptr: BlockPtr,
    size: BlockSize,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block {
            ptr: internal::alloc_block(size)?,
            size,
        })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        internal::dealloc_block(self.ptr, self.size)
    }
}

mod internal {
    use crate::diagnostic::BlockError;

    use super::{BlockPtr, BlockSize};
    use std::{
        alloc::{alloc, dealloc, Layout},
        ptr::NonNull,
    };

    pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            let ptr = alloc(layout);
            if ptr.is_null() {
                Err(BlockError::OOM)
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            dealloc(ptr.as_ptr(), layout);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Block;

    /// Ensure that the allocated block is a power of 2
    #[test]
    fn block_allign() {
        let size = 4;
        let mask = size - 1;
        let block = Block::new(size).unwrap();
        println!("Block ptr: {:#x}", block.as_ptr() as usize);
        assert_eq!(block.as_ptr() as usize & mask ^ mask, mask)
    }
}
