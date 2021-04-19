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
/*! Rudo is a program that permit a system administrator
to authorize a user to have privilege access with verification
like group membership and validity of the account
*/
#![deny(
    rustdoc,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    warnings,
    unused,
    missing_docs,
    unreachable_pub,
    macro_use_extern_crate,
    single_use_lifetimes,
    unused_lifetimes
)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::wildcard_dependencies,
    clippy::cargo_common_metadata,
    clippy::missing_docs_in_private_items,
    clippy::create_dir,
    clippy::verbose_file_reads,
    clippy::str_to_string,
    clippy::pattern_type_mismatch,
    clippy::string_add,
    clippy::string_to_string,
    clippy::use_debug,
    clippy::wrong_pub_self_convention,
    clippy::fallible_impl_from,
    clippy::needless_borrow,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::cognitive_complexity
)]

use std::error::Error;

/// Module to authenticate the Unix user with the provided configuration
mod auth;
/// Module to instantiate the command-line interface, and it's options
mod cli;
/// Module to extract the command, and it's arguments when user provide one
mod command;
/// Module that manage the configuration file, and it's options
mod config;
/// Module that manage the logs that Rudo create
mod journal;
/// Module that ask for the user password to authenticate him
mod pwd;
/// Module that take care of running Rudo
mod run;
/// Module that manage the session, and it's validity for the user
mod session;
/// Module to verify that the token path exist and return a bool wrap in a result
mod token;
/// Module to extract the name of the TTY and to verify its existence
mod tty;
/// Module that create user information and all the function with it
mod user;

/// The amount of time the session stay valid
pub(crate) static DEFAULT_SESSION_TIMEOUT: u64 = 600;
/// The beginning of the path where the session token will be written
pub(crate) static SESSION_DIR: &str = "/run/rudo/";
/// The default path of the configuration file
pub(crate) static CONFIG_PATH: &str = "/etc/rudo.conf";

/// Main function of the program
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CLI interface with clap
    let matches = cli::init_command_line();

    // Extract debug logging variable for further use
    let debug = matches.is_present("debug");

    #[cfg(feature = "journald")]
    // Use journald for logging
    journal::log_journald(debug)?;

    #[cfg(feature = "syslog3164")]
    // Use syslog for logging
    journal::log_syslog(debug)?;

    #[cfg(target_os = "macos")]
    // Use oslog for logging
    journal::log_oslog(debug)?;

    // Principal function of Rudo
    run::run(&matches)?;

    Ok(())
}
