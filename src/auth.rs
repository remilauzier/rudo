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
use crate::config;
use crate::session;
use crate::tty;
use crate::user;

use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
use std::error::Error;
use std::fs;
use std::path::Path;

/// The beginning of the path of the token file
static SESSION_DIR: &str = "/run/rudo/";

/// Function to verify if the user is authorized before using Pam
pub fn authentification(
    userconf: &config::UserConfig,
    userdata: &user::User,
) -> Result<(), Box<dyn Error>> {
    // Verify that the user is authorize to run Rudo
    debug!("User verification begin");
    userdata.verify_user(&userconf.username)?;
    debug!("User verification finish");

    // Verify that the user is a member of the privilege group for privilege access
    debug!("Group verification begin");
    userdata.verify_group(userconf.group.as_str())?;
    debug!("Group verification finish");
    Ok(())
}

/// Function to verify that the user is authorized to run Rudo with Pam and if a precedent session is valid
pub fn auth_pam(
    conf: &config::Config,
    userconf: &config::UserConfig,
    userdata: &user::User,
) -> Result<Context<Conversation>, Box<dyn Error>> {
    // Create the Pam context
    debug!("Pam context creation");
    let mut context = Context::new(
        "rudo",
        Some(userdata.username.as_str()), // Give the name of the actual user
        Conversation::new(),
    )?;
    debug!("Pam context create");

    // Extract the ttyname with libc
    debug!("extract tty name");
    let tty_name = tty::get_tty_name()?;
    debug!("TTY name has been extract: {}", tty_name);

    // Create the token path with the base, the username and the ttyname
    debug!("token_path will be create");
    let token_path = format!("{}{}{}", SESSION_DIR, &userdata.username, tty_name);
    let token_path = Path::new(&token_path);
    debug!("token_path has been create: {:?}", token_path);

    // Verify if an existing token exist and act accordingly
    let mut result = false;
    debug!("Verifying if token_path exist");
    if token_path.exists() && token_path.is_file() {
        // extract the UUID of the terminal for later use
        debug!("Will determine UUID of the terminal");
        let tty_uuid = tty::tty_uuid()?;
        debug!("Terminal UUID is {}", tty_uuid);

        // Read the token file
        debug!("Token will be read from file");
        let token = session::read_token_file(token_path.to_str().unwrap());

        // Verify if the token was valid and act accordingly
        if let Ok(token) = token {
            debug!("Token has been read from file");
            result = match token.verify_token(&tty_name, tty_uuid) {
                Ok(()) => true,
                Err(err) => {
                    info!("{}", err);
                    false
                }
            }
        }
    } else if token_path.exists() && token_path.is_dir() {
        // Erase the path if it's a directory
        error!("token_path is a directory and will be erase");
        fs::remove_dir(token_path)?;
    }

    // If the token was invalid ask for the password
    debug!("Asking for password if token is invalid");
    if !result {
        info!("{} demand authorization to use Rudo", userdata.username);
        // Don't ask for password if false in the configuration
        if userconf.password {
            // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
            debug!("Password will be ask");
            let mut count = 0;
            while count < 3 {
                match context.authenticate(Flag::DISALLOW_NULL_AUTHTOK) {
                    Ok(()) => break,
                    Err(err) => {
                        error!("Password was incorrect");
                        eprintln!("Error: {}", err);
                        count += 1
                    }
                }
            }
            info!("Password was given and validate");
        }
        // Validate the account (is not locked, expired, etc.)
        debug!("Validate the account");
        context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;
        debug!("Account validate");

        // Create the run directory where the token will be write
        debug!("Creating run directory");
        session::create_dir_run(&userdata.username)?;
        debug!("Run directory has been create");

        // Extract the ttyname with libc
        debug!("Getting TTY name");
        let tty_name = tty::get_tty_name()?;
        debug!("TTY name was get: {}", tty_name);

        // Extract the UUID of the terminal
        debug!("Will determine UUID of the terminal");
        let tty_uuid = tty::tty_uuid()?;
        debug!("Terminal UUID is {}", tty_uuid);

        // Create token with all the necessary information
        debug!("Creating a new Token");
        let token = session::Token::new(tty_name, tty_uuid);
        debug!("Token was create");

        // Write the token to file
        debug!("Token will be writing to file");
        token.create_token_file(&userdata.username)?;
        debug!("Token was writing to file");
    }

    // Change the user to have privilege access accordingly to the configuration of the user
    debug!("Change the user as demand");
    context.set_user(Some(conf.rudo.impuser.as_str()))?;
    info!("User change to: {}", conf.rudo.impuser);

    Ok(context)
}
