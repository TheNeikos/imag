use std::path::PathBuf;
use std::process::Command;
use std::env;

pub use clap::App;

use clap::{Arg, ArgMatches};
use log;
use log::LogLevelFilter;

use configuration::Configuration;
use error::RuntimeError;
use error::RuntimeErrorKind;
use logger::ImagLogger;

use libimagstore::store::Store;

pub struct Runtime<'a> {
    rtp: PathBuf,
    configuration: Option<Configuration>,
    cli_matches: ArgMatches<'a>,
    store: Store,
}

impl<'a> Runtime<'a> {

    /**
     * Gets the CLI spec for the program and retreives the config file path (or uses the default on
     * in $HOME/.imag/config, $XDG_CONFIG_DIR/imag/config or from env("$IMAG_CONFIG")
     * and builds the Runtime object with it.
     *
     * The cli_spec object should be initially build with the ::get_default_cli_builder() function.
     *
     */
    pub fn new(cli_spec: App<'a, 'a>) -> Result<Runtime<'a>, RuntimeError> {
        use std::env;
        use std::error::Error;

        use configuration::error::ConfigErrorKind;

        let matches = cli_spec.get_matches();
        let rtp : PathBuf = matches.value_of("runtimepath")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                env::var("HOME")
                    .map(PathBuf::from)
                    .map(|mut p| { p.push(".imag"); p})
                    .unwrap_or_else(|_| {
                        panic!("You seem to be $HOME-less. Please get a $HOME before using this software. We are sorry for you and hope you have some accommodation anyways.");
                    })
            });
        let storepath = matches.value_of("storepath")
                                .map(PathBuf::from)
                                .unwrap_or({
                                    let mut spath = rtp.clone();
                                    spath.push("store");
                                    spath
                                });

        let cfg = Configuration::new(&rtp);
        let cfg = if cfg.is_err() {
            let e = cfg.err().unwrap();
            if e.kind() != ConfigErrorKind::NoConfigFileFound {
                let cause : Option<Box<Error>> = Some(Box::new(e));
                return Err(RuntimeError::new(RuntimeErrorKind::Instantiate, cause));
            } else {
                None
            }
        } else {
            Some(cfg.unwrap())
        };

        let store_config = {
            match &cfg {
                &Some(ref c) => c.store_config().map(|c| c.clone()),
                _ => None
            }
        };

        Store::new(storepath, store_config).map(|store| {
            Runtime {
                cli_matches: matches,
                configuration: cfg,
                rtp: rtp,
                store: store,
            }
        })
        .map_err(|e| {
            RuntimeError::new(RuntimeErrorKind::Instantiate, Some(Box::new(e)))
        })
    }

    /**
     * Get a commandline-interface builder object from `clap`
     *
     * This commandline interface builder object already contains some predefined interface flags:
     *   * -v | --verbose for verbosity
     *   * --debug for debugging
     *   * -c <file> | --config <file> for alternative configuration file
     *   * -r <path> | --rtp <path> for alternative runtimepath
     *   * --store <path> for alternative store path
     * Each has the appropriate help text included.
     *
     * The `appname` shall be "imag-<command>".
     */
    pub fn get_default_cli_builder(appname: &'a str,
                                   version: &'a str,
                                   about: &'a str)
        -> App<'a, 'a>
    {
        App::new(appname)
            .version(version)
            .author("Matthias Beyer <mail@beyermatthias.de>")
            .about(about)
            .arg(Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .help("Enables verbosity")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("debugging")
                .long("debug")
                .help("Enables debugging output")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("config")
                .long("config")
                .help("Path to alternative config file")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("runtimepath")
                .long("rtp")
                .help("Alternative runtimepath")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("storepath")
                .long("store")
                .help("Alternative storepath. Must be specified as full path, can be outside of the RTP")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("editor")
                .long("editor")
                .help("Set editor")
                .required(false)
                .takes_value(true))
    }

    /**
     * Initialize the internal logger
     */
    pub fn init_logger(&self) {
        let lvl = if self.is_debugging() {
            LogLevelFilter::Debug
        } else if self.is_verbose() {
            LogLevelFilter::Info
        } else {
            LogLevelFilter::Error
        };

        log::set_logger(|max_log_lvl| {
            max_log_lvl.set(lvl);
            debug!("Init logger with {}", lvl);
            Box::new(ImagLogger::new(lvl.to_log_level().unwrap()))
        })
        .map_err(|_| {
            panic!("Could not setup logger");
        })
        .ok();
    }

    /**
     * Get the verbosity flag value
     */
    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbosity")
    }

    /**
     * Get the debugging flag value
     */
    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debugging")
    }

    /**
     * Get the runtimepath
     */
    pub fn rtp(&self) -> &PathBuf {
        &self.rtp
    }

    /**
     * Get the commandline interface matches
     */
    pub fn cli(&self) -> &ArgMatches {
        &self.cli_matches
    }

    /**
     * Get the store object
     */
    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn editor(&self) -> Option<Command> {
        self.cli()
            .value_of("editor")
            .map(String::from)
            .or({
                match &self.configuration {
                    &Some(ref c) => c.editor().map(|s| s.clone()),
                    _ => None,
                }
            })
            .or(env::var("EDITOR").ok())
            .map(Command::new)
    }
}


