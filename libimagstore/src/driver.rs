use std::fs::{File, OpenOptions, create_dir_all, copy, remove_file};
use std::io::Result as IoResult;
use std::path::Path;

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::io;
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
use std::sync::{Mutex, Arc};

use std::default::Default;

#[cfg(not(test))]
pub type StoreDriver = IoDriver;

#[cfg(test)]
pub type StoreDriver = TestDriver;

#[cfg(not(test))]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        IoDriver { }
    }
}

#[cfg(test)]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        TestDriver {
            map: global_map.clone(),
        }
    }
}

pub trait DriverImpl : Sized {
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()>;
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<u64>;
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()>;
    fn open_file<P: AsRef<Path>>(&mut self, path: P) -> IoResult<File>;
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> IoResult<File>;
}

pub struct IoDriver;
impl DriverImpl for IoDriver {
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()> {
        create_dir_all(path)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<u64> {
        copy(from, to)
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()> {
        remove_file(path)
    }

    fn open_file<A: AsRef<Path>>(&mut self, p: A) -> IoResult<File> {
        OpenOptions::new().write(true).read(true).open(p)
    }

    fn create_file<A: AsRef<Path>>(&mut self, p: A) -> IoResult<File> {
        if let Some(parent) = p.as_ref().parent() {
            debug!("Implicitely creating directory: {:?}", parent);
            if let Err(e) = create_dir_all(parent) {
                return Err(e);
            }
        }
        OpenOptions::new().write(true).read(true).create(true).open(p)
    }
}

#[cfg(test)]
lazy_static! {
    static ref global_map: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

#[cfg(test)]
pub struct TestDriver {
    map: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>
}

#[cfg(test)]
impl DriverImpl for TestDriver {
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()> {
        Ok(())
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<u64> {
        let mut map = self.map.lock().unwrap();
        let file = match map.get(from.as_ref()) {
            Some(p) => p,
            None =>
                return Err(io::Error::new(io::ErrorKind::NotFound, "File doesn't exist in HashMap"))
        }.clone();
        let len = file.len();
        map.insert(to.as_ref().to_owned(), file);
        Ok(len as u64)
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> IoResult<()> {
        let mut map = self.map.lock().unwrap();
        map.remove(path.as_ref());
        Ok(())
    }

    fn open_file<A: AsRef<Path>>(&mut self, p: A) -> IoResult<File> {
        unimplemented!()
    }

    fn create_file<A: AsRef<Path>>(&mut self, p: A) -> IoResult<File> {
        unimplemented!()
    }
}
