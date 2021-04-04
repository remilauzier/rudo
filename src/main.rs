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
#![deny(
    clippy::all,
    clippy::wildcard_dependencies,
    clippy::missing_docs_in_private_items,
    clippy::cargo_common_metadata,
    clippy::create_dir,
    clippy::verbose_file_reads
)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

/// Module to authenticate the Unix user with the provide configuration
mod auth;
/// Module to instantiate the command-line interface and it's options
mod cli;
/// Module to extract the command and it's arguments when user provide one
mod command;
/// Module that manage the configuration file and it's options
mod config;
/// Module that manage the logs that Rudo create
mod journal;
/// Module that take care of running Rudo
mod run;
/// Module that manage the session and it's validity for the user
mod session;
/// Module to extract the name of the TTY and to verify it's existence
mod tty;
/// Module that create user information and all the function with it
mod user;

use std::error::Error;
use std::path::Path;

/// Define the path to journald file to verify it's existence
pub static JOURNALD_PATH: &str = "/run/systemd/journal/";
/// The amount of time the session stay valid
pub static DEFAULT_SESSION_TIMEOUT: u64 = 600;
/// The beginning of the path where the session token will be written
pub static SESSION_DIR: &str = "/run/rudo/";
/// The default path of the configuration file
pub static CONFIG_PATH: &str = "/etc/rudo.conf";

/// Main function of the program
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
