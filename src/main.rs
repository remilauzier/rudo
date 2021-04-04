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
/*! Rudo is a program that permit a system administrator
to authorized a user to have privilege access with a few verification
like group membership and validity of the account
*/
#![deny(missing_docs, rustdoc, warnings)]
#![deny(clippy::all, clippy::clippy::wildcard_dependencies)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod auth;
mod cli;
mod command;
mod config;
mod journal;
mod run;
mod session;
mod tty;
mod user;

use std::error::Error;
use std::path::Path;

static JOURNALD_PATH: &str = "/run/systemd/journal/";

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CLI interface with clap
    let matches = cli::init_cli();
    // Extract debug logging variable for further use
    let debug = matches.is_present("debug");

    // Verify that journald file exist
    if Path::new(JOURNALD_PATH).exists() {
        // Use journald for logging
        journal::log_journald(debug)?;
    } else {
        eprintln!("Journald file not found");
    }

    debug!("Begin of run function");
    run::run(matches)?;
    debug!("End of run function");

    Ok(())
}
