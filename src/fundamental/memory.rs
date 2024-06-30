use std::{alloc::{alloc, dealloc, Layout}, ffi::c_void, os::raw::c_char};
use log::{debug, error, log_enabled, info, Level};
// Define constants for alignment and memory size
const KILO: usize = 1024;
const MEGA: usize = KILO * KILO;
const GIGA: usize = KILO * MEGA;

#[derive(Debug)]
struct MemoryServiceConfiguration {
    maximum_dynamic_size: usize,
}

impl Default for MemoryServiceConfiguration {
    fn default() -> Self {
        MemoryServiceConfiguration {
            maximum_dynamic_size: 32 * MEGA,
        }
    }
}
#[derive(Debug)]
struct MemoryStatistics {
    allocated_bytes: usize,
    total_bytes: usize,
    allocation_count: u32,
}

// Define a trait for Allocator
trait Allocator {
    fn allocate(&mut self, size: usize, alignment: usize) -> *mut u8;
    fn deallocate(&mut self, pointer: *mut u8);
}

// Implement Allocator trait for HeapAllocator
pub (crate) struct HeapAllocator {
    memory: *mut u8,
    allocated_size: usize,
    max_size: usize,
    tlsf_handle: *mut c_void, // Placeholder for tlsf handle type
}

impl HeapAllocator {
    pub (crate) fn new() -> Self {
        HeapAllocator {
            memory: std::ptr::null_mut(),
            allocated_size: 0,
            max_size: 0,
            tlsf_handle: std::ptr::null_mut(),
        }
    }

    pub (crate) fn init(&mut self, size: usize) {
        let layout = Layout::from_size_align(size, 1).unwrap();
        self.memory = unsafe{ alloc(layout) };
        self.allocated_size = size;
        self.max_size = size;
        self.tlsf_handle = std::ptr::null_mut(); // Initialize tlsf handle as needed
    }

    fn shutdown(&mut self) {
        let layout = Layout::from_size_align(self.allocated_size, 1).unwrap();
        unsafe { dealloc(self.memory, layout) };
        self.memory = std::ptr::null_mut();
        self.allocated_size = 0;
        self.max_size = 0;
        // Shutdown tlsf handle if needed
    }
}

impl Allocator for HeapAllocator {
    fn allocate(&mut self, size: usize, _alignment: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 1).unwrap();
        unsafe { alloc(layout) }
    }

    fn deallocate(&mut self, pointer: *mut u8) {
        let layout = Layout::from_size_align(self.allocated_size, 1).unwrap();
        unsafe { dealloc(pointer, layout);  }
    }
}


struct MemoryService {
    scratch_allocator: LinearAllocator,
    system_allocator: HeapAllocator,
}

impl MemoryService {
    fn new() -> Self {
        MemoryService {
            scratch_allocator: LinearAllocator::new(),
            system_allocator: HeapAllocator::new(), 
        }
    }

    fn init(&mut self, configuration: &MemoryServiceConfiguration) {
        self.system_allocator.init(configuration.maximum_dynamic_size);
        info!("MemoryService initialized");
    }

    fn shutdown(&mut self) {
        self.system_allocator.shutdown();
        info!("MemoryService shutdown");
    }

    #[cfg(feature = "RAPTOR_IMGUI")]
    fn imgui_draw(&mut self) {
        // Implement imgui_draw logic
        info!("Drawing ImGui for MemoryService");
    }

    fn test(&mut self) {
        // Implement test logic
        info!("Running test for MemoryService");
    }
}
// Define macro helpers
macro_rules! ralloca {
    ($size:expr, $allocator:expr) => {
        unsafe { (*$allocator).allocate($size, 1, file!(), line!()) }
    };
}

macro_rules! rallocam {
    ($size:expr, $allocator:expr) => {
        unsafe { (*$allocator).allocate($size, 1, file!(), line!()) as *mut u8 }
    };
}

macro_rules! rallocat {
    ($type:ty, $allocator:expr) => {
        unsafe { (*$allocator).allocate(std::mem::size_of::<$type>(), 1) as *mut $type }
    };
}

macro_rules! rfree {
    ($pointer:expr, $allocator:expr) => {
        unsafe { (*$allocator).deallocate($pointer) }
    };
}

struct DoubleStackAllocator {
    memory: *mut u8,
    top: usize,
    bottom: usize,
    total_size: usize,
}

impl DoubleStackAllocator {
    fn new(size: usize) -> Self {
        DoubleStackAllocator {
            memory: std::ptr::null_mut(),
            top: size,
            bottom: 0,
            total_size: size,
        }
    }

    fn init(&mut self, size: usize) {
        self.memory = unsafe { alloc(Layout::from_size_align(size, 1).unwrap()) as *mut u8 };
        self.top = size;
        self.bottom = 0;
        self.total_size = size;
        info!("DoubleStackAllocator initialized with size {}", size);
    }

    fn shutdown(&mut self) {
        unsafe {
            dealloc(self.memory as *mut u8, Layout::from_size_align(self.total_size, 1).unwrap());
        }
        self.memory = std::ptr::null_mut();
        self.top = 0;
        self.bottom = 0;
        info!("DoubleStackAllocator shutdown");
    }

    fn allocate_top(&mut self, size: usize, alignment: usize) -> *mut u8 {
        let new_start = memory_align(self.top - size, alignment);
        if new_start <= self.bottom {
            panic!("Overflow crossing in allocate_top");
        }
        self.top = new_start;
        unsafe{self.memory.offset(new_start as isize) }
    }

    fn allocate_bottom(&mut self, size: usize, alignment: usize) -> *mut u8 {
        let new_start = memory_align(self.bottom, alignment);
        let new_allocated_size = new_start + size;
        if new_allocated_size >= self.top {
            panic!("Overflow crossing in allocate_bottom");
        }
        self.bottom = new_allocated_size;
        unsafe{self.memory.offset(new_start as isize)}
    }

    fn deallocate_top(&mut self, size: usize) {
        if size > self.total_size - self.top {
            self.top = self.total_size;
        } else {
            self.top += size;
        }
    }

    fn deallocate_bottom(&mut self, size: usize) {
        if size > self.bottom {
            self.bottom = 0;
        } else {
            self.bottom -= size;
        }
    }

    fn get_top_marker(&self) -> usize {
        self.top
    }

    fn get_bottom_marker(&self) -> usize {
        self.bottom
    }

    fn free_top_marker(&mut self, marker: usize) {
        if marker > self.top && marker < self.total_size {
            self.top = marker;
        }
    }

    fn free_bottom_marker(&mut self, marker: usize) {
        if marker < self.bottom {
            self.bottom = marker;
        }
    }

    fn clear_top(&mut self) {
        self.top = self.total_size;
    }

    fn clear_bottom(&mut self) {
        self.bottom = 0;
    }
}

impl Allocator for DoubleStackAllocator {
    fn allocate(&mut self, size: usize, alignment: usize) -> *mut u8 {
        self.allocate_top(size, alignment)
    }

    fn deallocate(&mut self, _pointer: *mut u8) {
        // Deallocate method not implemented for DoubleStackAllocator in original C++ code.
        // Implement if necessary.
    }
}

pub struct StackAllocator {
    memory: *mut u8,
    total_size: usize,
    allocated_size: usize,
}

impl StackAllocator {
    pub fn new(size: usize) -> Self {
        StackAllocator {
            memory: std::ptr::null_mut(),
            total_size: size,
            allocated_size: 0,
        }
    }

    pub fn init(&mut self, size: usize) {
        self.memory = unsafe {
            let layout = Layout::from_size_align(size, 1).unwrap();
            alloc(layout)
        };
        self.total_size = size;
        self.allocated_size = 0;
    }

    pub fn shutdown(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.total_size, 1).unwrap();
            dealloc(self.memory, layout);
        }
        self.memory = std::ptr::null_mut();
        self.total_size = 0;
        self.allocated_size = 0;
    }



    pub fn get_marker(&self) -> usize {
        self.allocated_size
    }

    pub fn free_marker(&mut self, marker: usize) {
        if marker <= self.allocated_size {
            self.allocated_size = marker;
        }
    }

    pub fn clear(&mut self) {
        self.allocated_size = 0;
    }
}
impl Allocator for StackAllocator {
    fn allocate(&mut self, size: usize, alignment: usize) -> *mut u8 {
        let new_start = self.memory.align_offset(alignment);
        let new_allocated_size = new_start + size;
        if new_allocated_size > self.total_size {
            panic!("StackAllocator: Out of memory!");
        }

        self.allocated_size = new_allocated_size;
        unsafe { self.memory.add(new_start) }
    }

    fn deallocate(&mut self, _pointer: *mut u8) {
        // No-op in StackAllocator as deallocation happens implicitly by resetting allocated_size
    }
}
impl Drop for StackAllocator {
    fn drop(&mut self) {
        if !self.memory.is_null() {
            unsafe {
                let layout = Layout::from_size_align(self.total_size, 1).unwrap();
                dealloc(self.memory, layout);
            }
        }
    }
}

struct LinearAllocator {
    memory: *mut u8,
    total_size: usize,
    allocated_size: usize,
}

impl LinearAllocator {
    fn new() -> Self {
        LinearAllocator {
            memory: std::ptr::null_mut(),
            total_size: 0,
            allocated_size: 0,
        }
    }

    fn init(&mut self, size: usize) {
        self.memory = unsafe { alloc(Layout::from_size_align(size, 1).unwrap()) as *mut u8 };
        self.total_size = size;
        self.allocated_size = 0;
        info!("LinearAllocator initialized with size {}", size);
    }

    fn shutdown(&mut self) {
        unsafe {
            dealloc(self.memory as *mut u8, Layout::from_size_align(self.total_size, 1).unwrap());
        }
        self.memory = std::ptr::null_mut();
        self.total_size = 0;
        self.allocated_size = 0;
        info!("LinearAllocator shutdown");
    }

    fn allocate(&mut self, size: usize, alignment: usize) -> *mut u8 {
        let new_start = memory_align(self.allocated_size, alignment);
        assert!(new_start < self.total_size);
        let new_allocated_size = new_start + size;
        assert!(new_allocated_size <= self.total_size);
        self.allocated_size = new_allocated_size;
        unsafe {self.memory.offset(new_start as isize)}
    }

    fn deallocate(&mut self, _pointer: *mut u8) {
        // Deallocate method not implemented for LinearAllocator in original C++ code.
        // Implement if necessary.
    }

    fn clear(&mut self) {
        self.allocated_size = 0;
    }
}

impl Allocator for LinearAllocator {
    fn allocate(&mut self, size: usize, alignment: usize) -> *mut u8 {
        self.allocate(size, alignment)
    }

    fn deallocate(&mut self, _pointer: *mut u8) {
        // Deallocate method not implemented for LinearAllocator in original C++ code.
        // Implement if necessary.
    }
}

struct MallocAllocator;

impl Allocator for MallocAllocator {
    fn allocate(&mut self, size: usize, _alignment: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 1).unwrap();
        unsafe { alloc(layout) }
    }

    fn deallocate(&mut self, pointer: *mut u8) {
        let layout = Layout::from_size_align(std::mem::size_of_val(&pointer), 1 ).unwrap();
        unsafe { dealloc(pointer, layout) };
    }
}


#[inline]
fn memory_align(size: usize, alignment: usize) -> usize {
    // Calculate the aligned size
    let aligned_size = if size % alignment == 0 {
        size
    } else {
        (size / alignment + 1) * alignment
    };

    aligned_size
}
