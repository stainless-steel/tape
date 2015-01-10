//! Basic operations with tape archives (tar).

#![allow(unstable)]
#![feature(unsafe_destructor)]

extern crate libc;

extern crate "libtar-sys" as raw;

use std::sync::{StaticMutex, MUTEX_INIT};
use std::io::{IoError, IoResult};

// libtar is not thread safe.
static LOCK: StaticMutex = MUTEX_INIT;

/// An archive.
pub struct Archive {
    raw: *mut raw::TAR,
}

macro_rules! done(
    ($result:expr) => (
        if $result != 0 {
            return Err(IoError::last_error());
        }
    );
);

impl Archive {
    /// Open an archive.
    pub fn open(path: &Path) -> IoResult<Archive> {
        use libc::consts::os::posix88::O_RDONLY;
        use std::ffi::CString;

        let mut tar = 0 as *mut raw::TAR;
        unsafe {
            let _lock = LOCK.lock();
            done!(raw::tar_open(&mut tar, CString::from_slice(path.as_vec()).as_ptr(),
                                0 as *mut _, O_RDONLY, 0, 0));
        }
        Ok(Archive { raw: tar })
    }

    /// Extract all files from the archive into a directory.
    pub fn extract(&self, path: &Path) -> IoResult<()> {
        use std::ffi::CString;

        unsafe {
            let _lock = LOCK.lock();
            done!(raw::tar_extract_all(self.raw, CString::from_slice(path.as_vec()).as_ptr()));
        }
        Ok(())
    }
}

/// Open an archive.
#[inline]
pub fn open(path: &Path) -> IoResult<Archive> { Archive::open(path) }

#[unsafe_destructor]
impl Drop for Archive {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _lock = LOCK.lock();
            raw::tar_close(self.raw);
        }
    }
}
