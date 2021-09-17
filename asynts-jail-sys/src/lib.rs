extern crate libc;

#[repr(C)]
struct ChildArgumentsFFI {
    // The child will be restricted to this directory, which it sees as '/'.
    pub root_directory: *const libc::c_char,
}

pub struct ChildArguments {
    root_directory: std::ffi::CString,
    implementation: ChildArgumentsFFI
}
impl ChildArguments {
    pub fn new(root_directory: &str) -> ChildArguments {
        let mut args = ChildArguments {
            root_directory: std::ffi::CString::new(root_directory).unwrap(),
            implementation: ChildArgumentsFFI {
                root_directory: std::ptr::null(),
            }
        };
        args.implementation.root_directory = args.root_directory.as_ptr();

        args
    }

    // The ChildArguments objects must be kept around as long as the 'ffi' object
    // is used.
    pub unsafe fn ffi(&self) -> *const libc::c_void {
        &self.implementation as *const _ as *const libc::c_void
    }
}

pub struct ChildStack {
    _buffer: [u8; 0x1000]
}
impl ChildStack {
    pub fn new() -> ChildStack {
        ChildStack {
            _buffer: [0; 0x1000]
        }
    }

    pub unsafe fn top(&mut self) -> *mut u8 {
        self._buffer.as_mut_ptr().offset(0x1000)
    }
}

extern "C" {
    fn child_main_impl(argument: *const libc::c_void) -> libc::c_int;
}

pub extern "C" fn child_main(argument: *mut libc::c_void) -> libc::c_int {
    unsafe {
        let args = argument as *const ChildArguments;
        child_main_impl((*args).ffi())
    }
}
