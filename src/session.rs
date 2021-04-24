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
use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::DEFAULT_SESSION_TIMEOUT;
use crate::SESSION_DIR;

/// Create a structure to contain the UUID of the terminal, and the timestamp to determine
/// if the session is valid for later use
#[derive(Serialize, Deserialize)]
pub(crate) struct Token {
    /// Name of the TTY
    tty_name: String,
    /// UUID of the TTY
    tty_uuid: String,
    /// The timestamp determine at the creation of the token
    timestamp: SystemTime,
    /// The timestamp plus the DEFAULT_SESSION_TIMEOUT to determine the maximum validity of the session
    final_timestamp: SystemTime,
}

impl Token {
    /// Create the token and all it's parameter
    pub(crate) fn new(tty_name: &str, tty_uuid: &str) -> Result<Self, Box<dyn Error>> {
        debug!("Create the timestamp of the token");
        let timestamp = SystemTime::now();
        // Create the timestamp where the session become invalid
        debug!("Create the final timestamp to determine the maximum validity of the session");
        let duration = std::time::Duration::from_secs(DEFAULT_SESSION_TIMEOUT);
        let final_timestamp = match timestamp.checked_add(duration) {
            Some(time) => time,
            None => return Err(From::from("Couldn't create final timestamp")),
        };
        return Ok(Self {
            tty_name: String::from(tty_name),
            tty_uuid: String::from(tty_uuid),
            timestamp,
            final_timestamp,
        });
    }
    /// Create the file that will contain the token if it doesn't exist
    pub(crate) fn create_token_file(&self, username: &str) -> Result<(), Box<dyn Error>> {
        // Create the path of the file with the name of the program, the username to distinguish user
        // and the name of TTY to let user have multiple session, on multiple terminal
        let token_path_string = format!("{}{}{}", SESSION_DIR, username, self.tty_name);
        let token_path = Path::new(&token_path_string);
        debug!(
            "token_path has been created, will verify if it exists : {}",
            token_path_string
        );

        // Verify the existence of the path to act accordingly
        if token_path.exists() {
            // Erase the ancient file and create new one
            debug!("token_path exist will be erased and replace");
            fs::remove_file(token_path)?;

            // Put the token data in a string of YAML
            debug!("Put Token in a string");
            let token_file = serde_yaml::to_string(&self)?;

            // Creating the file for the token
            debug!("Creating the token file");
            let mut file = File::create(token_path)?;

            // Write the token data in the file
            debug!("Write the string in the file");
            file.write_all(token_file.as_bytes())?;

            // Sync data to be sure everything is writing on drive
            debug!("Syncing data on drive");
            file.sync_all()?;

            // Put file permission to 600 to restraint access
            debug!("Set file permission to 600 to restraint access");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
        } else {
            debug!("token_path doesn't exist, will create it");

            let path = match token_path.parent() {
                Some(path) => path,
                None => return Err(From::from("Error in token_path! Unable to give the parent")),
            };
            let path_str = match path.to_str() {
                Some(data) => data,
                None => return Err(From::from("Couldn't convert a path to str!")),
            };

            // Create the directory with mode 600 to restraint access
            debug!(
                "Create directory: {} with mode 600 to restraint access",
                path_str
            );
            DirBuilder::new().mode(0o600).recursive(true).create(path)?;

            // Put the token data in a string of YAML
            debug!("Put Token in a string");
            let token_file = serde_yaml::to_string(&self)?;

            // Creating the file for the token
            debug!("creating the token file");
            let mut file = File::create(token_path)?;

            // Write the token data in the file
            debug!("write the string in the file");
            file.write_all(token_file.as_bytes())?;

            // Sync data to be sure everything is writing on drive
            debug!("Syncing data on drive");
            file.sync_all()?;

            // Put file permission to 600 to restraint access
            debug!("Set file permission to 600 to restraint access");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
        }
        return Ok(());
    }
    /// Verify that the token is valid to decide if we must reuse the session or not
    pub(crate) fn verify_token(
        &self,
        tty_name: &str,
        tty_uuid: &str,
    ) -> Result<(), Box<dyn Error>> {
        let clock = SystemTime::now();
        if self.final_timestamp <= clock {
            debug!("Session has expired");
            return Err(From::from("Session has expired"));
        } else if self.tty_name == tty_name
            && self.tty_uuid == tty_uuid
            && self.final_timestamp > clock
        {
            debug!("Session is valid, will reuse it");
            return Ok(());
        } else {
            debug!("Not the same session");
            return Err(From::from("Not the same session"));
        }
    }
}

/// Create the full path of the directory containing the token file
pub(crate) fn create_dir_run(username: &str) -> Result<(), Box<dyn Error>> {
    // Create the first part of the path
    let run_path = Path::new(SESSION_DIR);

    // Verify that the first part of the path exist first
    debug!("Verify that {} exist", SESSION_DIR);
    if !run_path.exists() {
        info!(
            "{} doesn't exist, creating it with mode 600 to restraint access",
            SESSION_DIR
        );
        // Create the path with permissions of 600 to restraint access
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(SESSION_DIR)?;
    }
    // Extract permissions from the directory for further use
    let metadata = fs::metadata(SESSION_DIR)?;
    let mut perms = metadata.permissions();

    // Verify the permissions of the directory and act accordingly
    debug!("Verifying permission on {}", SESSION_DIR);
    if perms.mode() != 0o600 {
        warn!("Permissions are incorrect and will be adjusted to 600");
        perms.set_mode(0o600);
        fs::set_permissions(SESSION_DIR, perms)?;
    }

    // Create the second part of the path for further use
    let user_path_string = format!("{}{}/", SESSION_DIR, username);
    let user_path = Path::new(&user_path_string);

    // Verifying if the path exist and act accordingly
    debug!("Verifying that user_path exist: {}", user_path_string);
    if !user_path.exists() {
        info!(
            "{} doesn't exist, creating it with mode 600 to restraint access",
            user_path_string
        );
        // Create the path with permissions of 600 to restraint access
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(user_path)?;
    } else if user_path.is_file() {
        let err = format!("Error: {} is not a directory", user_path_string);
        error!("{}", err);
        return Err(From::from(err));
    } else {
        // Extract permissions from the directory for further use
        let metadata = fs::metadata(user_path)?;
        let mut perms = metadata.permissions();
        // Verifying if the permission of the directory and act accordingly
        debug!("Verifying {} permissions", user_path_string);
        if perms.mode() != 0o600 {
            warn!("Permissions are incorrect, adjusting it to 600 to restraint access");
            perms.set_mode(0o600);
            fs::set_permissions(user_path, perms)?;
        }
    }
    return Ok(());
}

/// Function to extract the token from its file with `serde_yaml`
pub(crate) fn read_token_file(token_path: &str) -> Result<Token, Box<dyn Error>> {
    // Open the file and extract its contents in a buffer
    debug!(
        "Open the file at {} and put it's content in a buffer",
        token_path
    );
    let buffer = fs::read_to_string(token_path)?;

    // Transform the buffer to the token structure
    debug!("Transform the buffer to the token structure");
    let token: Token = serde_yaml::from_str(&buffer)?;
    return Ok(token);
}

#[cfg(test)]
mod tests {
    use super::{Error, Token, DEFAULT_SESSION_TIMEOUT};

    #[test]
    fn test_timestamp() -> Result<(), Box<dyn Error>> {
        let token = Token::new("name", "1234")?;
        let duration = std::time::Duration::from_secs(DEFAULT_SESSION_TIMEOUT);
        if token.final_timestamp - duration == token.timestamp {
            return Ok(());
        } else {
            return Err(From::from("Test failed: timestamp creation got wrong"));
        }
    }
}
