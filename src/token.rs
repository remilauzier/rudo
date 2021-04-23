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
use std::fs;
use std::path::Path;

use log::{debug, error};

use crate::session;

/// `verify_path` analyze if the token exist, and it's valid, then it returns a bool for the result.
pub(crate) fn verify_path(
    token_path_str: &str,
    tty_name: &str,
    tty_uuid: &str,
) -> Result<bool, Box<dyn Error>> {
    let token_path = Path::new(&token_path_str);

    // Verify if a token exist and act accordingly
    debug!("Verifying if token_path exist and is a directory");
    if token_path.exists() && token_path.is_dir() {
        // Erase the path if it's a directory
        error!("token_path is a directory and will be erased");
        fs::remove_dir(token_path)?;
        return Ok(false);
    } else if token_path.exists() && token_path.is_file() {
        // Read the token file and return false if invalid or expired
        debug!("Token will be read from file and validate");
        let token = session::read_token_file(token_path_str);
        if token.is_err() {
            debug!("Token was invalid");
            return Ok(false);
        }
        if token?.verify_token(tty_name, tty_uuid).is_err() {
            debug!("Token was invalid");
            return Ok(false);
        }
        debug!("Token was valid");
        return Ok(true);
    } else {
        debug!("Token was non-existent");
        return Ok(false);
    }
}

#[cfg(test)]
mod tests {
    use super::{verify_path, Error};

    #[test]
    fn test_verify_path_non_existent() -> Result<(), Box<dyn Error>> {
        let result = verify_path("/run/rudo/pts/0", "pts/0/", "964045904534593458953")?;
        if result {
            Err(From::from("Test failed: the path should not be valid"))
        } else {
            Ok(())
        }
    }
}
