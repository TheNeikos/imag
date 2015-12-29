use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use clap::ArgMatches;

use runtime::Runtime;
pub use module::Module;

pub struct Show<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> Show<'a> {
    pub fn new(rt: &'a Runtime<'a>) -> Show<'a> {
        Show {
            rt: rt,
        }
    }
}

impl<'a> Debug for Show<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Show");
        Ok(())
    }

}

impl<'a> Module for Show<'a> {

    fn exec(&self, matches: &ArgMatches) -> bool {
        // Fetch ID from CLI | Fetch regex from CLI
        // Load files for (ID | regex)
        // Check what should be printed (content, links, meta)
        // Initiate FilePrinter
        // Initialize appropriate Module
        // Call printer function on module with File and printer
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "show"
    }

    fn runtime(&self) -> &Runtime {
        &self.rt
    }

}
