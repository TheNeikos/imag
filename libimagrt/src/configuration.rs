use std::fmt::{Debug, Formatter, Error};
use std::path::PathBuf;

pub use config::types::Config;
pub use config::reader::from_file;

pub struct Configuration {
    pub verbosity: bool,
    pub editor: Option<String>,
    pub editor_opts: String,
}

impl Configuration {

    pub fn new(rtp: &PathBuf) -> Configuration {
        let cfg = fetch_config(&rtp);

        let verbosity   = cfg.lookup_boolean("verbosity").unwrap_or(false);
        let editor      = cfg.lookup_str("editor").map(String::from);
        let editor_opts = String::from(cfg.lookup_str("editor-opts").unwrap_or(""));

        debug!("Building configuration");
        debug!("  - verbosity  : {:?}", verbosity);
        debug!("  - editor     : {:?}", editor);
        debug!("  - editor-opts: {}", editor_opts);

        Configuration {
            verbosity: verbosity,
            editor: editor,
            editor_opts: editor_opts,
        }
    }

}

fn fetch_config(rtp: &PathBuf) -> Config {
    use std::process::exit;

    let mut configpath = rtp.clone();
    configpath.push("/config");
    from_file(&configpath).map_err(|e| {
        println!("Error loading config at '{:?}' -> {:?}", configpath.to_str(), e);
        println!("Exiting now.");
        exit(1)
    }).unwrap()
}

impl Debug for Configuration {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "Configuration (verbose: {})", self.verbosity));
        Ok(())
    }

}

