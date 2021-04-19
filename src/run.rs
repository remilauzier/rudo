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
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

use clap::ArgMatches;
use log::{debug, info};
use pam_client::conv_cli::Conversation;
use pam_client::{Flag, Session};

use crate::auth;
use crate::command;
use crate::config;
use crate::user;

/// Run function of Rudo.
/// It takes the result of the command-line interface to decide
/// if it will create a login shell or to pass a command or to invoke the editor
pub(crate) fn run(matches: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    // Initialize configuration
    debug!("Starting configuration initialization");
    let conf = config::init_conf()?;

    // Create the user data for later use
    debug!("Starting extraction of User information");
    let userdata = user::User::new();

    // Extract the information from rudo.conf that is tie to the actual user
    debug!(
        "Starting extraction of the vector of UserConf tie to {} in rudo.conf",
        &userdata.username
    );
    let userconf = config::extract_userconf(conf.user.clone(), &userdata.username);

    // Update configuration if necessary, as CLI as the priority
    debug!("Update configuration with CLI option as it as the priority");
    let userconf = config::UserConf::update(userconf, matches);
    let conf = config::Config::update(conf, matches);

    // Get the UID and GID of the impersonated user for further use
    debug!(
        "Extract UID and GID of the impersonated user {}",
        &conf.rudo.impuser
    );
    let impuser =
        users::get_user_by_name(&conf.rudo.impuser).expect("Please give rudo a real unix username");
    let impuser_uid = impuser.uid();
    let impuser_group_id = impuser.primary_group_id();

    // Greet the user if the configuration said so
    if userconf.greeting {
        debug!("Start user greeting messages and disclaimer");
        println!(
            "Hello {}! Think carefully before using Rudo.",
            userdata.username
        );
    }

    // Authenticate the user with the list of authorized user and group
    debug!(
        "Authenticate {} with the list in rudo.conf",
        userdata.username
    );
    auth::authentification(&userconf, &userdata)?;

    // Create the Pam context and authenticate the user with Pam
    debug!(
        "Pam context initialization and identification of {}",
        userdata.username
    );
    let mut context = auth::authentification_pam(&conf, &userconf, &userdata)?;

    // Open session with Pam credentials
    debug!("Session initialize with Pam credential");
    let session = context.open_session(Flag::NONE)?;

    // Run the command the user as choose
    debug!("Run the command {} as choose", userdata.username);
    run_command(matches, &session, impuser_uid, impuser_group_id, &userdata)?;

    Ok(())
}
/// `run_command` is a function that run the precise command the user demand
fn run_command(
    matches: &ArgMatches<'_>,
    session: &Session<'_, Conversation>,
    uid: u32,
    group_id: u32,
    userdata: &user::User,
) -> Result<(), Box<dyn Error>> {
    // Verify the option the user as pass and act accordingly
    if matches.is_present("command") {
        // Extract the command in two part. First the name of the program then it's arguments.
        debug!("Extracting the supply command for further use");
        let command: Vec<&str> = matches.values_of("command").unwrap().collect();
        let data = command::Command::new(command).unwrap();

        // Log the user, and it's command for further audit by system administrator
        info!(
            "{} has been authorized. Command: {} {:?}",
            userdata.username, data.program, data.args
        );

        // Creation and ignition of the new command
        debug!("Start the supply command");
        let mut child = Command::new(data.program)
            .args(data.args)
            .envs(session.envlist().iter_tuples()) // Pass the Pam session to the new process
            .uid(uid) // Necessary to have full access
            .gid(group_id) // Necessary to have full access
            .spawn()?;

        // Wait for the command to finish, or the program end before the command
        child.wait()?;
    } else if matches.is_present("shell") {
        // Extraction of the shell environment variable
        debug!("Extracting shell environment variable");
        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));

        // Log the user, and it's shell for further audit by system administrator
        info!("{} has been authorized to use {}", userdata.username, shell);

        // Creation and ignition of the new shell
        debug!("Starting {}", shell);
        let mut child = Command::new(shell)
            .arg("-l") // Login shell
            .envs(session.envlist().iter_tuples()) // Pass the Pam session to the new process
            .uid(uid) // Necessary to have full access
            .gid(group_id) // Necessary to have full access
            .spawn()?;

        // Wait for the shell to finish, or the program end before the shell
        child.wait()?;
    } else if matches.is_present("edit") {
        // Extraction of the editor environment variable
        debug!("Extracting editor environment variable");
        let editor = env::var("EDITOR")?;

        // Extraction of the arguments and file path for the editor
        debug!("Extracting arguments and file path give to the editor");
        let arg = matches.value_of("edit").unwrap();

        // Log the user, it's editor, and it's arguments for further audit by system administrator
        info!(
            "{} has been authorized to use {} {}",
            userdata.username, editor, arg
        );

        // Creation and ignition of the new editor
        debug!("Starting {}", editor);
        let mut child = Command::new(editor)
            .arg(arg)
            .envs(session.envlist().iter_tuples()) // Pass the Pam session to the new process
            .uid(uid) // Necessary to have full access
            .gid(group_id) // Necessary to have full access
            .spawn()?;

        // Wait for the editor to finish, or the program will end before the editor
        child.wait()?;
    }
    Ok(())
}
