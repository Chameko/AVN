use super::block::Block;

pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

pub struct BlockMeta {
    line_mark: [bool; LINE_COUNT],
    block_mark: bool,
}

impl BlockMeta {
    /// Find the next hole in the block from starting_at. Returns the cursor location in the block
    /// and the limit of its size
    ///
    /// ## Skipping first line
    /// As many small objects may cross line boundaries and have at least 2 lines in size, rather than
    /// checking if they cross the boundary, which takes CPU cycles, we just conservativly mark it and skip.
    ///
    /// Memory
    /// x = object
    /// \[\_,x,*,\_,\_\]
    /// We consider * full as small objects often cross line boundaries
    /// i.e. likely \[\_,x,x,\_,\_\]
    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;

        let starting_line = starting_at / LINE_SIZE;

        for (index, marked) in self.line_mark[starting_line..].iter().enumerate() {
            let abs_index = starting_line + index;

            // count unmarked lines
            if !*marked {
                count += 1;

                // If its the first line in a hole (and not the zeroth line), consider it conservativly marked and skip to the next line
                if count == 1 && abs_index > 0 {
                    continue;
                }

                // record the first hole index
                if start.is_none() {
                    start = Some(abs_index)
                }

                // stop is now at the end of this line
                stop = abs_index + 1;
            }

            // If we reach a marked lien or at the end of the block, see if we have a valid hole to work with
            if count > 0 && (*marked || stop >= LINE_COUNT) {
                if let Some(start) = start {
                    let cursor = start * LINE_SIZE;
                    let limit = stop * LINE_SIZE;

                    return Some((cursor, limit));
                }
            }

            // if the line is marked and we didn't return a new cursor/limit pair
            // reset the hole state
            if *marked {
                count = 0;
                start = None;
            }
        }
        None
    }
}

pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    meta: Box<BlockMeta>,
}

impl BumpBlock {
    /// Return pointer to available space in the block that can hold an object of alloc_size
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor + alloc_size;

        if next_bump > BLOCK_SIZE {
            None
        } else {
            let offset = self.cursor;
            self.cursor = next_bump;
            unsafe { Some(self.block.as_ptr().add(offset) as *const u8) }
        }
    }
}
