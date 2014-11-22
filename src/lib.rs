//! Basic operations with tape archives (tar).

#![feature(macro_rules, unsafe_destructor)]

extern crate libc;
extern crate sync;

extern crate "libtar-sys" as raw;

use self::sync::mutex::{StaticMutex, MUTEX_INIT};
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
)

impl Archive {
    /// Open an archive.
    pub fn open(path: &Path) -> IoResult<Archive> {
        use libc::consts::os::posix88::O_RDONLY;

        let mut tar = 0 as *mut raw::TAR;
        unsafe {
            let _lock = LOCK.lock();
            done!(raw::tar_open(&mut tar, path.to_c_str().as_ptr(),
                                0 as *mut _, O_RDONLY, 0, 0));
        }
        Ok(Archive { raw: tar })
    }

    /// Extract all files from the archive into a directory.
    pub fn extract(&self, path: &Path) -> IoResult<()> {
        unsafe {
            let _lock = LOCK.lock();
            done!(raw::tar_extract_all(self.raw, path.to_c_str().as_ptr()));
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
