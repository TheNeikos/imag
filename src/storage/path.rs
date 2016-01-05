use std::path::PathBuf;

use glob::glob;
use glob::Paths;
use glob::PatternError;

use storage::file::id::FileID;
use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;
use module::Module;

/*
 * A path represents either a GLOB ("/tmp/store/module-*-*.imag" for example) or a full path
 *
 * It can be used to generate a File or iterate over some files
 *
 */
struct Path<'a> {

    /*
     * The base part ("/tmp/")
     */
    base: PathBuf,

    /*
     * The store part ("/store/")
     */
    store: PathBuf,

    /*
     * The module
     */
    module: &'a Module,

    /*
     * The ID
     */
    idtype: Option<FileIDType>,
    idhash: Option<FileHash>,
    id:     Option<FileID>,

}

impl<'a> Path<'a> {

    fn new(base: PathBuf, store: PathBuf, m: &'a Module, id: FileID) -> Path<'a> {
        Path {
            base:   base,
            store:  store,
            module: m,
            idtype: Some(id.get_type()),
            idhash: Some(id.get_id()),
            id:     Some(id),
        }
    }

    fn new_with_idtype(base: PathBuf, store: PathBuf, m: &'a Module, id: FileIDType) -> Path<'a> {
        Path {
            base:   base,
            store:  store,
            module: m,
            idtype: Some(id),
            idhash: None,
            id:     None,
        }
    }

    fn new_with_idhash(base: PathBuf, store: PathBuf, m: &'a Module, id: FileHash) -> Path<'a> {
        Path {
            base:   base,
            store:  store,
            module: m,
            idtype: None,
            idhash: Some(id),
            id:     None,
        }
    }

}

/*
 * Transform Path into str, so we can call glob() on it
 */
impl<'a> Into<String> for Path<'a> {

    fn into(self) -> String {
        let mut s = self.base.clone();
        s.push(self.store.clone());
        s.push(self.module.name());
        if self.id.is_some() {
            let idstr : String = self.id.unwrap().into();
            s.push(idstr);
        } else {
            s.push("*");
        }
        s.set_extension("imag");
        s.to_str().unwrap_or("").into()
    }
}

impl<'a> Into<Result<Paths, PatternError>> for Path<'a> {

    fn into(self) -> Result<Paths, PatternError> {
        let s : String = self.into();
        glob(&s[..])
    }

}
