//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::io::Write;
use std::io::stderr;

use log::{Log, LogLevel, LogRecord, LogMetadata};

use ansi_term::Style;
use ansi_term::Colour;
use ansi_term::ANSIString;

/// Logger implementation for `log` crate.
pub struct ImagLogger {
    prefix: String,
    dbg_fileline: bool,
    lvl: LogLevel,
    color_enabled: bool,
}

impl ImagLogger {

    /// Create a new ImagLogger object with a certain level
    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            prefix: "[imag]".to_owned(),
            dbg_fileline: true,
            lvl: lvl,
            color_enabled: true
        }
    }

    /// Set debugging to include file and line
    pub fn with_dbg_file_and_line(mut self, b: bool) -> ImagLogger {
        self.dbg_fileline = b;
        self
    }

    /// Set debugging to include prefix
    pub fn with_prefix(mut self, pref: String) -> ImagLogger {
        self.prefix = pref;
        self
    }

    /// Set debugging to have color
    pub fn with_color(mut self, b: bool) -> ImagLogger {
        self.color_enabled = b;
        self
    }

    /// Helper function to colorize a string with a certain Style
    fn style_or_not(&self, c: Style, s: String) -> ANSIString {
        if self.color_enabled {
            c.paint(s)
        } else {
            ANSIString::from(s)
        }
    }

    /// Helper function to colorize a string with a certain Color
    fn color_or_not(&self, c: Colour, s: String) -> ANSIString {
        if self.color_enabled {
            c.paint(s)
        } else {
            ANSIString::from(s)
        }
    }

}

impl Log for ImagLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &LogRecord) {
        use ansi_term::Colour::Red;
        use ansi_term::Colour::Yellow;
        use ansi_term::Colour::Cyan;

        if self.enabled(record.metadata()) {
            // TODO: This is just simple logging. Maybe we can enhance this lateron
            let loc = record.location();
            match record.metadata().level() {
                LogLevel::Debug => {
                    let lvl  = self.color_or_not(Cyan, format!("{}", record.level()));
                    let args = self.color_or_not(Cyan, format!("{}", record.args()));
                    if self.dbg_fileline {
                        let file = self.color_or_not(Cyan, format!("{}", loc.file()));
                        let ln   = self.color_or_not(Cyan, format!("{}", loc.line()));

                        writeln!(stderr(), "{}[{: <5}][{}][{: >5}]: {}", self.prefix, lvl, file, ln, args).ok();
                    } else {
                        writeln!(stderr(), "{}[{: <5}]: {}", self.prefix, lvl, args).ok();
                    }
                },
                LogLevel::Warn | LogLevel::Error => {
                    let lvl  = self.style_or_not(Red.blink(), format!("{}", record.level()));
                    let args = self.color_or_not(Red, format!("{}", record.args()));

                    writeln!(stderr(), "{}[{: <5}]: {}", self.prefix, lvl, args).ok();
                },
                LogLevel::Info => {
                    let lvl  = self.color_or_not(Yellow, format!("{}", record.level()));
                    let args = self.color_or_not(Yellow, format!("{}", record.args()));

                    writeln!(stderr(), "{}[{: <5}]: {}", self.prefix, lvl, args).ok();
                },
                _ => {
                    writeln!(stderr(), "{}[{: <5}]: {}", self.prefix, record.level(), record.args()).ok();
                },
            }
        }
    }
}

