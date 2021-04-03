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
/*! Rudo is a program that permit a system adminitrator
to authorized a user to have privilege access with a few verification
like group membership and validity of the account
*/
#![deny(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod auth;
mod command;
mod config;
mod session;
mod tty;
mod user;

use clap::ArgMatches;
use pam_client::Flag;
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

/// Run function of Rudo.
/// It take the result of the command-line interface to decide
/// if it most create a shell or to pass a command or invoc an editor
pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize configuration
    debug!("Start configuration initialization");
    let conf = config::init_conf()?;
    debug!("Configuration has been initialize");

    // Create the user data for later use
    debug!("User information extraction");
    let userdata = user::User::new();
    debug!("User extraction finish");

    // Extract the information from rudo.conf that is tie to the actual user
    debug!("Extraction of the vec of Userconf in rudo.conf");
    let userconf = config::extract_userconf(conf.user.clone(), &userdata.username)?;
    debug!("Extraction has been done");

    // Update configuration if necessary, as CLI as the priority
    debug!("Update configuration with CLI option");
    let userconf = config::UserConfig::update(userconf, &matches);
    debug!("Configuration has been update");

    // Update configuration if necessary, as CLI as the priority
    debug!("Update configuration with CLI option");
    let conf = config::Config::update(conf, &matches);
    debug!("Configuration has been update");

    // Get the uid and gid of the impersonate user for further use
    debug!("Extract uid and gid of the impersonate user");
    let impuser =
        users::get_user_by_name(&conf.rudo.impuser).expect("Please give rudo a real unix username");
    let impuser_uid = impuser.uid();
    let impuser_gid = impuser.primary_group_id();
    debug!("Extraction finish");

    // Greet the user if the conf said so
    if userconf.greeting {
        debug!("Start user greeting");
        println!("Hello {}!", userdata.username);
        debug!("User greeting finish");
    }

    // Authenticate the user with the list of authorized user and group
    debug!("Authenticate the user");
    auth::authentification(&userconf, &userdata)?;
    debug!("User has been authenticate");

    // Create the pam context and authenticate the user with pam
    debug!("Pam context initialization and identification of user");
    let mut context = auth::auth_pam(&conf, &userconf, &userdata)?;
    debug!("Pam context create and user authenticate");

    // Open session with pam credentials
    debug!("Session initialize with pam credential");
    let session = context.open_session(Flag::NONE)?;
    debug!("Session has been create");

    // Verify the option the user as pass and act accordingly
    if matches.is_present("command") {
        // Extract the command in two part. First the name of the program then it's arguments.
        debug!("Extracting the supply command for further use");
        let command: Vec<&str> = matches.values_of("command").unwrap().collect();
        let data = command::Command::new(command).unwrap();
        debug!("Extraction has finish");

        // Log the user and it's command for further audit by system adminitrator
        info!(
            "{} has been authorized. Command: {} {:?}",
            userdata.username, data.program, data.args
        );

        // Creation and ignition of the new command
        debug!("Start the supply command");
        let mut child = Command::new(data.program)
            .args(data.args)
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the command to finish or the program end before the command
        child.wait()?;
        debug!("End of the supply command");
    } else if matches.is_present("shell") {
        // Extraction of the shell environment variable
        debug!("Extracting shell environment variable");
        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));

        // Log the user and it's shell for further audit by system adminitrator
        info!("{} has been authorized to use {}", userdata.username, shell);

        // Creation and ignition of the new shell
        debug!("Starting the shell");
        let mut child = Command::new(shell)
            .arg("-l") // Login shell
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the shell to finish or the program end before the shell
        child.wait()?;
        debug!("End of the shell");
    } else if matches.is_present("edit") {
        // Extraction of the editor environment variable
        debug!("Extracting editor environment variable");
        let editor = env::var("EDITOR")?;

        // Extraction of the arguments and file path for the editor
        debug!("Extracting arguments and file path give to the editor");
        let arg = matches.value_of("edit").unwrap();

        // Log the user, it's editor and it's arguments for further audit by system adminitrator
        info!(
            "{} has been authorized to use {} {}",
            userdata.username, editor, arg
        );

        // Creation and ignition of the new editor
        debug!("Starting the editor");
        let mut child = Command::new(editor)
            .arg(arg)
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the editor to finish or the program will end before the editor
        child.wait()?;
        debug!("End of the editor");
    }

    Ok(())
}
