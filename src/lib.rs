//! An interface to [libtar][1].
//!
//! [1]: http://www.feep.net/libtar/

#![feature(macro_rules, unsafe_destructor)]

extern crate libc;

use std::io::{IoError, IoResult};

mod raw;

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
            done!(raw::tar_open(&mut tar, path.to_c_str().as_ptr(),
                                0 as *mut _, O_RDONLY, 0, 0));
        }
        Ok(Archive { raw: tar })
    }

    /// Extract all files from the archive into a path.
    pub fn extract_all(&self, path: &Path) -> IoResult<()> {
        unsafe {
            done!(raw::tar_extract_all(self.raw, path.to_c_str().as_ptr()));
        }
        Ok(())
    }
}

#[unsafe_destructor]
impl Drop for Archive {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::tar_close(self.raw);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn extract_all() {
        use std::io::TempDir;
        use std::io::fs::PathExtensions;

        let foo = Path::new("tests").join_many(["fixtures", "foo.tar"]);
        assert!(foo.exists());

        let dir = TempDir::new("rs-tar").unwrap();

        let tar = ::Archive::open(&foo).unwrap();
        tar.extract_all(dir.path()).unwrap();

        let bar = dir.path().join("bar.txt");
        assert!(bar.exists());
    }
}
