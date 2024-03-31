// made to run with unix (else use std::os::windows)

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
// CStr = c to rust, CString = rust to c, OS : to and from the OS
use std::os::unix::ffi::OsStrExt;
// or make : std::os::windows to run on windows

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        let path = CString::new(path).map_err(|e| e.to_string())?;  // convert path to CString
        let dir = unsafe { ffi::opendir(path.as_ptr()) };   // dir : call opendir
        if dir.is_null() {
            return Err("opendir failed".to_string());
        }
        Ok(DirectoryIterator {
            path,
            dir,
        })
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        loop {
            let entry = unsafe { ffi::readdir(self.dir) };
            if entry.is_null() {
                return None;
            }
            let entry = unsafe { &*entry };  // get entry from readdir
            let name = unsafe { CStr::from_ptr(entry.d_name.as_ptr()) };
            let name = name.to_bytes();
            if name == b"." || name == b".." {  // skip . and ..
                continue;
            }
            return Some(OsStr::from_bytes(name).to_os_string());
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        unsafe { ffi::closedir(self.dir) };
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}