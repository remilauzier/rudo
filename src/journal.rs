//    Rudo is a program to get privilege access on unix system
//    Copyright (C) 2021  Rémi Lauzier <remilauzier@protonmail.com>
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

#[cfg(features = "journald")]
use log::{LevelFilter, SetLoggerError};
#[cfg(features = "journald")]
use std::error::Error;

#[cfg(features = "syslogging")]
use log::{LevelFilter, SetLoggerError};
#[cfg(features = "syslogging")]
use std::error::Error;

#[cfg(features = "journald")]
use systemd::journal;

#[cfg(features = "syslogging")]
use syslog::{Facility, Formatter5424, Logger};

#[cfg(features = "macos")]
use oslog;

#[cfg(features = "journald")]
/// Function to decide the maximum level of logging that journald will receive with the user supply option
pub fn log_journald(debug: bool) -> Result<(), Box<dyn Error>> {
    // Initialize Logs with journald
    if let Ok(()) = journal::JournalLog::init() {
        // Determine the maximum level of log the user want
        if debug {
            log::set_max_level(LevelFilter::Debug);
            info!("Starting Debug logs");
        } else {
            log::set_max_level(LevelFilter::Info);
            info!("Starting logs");
        }
    } else if journal::JournalLog::init().is_err() {
        return Err(From::from("Error can't initialize logging with journald"));
    }
    Ok(())
}

#[cfg(features = "syslogging")]
/// Function to decide the maximum level of logging that syslog server will receive with the user supply option
pub fn log_syslog(debug: bool) -> Result<(), Box<dyn Error>> {
    let formatter = Formatter5424 {
        facility: Facility::LOG_AUTH,
        hostname: None,
        process: "rudo".into(),
        pid: 0,
    };
    let logger = syslog::unix(&formatter)?;
    if !debug {
        log::set_boxed_logger(Box::new(Logger::new(logger, formatter)))
            .map(|()| log::set_max_level(LevelFilter::Info));
    } else {
        log::set_boxed_logger(Box::new(Logger::new(logger, formatter)))
            .map(|()| log::set_max_level(LevelFilter::Debug));
    }
    Ok(())
}

#[cfg(features = "macos")]
pub fn log_oslog(debug: bool) -> Result<(), Box<dyn Error>> {
    if debug {
        OsLogger::new("com.github.rudo")
            .level_filter(LevelFilter::Debug)
            .category_level_filter("Settings", LevelFilter::Debug)
            .init()?;
    } else {
        OsLogger::new("com.github.rudo")
            .level_filter(LevelFilter::Info)
            .category_level_filter("Settings", LevelFilter::Info)
            .init()?;
    }
}

#[cfg(test)]
mod tests {
    #[cfg(features = "journald")]
    use super::*;
    #[cfg(features = "syslogging")]
    use super::*;
    #[cfg(features = "macos")]
    use super::*;

    #[cfg(features = "journald")]
    #[test]
    fn test_journald() -> Result<(), Box<dyn Error>> {
        log_journald(false)
    }
    #[cfg(features = "syslogging")]
    #[test]
    fn test_syslog() -> Result<(), Box<dyn Error>> {
        log_syslog(false)
    }
    #[cfg(features = "macos")]
    #[test]
    fn test_oslog() -> Result<(), Box<dyn Error>> {
        log_oslog(false)?
    }
}
