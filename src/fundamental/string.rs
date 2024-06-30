use std::fmt::Write;
use std::io::Write as _;
pub(crate) struct StringBuffer {
    data: Vec<u8>,
    allocator: Option<*mut HeapAllocator>, // Use Option to denote nullable pointers
}

impl StringBuffer {
    fn new(size: usize, allocator: *mut HeapAllocator) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0); // Allocate buffer with zeros

        StringBuffer {
            data,
            allocator: Some(allocator),
        }
    }

    fn append(&mut self, string: &str) {
        self.data.extend_from_slice(string.as_bytes());
    }

    fn append_f(&mut self, format: &str, args: std::fmt::Arguments) {
        self.data.reserve(format.len()); // Pre-allocate space
        self.data.write_fmt(args).unwrap(); // Write formatted string
    }

    fn clear(&mut self) {
        self.data.clear();
    }

}

use std::collections::HashMap;


use super::HeapAllocator;
