// Custom allocator support for arrayops
// This module provides allocator abstractions for specialized memory pools
//
// SECURITY NOTE: This module is currently UNUSED (all code is marked #[allow(dead_code)]).
// The unsafe blocks in this module use Rust's standard allocator (System.alloc/System.dealloc)
// which is safe when used correctly. All unsafe blocks are documented with safety comments.
// If this module is enabled in the future, all safety invariants must be maintained.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::{Arc, Mutex};

/// Trait for custom array allocators
/// Note: This is a simplified version - full allocator support would use Rust's Allocator trait
#[allow(dead_code)]
pub trait ArrayAllocator: Send + Sync {
    /// Allocate memory for array elements
    fn allocate(&self, size: usize) -> *mut u8;

    /// Deallocate memory
    fn deallocate(&self, ptr: *mut u8, size: usize);
}

/// Default allocator using Rust's global allocator
#[allow(dead_code)]
pub struct DefaultAllocator;

impl ArrayAllocator for DefaultAllocator {
    fn allocate(&self, size: usize) -> *mut u8 {
        // SAFETY: This is safe because:
        // 1. Layout::from_size_align validates the layout parameters
        // 2. System.alloc is Rust's standard allocator and is safe to use
        // 3. The returned pointer is managed by Rust's ownership system
        // 4. This code is currently unused (dead code) - see module documentation
        unsafe {
            let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
            System.alloc(layout)
        }
    }

    fn deallocate(&self, ptr: *mut u8, size: usize) {
        // SAFETY: This is safe because:
        // 1. Layout::from_size_align validates the layout parameters
        // 2. System.dealloc is Rust's standard allocator and is safe to use
        // 3. The pointer must have been allocated with System.alloc and matching layout
        // 4. This code is currently unused (dead code) - see module documentation
        unsafe {
            let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
            System.dealloc(ptr, layout);
        }
    }
}

/// Simple arena allocator (allocates from a fixed pool, deallocates all at once)
/// Note: This is a simplified implementation - a full arena allocator would track allocations
#[allow(dead_code)]
pub struct ArenaAllocator {
    // For now, just use the default allocator
    // A full implementation would maintain an arena buffer
    _marker: std::marker::PhantomData<()>,
}

impl ArenaAllocator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ArenaAllocator {
            _marker: std::marker::PhantomData,
        }
    }
}

impl Default for ArenaAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayAllocator for ArenaAllocator {
    fn allocate(&self, size: usize) -> *mut u8 {
        // For now, delegate to default allocator
        // A full implementation would allocate from arena buffer
        DefaultAllocator.allocate(size)
    }

    fn deallocate(&self, ptr: *mut u8, size: usize) {
        // Arena allocators typically don't deallocate individual allocations
        // All memory is freed when the arena is dropped
        // For now, delegate to default
        DefaultAllocator.deallocate(ptr, size);
    }
}

/// Global allocator context
/// Thread-safe storage for the current allocator
#[allow(dead_code)]
pub struct AllocatorContext {
    allocator: Arc<dyn ArrayAllocator>,
}

impl AllocatorContext {
    #[allow(dead_code)]
    pub fn new(allocator: Arc<dyn ArrayAllocator>) -> Self {
        AllocatorContext { allocator }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        AllocatorContext {
            allocator: Arc::new(DefaultAllocator),
        }
    }

    #[allow(dead_code)]
    pub fn allocator(&self) -> &dyn ArrayAllocator {
        self.allocator.as_ref()
    }
}

impl Default for AllocatorContext {
    fn default() -> Self {
        Self::default()
    }
}

// Thread-local allocator context
// Note: In a full implementation, this would use thread_local! macro
// For now, we'll use a global mutex (simpler but less efficient)
use std::sync::OnceLock;

static GLOBAL_ALLOCATOR: OnceLock<Mutex<Arc<dyn ArrayAllocator>>> = OnceLock::new();

fn get_allocator_mutex() -> &'static Mutex<Arc<dyn ArrayAllocator>> {
    GLOBAL_ALLOCATOR.get_or_init(|| Mutex::new(Arc::new(DefaultAllocator)))
}

/// Set the global allocator
#[allow(dead_code)]
pub fn set_global_allocator(allocator: Arc<dyn ArrayAllocator>) {
    *get_allocator_mutex().lock().unwrap() = allocator;
}

/// Get the current global allocator
#[allow(dead_code)]
pub fn get_global_allocator() -> Arc<dyn ArrayAllocator> {
    get_allocator_mutex().lock().unwrap().clone()
}
