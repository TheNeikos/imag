use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

use clap::ArgMatches;

use runtime::Runtime;
use storage::file::File;
use storage::file::hash::FileHash;
use ui::file::FilePrinter;

pub mod bm;
pub mod helpers;
pub mod notes;
pub mod show;

pub type Link = FileHash;

/**
 * Module interface, each module has to implement this.
 */
pub trait Module : Debug {
    fn exec(&self, matches: &ArgMatches) -> bool;
    fn name(&self) -> &'static str;
    fn runtime(&self) -> &Runtime;
}

/**
 * Module interface, each module which is not a meta-module has to implement this.
 *
 * Meta-modules call modules and therefor don't need these functions
 */
pub trait StoreFileInterfaceModule<'a> : Module + Debug {

    fn links_in_file(&self, Rc<RefCell<File>>) -> Vec<Link>;

    fn print_file<P: FilePrinter>(&self, f: Rc<RefCell<File>>, p: P)
        where Self: Sized
    {
        p.print_file(f);
    }

    fn print_links<P: FilePrinter>(&self, f: Rc<RefCell<File>>, p: P)
        where Self: Sized;

    fn print_meta<P: FilePrinter>(&self, f: Rc<RefCell<File>>, p: P)
        where Self: Sized;

}

