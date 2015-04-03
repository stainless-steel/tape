//! Basic operations with tape archives (tar).

#![feature(std_misc, unsafe_destructor)]

extern crate libc;

extern crate libtar_sys as raw;

use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::sync::{StaticMutex, MUTEX_INIT};

// libtar is not thread safe.
static LOCK: StaticMutex = MUTEX_INIT;

/// An archive.
pub struct Archive {
    raw: *mut raw::TAR,
}

macro_rules! done(
    ($result:expr) => (
        if $result != 0 {
            return Err(Error::last_os_error());
        }
    );
);

macro_rules! raise(
    ($message:expr) => (
        return Err(Error::new(ErrorKind::Other, $message))
    );
);

macro_rules! path_to_c_str(
    ($path:expr) => (
        match $path.to_str() {
            Some(path) => match CString::new(path) {
                Ok(path) => path,
                Err(_) => raise!("the path is invalid"),
            },
            None => raise!("the path is invalid"),
        }
    );
);

impl Archive {
    /// Open an archive.
    pub fn open(path: &Path) -> Result<Archive> {
        use libc::consts::os::posix88::O_RDONLY;
        use std::ffi::CString;

        let mut tar = 0 as *mut raw::TAR;
        unsafe {
            let _lock = LOCK.lock();
            let path = path_to_c_str!(path);
            done!(raw::tar_open(&mut tar, path.as_ptr(), 0 as *mut _, O_RDONLY, 0, 0));
        }
        Ok(Archive { raw: tar })
    }

    /// Extract all files from the archive into a directory.
    pub fn extract(&self, path: &Path) -> Result<()> {
        use std::ffi::CString;

        unsafe {
            let _lock = LOCK.lock();
            let path = path_to_c_str!(path);
            done!(raw::tar_extract_all(self.raw, path.as_ptr()));
        }
        Ok(())
    }
}

/// Open an archive.
#[inline]
pub fn open(path: &Path) -> Result<Archive> { Archive::open(path) }

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
