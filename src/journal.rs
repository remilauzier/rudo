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
use log::LevelFilter;
use std::error::Error;
use systemd::journal;

/// Function to decide of the maximum level of logging with the user supply option
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
