use std::mem;


pub struct DarkAlloc<'m> {
    raw: &'m mut [u8], // Raw memory supplied for the program
}

impl<'m> DarkAlloc<'m> {
    pub fn new(raw: &'m mut [u8]) -> DarkAlloc<'m> {
        DarkAlloc { raw }
    }

    pub fn alloc<T>(&mut self, value: T) -> &'m mut T {
        let value_size = mem::size_of::<T>();
        let value_align = mem::align_of::<T>();

        // Calculate padding needed to align value properly 
        let padding = match self.raw.get(0..value_size) {
            Some(first) => {
                let offset = first.as_ptr() as usize; // Get memory address
                value_align - (offset % value_align)
            }
            None => panic!("Not enough memory"),
        };

        let size = value_size + padding; // Size of allocation
        let (_, rest) = self.raw.split_at_mut(size); // Split off data bytes
        let (fill, _) = rest.split_at_mut(padding);

        unsafe {
            let fill_ptr = fill.as_mut_ptr() as *mut T;
            *fill_ptr = value;
            &mut *fill_ptr
        }
    }
}