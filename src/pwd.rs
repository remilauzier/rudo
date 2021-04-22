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

use log::{debug, error};
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};

/// `Password_input` is a function that ask the user for their password.
/// Pam validates the password
pub(crate) fn password_input(
    password: bool,
    context: &mut Context<Conversation>,
) -> Result<(), Box<dyn Error>> {
    // Don't ask for password if false in the configuration
    if password {
        // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
        debug!("Password will be asked a maximum of 3 time to the user");
        let mut count: u8 = 0;
        while count < 3 {
            if let Ok(()) = context.authenticate(Flag::DISALLOW_NULL_AUTHTOK) {
                break;
            }
            error!("Password was incorrect! Will be report to administrator!");
            eprintln!("Password was incorrect! Will be report to administrator!");
            count += 1
        }
        if count == 3 {
            return Err(From::from("You have made three mistake! Rudo Out!"));
        }
    }
    Ok(())
}
