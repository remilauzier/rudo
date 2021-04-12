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
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
    ArgMatches,
};

/// Function to initialize the command-line interface with all it's option,
/// and pass these to the rest of the program
pub(crate) fn init_command_line() -> ArgMatches<'static> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp) // Show help by default
        .setting(AppSettings::AllowLeadingHyphen) // Authorize "-" in command
        .setting(AppSettings::TrailingVarArg) // Make Rudo don't care about other option after the command is pass
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .value_name("command")
                .help("Pass the command to execute")
                .conflicts_with("shell")
                .required_unless("shell")
                .conflicts_with("edit")
                .required_unless("edit")
                .index(1) // Be sure that the command is the first so we don't have to write "-c" to take a command
                .multiple(true) // To be able to have the command and it's list of argument
                .allow_hyphen_values(true) // Should authorize "-" in command
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("shell")
                .help("Initialize a privilege shell")
                .conflicts_with("command")
                .required_unless("command")
                .conflicts_with("edit")
                .required_unless("edit")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("user")
                .help("The user you want to impersonate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("greeting")
                .short("g")
                .long("greeting")
                .value_name("greeting")
                .help("Greeting user")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .value_name("debug")
                .help("Log debug messages")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .value_name("edit")
                .help("Edit a document with the editor of user")
                .conflicts_with("command")
                .required_unless("command")
                .conflicts_with("shell")
                .required_unless("shell")
                .takes_value(true),
        )
        .get_matches();
    matches
}
