/*    Rudo is a program to get privilege access on unix system
 *    Copyright (C) 2021  Rémi Lauzier <remilauzier@protonmail.com>
 *
 *    This program is free software; you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation; either version 2 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License along
 *    with this program; if not, write to the Free Software Foundation, Inc.,
 *    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */
use std::error::Error;

use log::{info, LevelFilter};
#[cfg(all(target_os = "macos", feature = "macos"))]
use oslog::OsLogger;
#[cfg(target_os = "linux")]
use systemd::journal;

#[cfg(target_os = "linux")]
/// Function to decide the maximum level of logging that journald will receive with the user supply option
pub(crate) fn log_journald(debug: bool) -> Result<(), Box<dyn Error>> {
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
    } else {
        return Err(From::from("Error can't initialize logging with journald"));
    }
    return Ok(());
}

#[cfg(all(target_os = "macos", feature = "macos"))]
/// Function to decide the maximum level of logging that oslog server will receive with the user supply option
pub(crate) fn log_oslog(debug: bool) -> Result<(), Box<dyn Error>> {
    if debug {
        OsLogger::new("com.github.rudo")
            .level_filter(LevelFilter::Debug)
            .category_level_filter("Settings", LevelFilter::Debug)
            .init()?;
        info!("Starting Debug logs");
    } else {
        OsLogger::new("com.github.rudo")
            .level_filter(LevelFilter::Info)
            .category_level_filter("Settings", LevelFilter::Info)
            .init()?;
        info!("Starting logs");
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use super::log_journald;
    #[cfg(all(target_os = "macos", feature = "macos"))]
    use super::log_oslog;
    use super::Error;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_journald() -> Result<(), Box<dyn Error>> {
        log_journald(false)
    }

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn test_oslog() -> Result<(), Box<dyn Error>> {
        log_oslog(false)
    }
}
