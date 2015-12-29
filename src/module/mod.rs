<<<<<<< 670f0e16e9c118e49d82346428adbf2c2d796907
use std::fmt::Debug;
=======
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FMTResult;
use std::result::Result;
use std::rc::Rc;
use std::cell::RefCell;
>>>>>>> Module trait: Add function to fetch links in a File

use clap::ArgMatches;

use runtime::Runtime;
use storage::file::hash::FileHash;

pub mod bm;
pub mod helpers;
pub mod notes;

pub type Link = FileHash;

/**
 * Module interface, each module has to implement this.
 */
pub trait Module<'a> : Debug {
    fn exec(&self, matches: &ArgMatches) -> bool;
    fn name(&self) -> &'static str;

    fn runtime(&self) -> &Runtime;

    fn links_in_file(&self, Rc<RefCell<File>>) -> Vec<Link>;

}

