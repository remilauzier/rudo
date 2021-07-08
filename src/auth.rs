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

use log::{debug, info};
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};

use crate::config;
use crate::pwd;
use crate::session;
use crate::token;
use crate::tty;
use crate::user;
use crate::SESSION_PATH;

/// Function to verify if the user is authorized before using Pam
pub(crate) fn authentification(
    userconf: &config::UserConf,
    userdata: &user::User,
) -> Result<(), Box<dyn Error>> {
    // Verify that the user is authorized to run Rudo
    debug!("Starting verification of {}", &userconf.username);
    userdata.verify_user(&userconf.username)?;

    // Verify that the user is a member of the privilege group for privilege access
    debug!(
        "User was approved, starting group verification of {}",
        userconf.group
    );
    userdata.verify_group(&userconf.group)?;

    Ok(())
}

/// Function to verify that the user is authorized to run Rudo with Pam and if a precedent session is valid
pub(crate) fn authentification_pam(
    conf: &config::Config,
    userconf: &config::UserConf,
    userdata: &user::User,
) -> Result<Context<Conversation>, Box<dyn Error>> {
    // Create the Pam context
    debug!("Creating Pam context for Rudo");
    let mut context = Context::new(
        "rudo",
        Some(&userdata.username), // Give the name of the actual user
        Conversation::new(),
    )?;

    // Extract the Terminal name and identifier
    let tty = tty::Terminal::new()?;

    debug!("TTY name is: {}", tty.terminal_name);
    debug!("Terminal UUID is {}", tty.terminal_uuid);

    // Create the token path with the base, the username and the TTY name
    let token_path = format!(
        "{}{}{}",
        SESSION_PATH, &userdata.username, tty.terminal_name
    );
    debug!("token_path has been created: {}", token_path);

    // Verify that token_path is valid and that the session is not expired,
    // then pass the result.
    debug!("Verifying token_path validity and extracting result");
    let result = token::verify_path(&token_path, &tty)?;

    debug!("Asking for password if token is invalid or non-existent");
    if !result {
        info!(
            "{} demand authorization to use Rudo, password will be asked",
            userdata.username
        );
        // Password will be asked to validate the authorization
        pwd::password_input(userconf.password, &mut context)?;
        info!(
            "{} has given is password that was validated by Pam",
            userdata.username
        );

        // Validate the account (is not locked, expired, etc.)
        debug!("Validate the account of {}", userdata.username);
        context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;

        // Create the run directory where the token will be written
        debug!("Creating the directory of the token in /run");
        session::create_dir_run(&userdata.username)?;

        // Create token with all the necessary information
        let token = session::Token::new(&tty.terminal_name, &tty.terminal_uuid);
        debug!(
            "Token was created for {} with UUID: {}",
            tty.terminal_name, tty.terminal_uuid
        );

        // Write the token to file
        debug!("Token will be written to {}", token_path);
        token?.create_token_file(&userdata.username)?;
    }

    // Change the user to have privilege access accordingly to the configuration of the user
    context.set_user(Some(conf.rudo.impuser.as_str()))?;
    info!("User was change to: {}", conf.rudo.impuser);

    Ok(context)
}
