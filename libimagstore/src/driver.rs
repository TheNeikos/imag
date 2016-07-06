
use std::default::Default;

#[cfg(not(test))]
pub type StoreDriver = InnerDriver<IoDriver>;

#[cfg(test)]
pub type StoreDriver = InnerDriver<TestDriver>;

pub struct InnerDriver<I>
    where I: DriverImpl
{
    imp: I,
}

#[cfg(not(test))]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        StoreDriver {
            imp: IoDriver,
        }
    }
}

#[cfg(test)]
impl Default for StoreDriver {
    fn default() -> StoreDriver {
        StoreDriver {
            imp: TestDriver,
        }
    }
}

pub trait DriverImpl : Sized {}

pub struct IoDriver;
impl DriverImpl for IoDriver {}

pub struct TestDriver;
impl DriverImpl for TestDriver {}
