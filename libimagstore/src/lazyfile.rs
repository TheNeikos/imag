use error::{MapErrInto, StoreError as SE, StoreErrorKind as SEK};
use std::io::{self, Seek, SeekFrom, Read, Write};
use std::path::PathBuf;
use std::fs::File;
use std::default::Default;

use driver::{StoreDriver, DriverImpl};

pub trait FileOps : Read + Write {
    fn trim(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl FileOps for File {
    fn trim(&mut self) -> io::Result<()> {
        self.set_len(0)
    }
}

/// `LazyFile` type
///
/// A lazy file is either absent, but a path to it is available, or it is present.
#[derive(Debug)]
pub enum LazyFile {
    Absent(PathBuf),
    File(File, PathBuf)
}

impl LazyFile {

    /**
     * Get the mutable file behind a LazyFile object
     */
    pub fn get_file_mut(&mut self) -> Result<&mut FileOps, SE> {
        debug!("Getting lazy file: {:?}", self);
        let file = match *self {
            LazyFile::File(ref mut f, _) => return {
                // We seek to the beginning of the file since we expect each
                // access to the file to be in a different context
                f.seek(SeekFrom::Start(0))
                    .map_err_into(SEK::FileNotCreated)
                    .map(|_| f as &mut FileOps)
            },
            LazyFile::Absent(ref p) =>
                try!(StoreDriver::default().open_file(p).map_err_into(SEK::FileNotFound)),
        };

        let path;
        if let LazyFile::Absent(ref p) = *self {
            path = p.clone();
        } else {
            unreachable!();
        }

        *self = LazyFile::File(file, path);
        if let LazyFile::File(ref mut f, _) = *self {
            return Ok(f);
        }
        unreachable!()
    }

    /**
     * Create a file out of this LazyFile object
     */
    pub fn create_file(&mut self) -> Result<&mut FileOps, SE> {
        debug!("Creating lazy file: {:?}", self);
        let file = match *self {
            LazyFile::File(ref mut f, _) => return Ok(f),
            LazyFile::Absent(ref p) =>
                try!(StoreDriver::default().create_file(p).map_err_into(SEK::FileNotFound)),
        };

        let path;
        if let LazyFile::Absent(ref p) = *self {
            path = p.clone();
        } else {
            unreachable!();
        }

        *self = LazyFile::File(file, path);
        if let LazyFile::File(ref mut f, _) = *self {
            return Ok(f);
        }
        unreachable!()
    }

    pub fn move_to(&mut self, new_path: &PathBuf, remove: bool) -> Result<u64, SE> {
        let path = match *self {
            LazyFile::File(_, ref p) => p.clone(),
            LazyFile::Absent(ref p) => p.clone()
        };

        let ret = try!(StoreDriver::default().copy(&path, new_path));

        if remove {
            try!(StoreDriver::default().remove_file(&path));
        }

        *self = match *self {
            LazyFile::File(ref f, _) => LazyFile::File(try!(f.try_clone()), new_path.clone()),
            LazyFile::Absent(_) => LazyFile::Absent(new_path.clone()),
        };

        Ok(ret)
    }

    pub fn remove(&mut self) -> Result<(), SE> {
        try!(match *self {
            LazyFile::File(_, ref p) => StoreDriver::default().remove_file(p),
            LazyFile::Absent(ref p) => StoreDriver::default().remove_file(p),
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use super::LazyFile;
    // use std::io::{Read, Write};
    // use std::path::PathBuf;
    // use tempdir::TempDir;

    // fn get_dir() -> TempDir {
    //     TempDir::new("test-image").unwrap()
    // }

    // #[test]
    // fn lazy_file() {
    //     let dir = get_dir();
    //     let mut path = PathBuf::from(dir.path());
    //     path.set_file_name("test1");
    //     let mut lf = LazyFile::Absent(path);

    //     write!(lf.create_file().unwrap(), "Hello World").unwrap();
    //     dir.close().unwrap();
    // }

    // #[test]
    // fn lazy_file_with_file() {
    //     let dir = get_dir();
    //     let mut path = PathBuf::from(dir.path());
    //     path.set_file_name("test2");
    //     let mut lf = LazyFile::Absent(path.clone());

    //     {
    //         let mut file = lf.create_file().unwrap();

    //         file.write(b"Hello World").unwrap();
    //         file.sync_all().unwrap();
    //     }

    //     {
    //         let mut file = lf.get_file_mut().unwrap();
    //         let mut s = Vec::new();
    //         file.read_to_end(&mut s).unwrap();
    //         assert_eq!(s, "Hello World".to_string().into_bytes());
    //     }

    //     dir.close().unwrap();
    // }
}
