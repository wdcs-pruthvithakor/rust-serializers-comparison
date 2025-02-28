use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Trallocator<A: GlobalAlloc>(pub A, AtomicU64);

unsafe impl<A: GlobalAlloc> GlobalAlloc for Trallocator<A> {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        self.1.fetch_add(l.size() as u64, Ordering::SeqCst);
        self.0.alloc(l)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        self.0.dealloc(ptr, l);
        self.1.fetch_sub(l.size() as u64, Ordering::SeqCst);
    }
}

impl<A: GlobalAlloc> Trallocator<A> {
    pub const fn new(a: A) -> Self {
        Trallocator(a, AtomicU64::new(0))
    }

    pub fn reset(&self) {
        self.1.store(0, Ordering::SeqCst);
    }
    pub fn get(&self) -> u64 {
        self.1.load(Ordering::SeqCst)
    }
}



// use jemalloc_sys::malloc_stats_print;
// use std::ffi::{CStr, CString};
// use std::ptr::{null, null_mut};
// use libc::{c_char, c_void};
// use std::sync::Mutex;
// use std::io::{Write};

// // Global buffer to store memory stats
// static OUTPUT_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

// // Custom callback to capture jemalloc stats output
// extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
//     let msg = unsafe { CStr::from_ptr(message).to_string_lossy() };

//     // Only capture key memory stats
//     let keywords = ["allocated", "active", "mapped", "resident"];
//     if keywords.iter().any(|&key| msg.contains(key)) {
//         let mut buffer = OUTPUT_BUFFER.lock().unwrap();
//         writeln!(buffer, "{}", msg).unwrap();
//     }
// }

// // Function to print filtered memory statistics
// fn mem_print() {
//     {
//         let mut buffer = OUTPUT_BUFFER.lock().unwrap();
//         buffer.clear(); // Clear previous output
//     }

//     unsafe {
//         malloc_stats_print(Some(write_cb), null_mut(), null());
//     }

//     // Print the filtered memory stats
//     let buffer = OUTPUT_BUFFER.lock().unwrap();
//     print!("{}", String::from_utf8_lossy(&buffer));
// }