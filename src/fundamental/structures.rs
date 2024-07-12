
use std::alloc::{alloc, dealloc, Allocator, Global, Layout};
use std::ptr;
use std::mem;

pub(crate) struct ResourcePool {
    memory: *mut u8,
    free_indices: *mut u32,
    allocator: &'static mut dyn Allocator,
    free_indices_head: u32,
    pool_size: u32,
    resource_size: u32,
    used_indices: u32,
}

impl ResourcePool {
    pub(crate) fn init(&mut self, allocator: &'static mut dyn Allocator, pool_size: u32, resource_size: u32) {
        self.allocator = allocator;
        self.pool_size = pool_size;
        self.resource_size = resource_size;
        self.free_indices_head = 0;
        self.used_indices = 0;

        let memory_size = pool_size * resource_size;
        let memory_layout = Layout::from_size_align(memory_size as usize, mem::align_of::<u8>()).unwrap();
        self.memory = unsafe { alloc(memory_layout) };

        let indices_size = pool_size * mem::size_of::<u32>() as u32;
        let indices_layout = Layout::from_size_align(indices_size as usize, mem::align_of::<u32>()).unwrap();
        self.free_indices = unsafe { alloc(indices_layout) as *mut u32 };

        for i in 0..pool_size {
            unsafe {
                *self.free_indices.add(i as usize) = pool_size - i - 1;
            }
        }
    }

    pub(crate) fn shutdown(&mut self) {
        if !self.memory.is_null() {
            let memory_layout = Layout::from_size_align((self.pool_size * self.resource_size) as usize, mem::align_of::<u8>()).unwrap();
            unsafe {
                dealloc(self.memory, memory_layout);
            }
        }

        if !self.free_indices.is_null() {
            let indices_layout = Layout::from_size_align((self.pool_size * mem::size_of::<u32>() as u32) as usize, mem::align_of::<u32>()).unwrap();
            unsafe {
                dealloc(self.free_indices as *mut u8, indices_layout);
            }
        }

        self.memory = ptr::null_mut();
        self.free_indices = ptr::null_mut();
    }

    pub(crate) fn obtain_resource(&mut self) -> u32 {
        if self.free_indices_head < self.pool_size {
            let index = unsafe { *self.free_indices.add(self.free_indices_head as usize) };
            self.free_indices_head += 1;
            self.used_indices += 1;
            index
        } else {
            u32::MAX
        }
    }

    pub(crate) fn release_resource(&mut self, index: u32) {
        if self.used_indices > 0 {
            self.free_indices_head -= 1;
            unsafe {
                *self.free_indices.add(self.free_indices_head as usize) = index;
            }
            self.used_indices -= 1;
        }
    }

    pub(crate) fn free_all_resources(&mut self) {
        self.free_indices_head = 0;
        self.used_indices = 0;
        for i in 0..self.pool_size {
            unsafe {
                *self.free_indices.add(i as usize) = self.pool_size - i - 1;
            }
        }
    }

    pub(crate) fn access_resource(&self, index: u32) -> *mut u8 {
        unsafe { self.memory.add((index * self.resource_size) as usize) }
    }

    pub(crate) fn access_resource_const(&self, index: u32) -> *const u8 {
        unsafe { self.memory.add((index * self.resource_size) as usize) as *const u8 }
    }
}
pub(crate) trait Resource {
    fn get_name(&self) -> &str;
    fn get_pool_index(&self) -> u32;
    fn set_pool_index(&mut self, index :u32) -> u32;

}
pub(crate) struct ResourcePoolTyped<T> 
where T: Resource{
    pool: ResourcePool,
    phantom: std::marker::PhantomData<T>,
}

impl<T> ResourcePoolTyped<T> 
where T: Resource{
    pub(crate) fn init(&mut self, allocator: &'static mut dyn Allocator, pool_size: u32) {
        self.pool.init(allocator, pool_size, mem::size_of::<T>() as u32);
    }

    pub(crate) fn shutdown(&mut self) {
        if self.pool.free_indices_head != 0 {
            println!("Resource pool has unfreed resources.");
            for i in 0..self.pool.free_indices_head {
                unsafe {
                    let index = *self.pool.free_indices.add(i as usize);
                    
                    println!("\tResource {}, {:?}", index, (*self.get(index)).get_name());
                }
            }
        }
        self.pool.shutdown();
    }

    pub(crate) fn obtain(&mut self) -> *mut T {
        let resource_index = self.pool.obtain_resource();
        if resource_index != u32::MAX {
            let resource = self.get(resource_index);
            unsafe { (*resource).set_pool_index(resource_index) ; }
            resource
        } else {
            ptr::null_mut()
        }
    }

    pub(crate) fn release(&mut self, resource: *mut T) {
        unsafe {
            self.pool.release_resource((*resource).get_pool_index());
        }
    }

    pub(crate) fn get(&self, index: u32) -> *mut T {
        self.pool.access_resource(index) as *mut T
    }

    pub(crate) fn get_const(&self, index: u32) -> *const T {
        self.pool.access_resource_const(index) as *const T
    }
}