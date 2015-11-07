//! Basic operations with tape archives (tar).
//!
//! ## Example
//!
//! ```rust
//! # extern crate tape;
//! # extern crate temporary;
//! let (from, into) = ("foo.tar", "bar");
//! # let from = "tests/fixtures/foo.tar";
//! # let directory = temporary::Directory::new("tape").unwrap();
//! # let into = directory.path();
//! tape::open(from).unwrap().extract(into).unwrap();
//! ```

#[macro_use]
extern crate lazy_static;
extern crate libc;

extern crate tar_sys as raw;

use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    // libtar is not thread safe.
    static ref MUTEX: Mutex<isize> = Mutex::new(0);
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

macro_rules! path_to_cstr(
    ($path:expr) => (match $path.to_str() {
        Some(path) => match CString::new(path) {
            Ok(path) => path,
            _ => raise!("the path is invalid"),
        },
        _ => raise!("the path is invalid"),
    });
);

/// An archive.
pub struct Archive {
    raw: *mut raw::TAR,
}

impl Archive {
    /// Open an archive.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Archive> {
        use libc::O_RDONLY;
        use std::ffi::CString;

        let mut tar = 0 as *mut raw::TAR;
        unsafe {
            let _guard = MUTEX.lock().unwrap();
            let path = path_to_cstr!(path.as_ref());
            done!(raw::tar_open(&mut tar, path.as_ptr(), 0 as *mut _, O_RDONLY, 0, 0));
        }
        Ok(Archive { raw: tar })
    }

    /// Extract all files from the archive into a directory.
    pub fn extract<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        use std::ffi::CString;

        unsafe {
            let _guard = MUTEX.lock().unwrap();
            let path = path_to_cstr!(path.as_ref());
            done!(raw::tar_extract_all(self.raw, path.as_ptr()));
        }
        Ok(())
    }
}

/// Open an archive.
#[inline]
pub fn open<T: AsRef<Path>>(path: T) -> Result<Archive> {
    Archive::open(path)
}

impl Drop for Archive {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _guard = MUTEX.lock().unwrap();
            raw::tar_close(self.raw);
        }
    }
}
