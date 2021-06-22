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
like group membership
*/
#![deny(
//rustdoc::all,                  // rustdoc is for verifying the validity of some part of the documentation
future_incompatible,           // future_incompatible is to ensure to be ready for future version of rust
nonstandard_style,             // nonstandard_style is for verifying that Rudo respect convention. Rule LANG-NAMING anssi
rust_2018_compatibility,       // rust_2018_compatibility is for forcing the 2018 convention as some small thing can be ignored by compiler
rust_2018_idioms,              // rust_2018_idioms is for forcing the 2018 convention as some small thing can be ignored by compiler
warnings,                      // Switch all warning to deny, to force better code by default
unused,                        // Deny all unused code
unreachable_pub,               // unreachable_pub is for verifying that public code get pub(crate) as Rudo is a binary only
macro_use_extern_crate,        // Prefer to declare in each file the macro that must be use instead of #[macro_use]
single_use_lifetimes,          // Prefer <'_> for single_use_lifetimes instead of <'a>
unused_lifetimes,              // Detect unused_lifetimes and remove them
unused_qualifications,         // Detect unnecessary qualifications for simpler code style
//missing_crate_level_docs,      // Verify that the crate always have a documentation explaining its utility
missing_docs,                  // Ensure documentation is present
non_ascii_idents,              // Ensure variable have only ascii character for security reason and clarity of code
trivial_casts,                 // Ensure cast are not misused and prefer coercion instead
trivial_numeric_casts,         // Ensure cast are not misused and prefer coercion instead
//unaligned_references,          // Force alignment of reference to avoid Undefined Behavior in unsafe function
//unused_crate_dependencies,     // Ensure no unused crate get compiled or used
unused_import_braces,          // Ensure brace are use only for multiple items only
variant_size_differences,      // Detect if some enum contain variable of different size that could consume more memory
//unsafe_op_in_unsafe_fn         // Force unsafe block in unsafe function
)]
#![deny(
clippy::all,                           // Deny everything that is in the correctness, performance, style and complexity categories to be more strict in code quality
clippy::pedantic,                      // Deny everything in the pedantic categories to be more strict on code quality
clippy::wildcard_dependencies,         // Refuse to work with * in a version of dependency since Rudo can't work with every possible version
clippy::cargo_common_metadata,         // Verify that cargo.toml as a minimum of metadata to ensure discoverability. C-METADATA of rust api guideline
clippy::missing_docs_in_private_items, // Verify that private items have documentation to help others understand Rudo functionality
clippy::create_dir,                    // Ensure to use create_dir_all since we must create a long chain of directory in Rudo for the token
clippy::verbose_file_reads,            // Use read_to_string instead of open and read to reduce code size. 1 lines instead of 3.
clippy::str_to_string,                 // Prefer to_owned instead of to_string for better clarity since other type can be to_string
clippy::string_add,                    // Prefer push_str instead of + for more clarity
clippy::string_to_string,              // Prefer clone instead of to_string on string for better clarity
clippy::use_debug,                     // Prefer not to use {:?} in production code, but it doesn't catch them in log macro for now. Rust-1.51
clippy::needless_borrow,               // Removed needless borrow in code for better clarity. Nursery
clippy::use_self,                      // Use self when its possible instead of given the name of a struct or other type everywhere. Nursery
clippy::useless_let_if_seq,            // Prefer idiomatic rust for clarity in code . Nursery
clippy::expect_used,                   // Refuse .expect() since Rudo is production code. Rule LANG-NOPANIC anssi
clippy::filetype_is_file,              // Prefer !FileType::is_dir() instead of is_file() since it can have problem with special file or symlink
clippy::get_unwrap,                    // Prefer get() with good error management instead of .get(0).unwrap to avoid panic at runtime. Rule LANG-ARRINDEXING anssi
clippy::unwrap_in_result,              // Refuse to change a recoverable error in a non-recoverable one. Rule LANG-NOPANIC anssi
clippy::unwrap_used,                   // Refuse .unwrap() since Rudo is production code. Rule LANG-NOPANIC anssi
clippy::let_underscore_must_use,       // It’s better to explicitly handle the value of a #[must_use] expr
clippy::cognitive_complexity,          // Verify the complexity of a function to not be further than 25. Can be change later if necessary. Nursery
clippy::else_if_without_else,          // Follow MISRA-C:2004 Rule 14.10 and be defensive in the code. Take care of 0.01% chance an error happens.
clippy::mem_forget,                    // Don't use mem_forget since it can cause memory leaks. Rule MEM-FORGET and Recommendation MEM-FORGET-LINT anssi
clippy::shadow_reuse,                  // Ensure the code is easy to follow by refusing meaningless shadowing
clippy::shadow_same,                   // Ensure the code is easy to follow by refusing meaningless shadowing
clippy::as_conversions,                // Ensure no lossy conversion are performed silently
clippy::exit,                          // Ensure no exit() is performed on code. Always use an error that go to the main function.
clippy::multiple_inherent_impl,        // Ensure to have only one impl for each struct
clippy::integer_arithmetic,            // Prefer safer method to avoid overflow like saturating_add(). Rule LANG-ARITH anssi
clippy::indexing_slicing,              // Prefer get() method with good error management to avoid panic in runtime. Rule LANG-ARRINDEXING anssi
clippy::semicolon_if_nothing_returned, // Prefer to finish function with a semicolon even if the function is one line
clippy::if_then_some_else_none,        // Prefer less redundant code
clippy::unnecessary_self_imports       // Prefer clearer code
)]
// Authorized redundant else to conform to MISRA-C:2004 Rule 14.10 and to not conflict with clippy::else_if_without_else
#![allow(clippy::redundant_else)]

use std::error::Error;

/// Module to authenticate the Unix user with the provided configuration
mod auth;
/// Module to instantiate the command-line interface, and it's options
mod cli;
/// Module to extract the command, and it's arguments when user provide one
mod cmd;
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
/// Generics Function that can be reused by others modules
mod utils;

/// The amount of time the session stay valid
pub(crate) static DEFAULT_SESSION_TIMEOUT: u64 = 600;
/// The beginning of the path where the session token will be written
pub(crate) static SESSION_PATH: &str = "/run/rudo/";
/// The default path of the configuration file
pub(crate) static CONFIG_PATH: &str = "/etc/rudo.conf";

/// Main function of the program
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CLI interface with clap
    let matches = cli::init_command_line();

    // Extract debug logging variable for further use
    let debug = matches.is_present("debug");

    #[cfg(all(target_os = "linux", feature = "journald"))]
    // Use journald for logging
    journal::log_journald(debug)?;

    #[cfg(all(target_os = "macos", feature = "macos"))]
    // Use oslog for logging
    journal::log_oslog(debug)?;

    // Principal function of Rudo
    run::run(&matches)?;

    Ok(())
}
