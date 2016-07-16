
use std::default::Default;

#[cfg(not(test))]
pub type StoreDriver = IoDriver;

#[cfg(test)]
pub type StoreDriver = TestDriver;

#[cfg(not(test))]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        StoreDriver { }
    }
}

#[cfg(test)]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        StoreDriver { }
    }
}

pub trait DriverImpl : Sized {}

pub struct IoDriver;
impl DriverImpl for IoDriver {}

pub struct TestDriver;
impl DriverImpl for TestDriver {}
