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
use std::ffi::CStr;

use libc::{isatty, ttyname};
use log::{debug, error};

/// `Terminal` is a struct that contain the name and the identifier of a terminal
pub(crate) struct Terminal {
    /// Name of the terminal
    pub(crate) terminal_name: String,
    /// Identifier of the terminal
    pub(crate) terminal_uuid: String,
}

impl Terminal {
    /// Create a new instance of the terminal structure to have its name and identifier
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        let terminal_name = get_tty_name()?;
        let terminal_uuid = terminal_uuid()?;

        return Ok(Self {
            terminal_name,
            terminal_uuid,
        });
    }
}

/// Safe wrapper to get the name of the current TTY
/// and return it as a Rust string for further use
fn get_tty_name() -> Result<String, Box<dyn Error>> {
    unsafe {
        // Verify that we are indeed in a terminal
        if isatty(0) == 0 {
            error!("Rudo must be called from a terminal!");
            return Err(From::from("Rudo must be called from a terminal!"));
        }
        // Transform the File Descriptor to a c_char
        let ttyname_c = ttyname(0);

        // Verify that there is indeed a c_char
        if ttyname_c.is_null() {
            error!("Couldn't transform File Descriptor to c_char for ttyname");
            return Err(From::from(
                "Couldn't transform File Descriptor to c_char for ttyname",
            ));
        }
        // Transform the c_char to a rust string
        let ttyname_rust = CStr::from_ptr(ttyname_c).to_string_lossy().into_owned();
        debug!("Terminal: {} is use", ttyname_rust);
        return Ok(ttyname_rust);
    }
}

/// `terminal_uuid` is a function to determine the identity of the use terminal across time
/// with different environment variable.
/// WINDOWID is the least trust because of is small size and don't change for different tabs.
/// It only changes the last five number most of the time,
/// But it is used by st, xterm, sakura, kitty, xfce terminal, mate terminal and terminology.
/// Rox terminal use a value that change only the last six number but change for tabs.
/// Qterminal is insecure as it put 0 in WINDOWID and Rudo will refuse to consider it.
/// Guake, lxterminal, elementary terminal and deepin terminal as no UUID to use for now.
fn terminal_uuid() -> Result<String, Box<dyn Error>> {
    if env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        let uuid = env::var("GNOME_TERMINAL_SCREEN")?;
        debug!("GNOME_TERMINAL_SCREEN: {}", uuid);
        return Ok(uuid);
    } else if env::var("SHELL_SESSION_ID").is_ok() {
        let uuid = env::var("SHELL_SESSION_ID")?;
        debug!("SHELL_SESSION_ID: {}", uuid);
        return Ok(uuid);
    } else if env::var("TERMINATOR_UUID").is_ok() {
        let uuid = env::var("TERMINATOR_UUID")?;
        debug!("TERMINATOR_UUID: {}", uuid);
        return Ok(uuid);
    } else if env::var("TILIX_ID").is_ok() {
        let uuid = env::var("TILIX_ID")?;
        debug!("TILIX_ID: {}", uuid);
        return Ok(uuid);
    } else if env::var("ROXTERM_ID").is_ok() {
        let uuid = env::var("ROXTERM_ID")?;
        debug!("ROXTERM_ID: {}", uuid);
        return Ok(uuid);
    } else if env::var("WINDOWID").is_ok() {
        let uuid = env::var("WINDOWID")?;
        debug!("WINDOWID: {}", uuid);
        if uuid == "0" {
            error!("Error: terminal has a UUID of zero");
            return Err(From::from("Error: terminal has a UUID of zero"));
        }
        return Ok(uuid);
    } else {
        error!("Couldn't determine the terminal UUID");
        return Err(From::from("Couldn't determine the terminal UUID"));
    }
}

#[cfg(test)]
mod tests {
    use super::{env, terminal_uuid, Error};

    #[test]
    fn test_ttyuuid_empty() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::remove_var("TERMINATOR_UUID");
        env::remove_var("TILIX_ID");
        env::remove_var("ROXTERM_ID");
        env::remove_var("WINDOWID");
        let result = terminal_uuid();
        if result.is_err() {
            return Ok(());
        } else {
            return Err(From::from("Test failed! It shouldn't accept 0"));
        }
    }

    #[test]
    fn test_ttyuuid_windowid_zero() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::remove_var("TERMINATOR_UUID");
        env::remove_var("TILIX_ID");
        env::remove_var("ROXTERM_ID");
        env::set_var("WINDOWID", "0");
        let result = terminal_uuid();
        if result.is_err() {
            return Ok(());
        } else {
            return Err(From::from("Test failed! It shouldn't accept 0"));
        }
    }

    #[test]
    fn test_ttyuuid_windowid() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::remove_var("TERMINATOR_UUID");
        env::remove_var("TILIX_ID");
        env::remove_var("ROXTERM_ID");
        env::set_var("WINDOWID", "325");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "325" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as WINDOWID",
            ));
        }
    }

    #[test]
    fn test_ttyuuid_roxterm_id() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::remove_var("TERMINATOR_UUID");
        env::remove_var("TILIX_ID");
        env::set_var("ROXTERM_ID", "325768");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "325768" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as ROXTERM_ID",
            ));
        }
    }

    #[test]
    fn test_ttyuuid_tilix_id() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::remove_var("TERMINATOR_UUID");
        env::set_var("TILIX_ID", "32576");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "32576" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as TILIX_ID",
            ));
        }
    }

    #[test]
    fn test_ttyuuid_terminator_uuid() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::remove_var("SHELL_SESSION_ID");
        env::set_var("TERMINATOR_UUID", "3257689");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "3257689" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as TERMINATOR_UUID",
            ));
        }
    }

    #[test]
    fn test_ttyuuid_shell_session_id() -> Result<(), Box<dyn Error>> {
        env::remove_var("GNOME_TERMINAL_SCREEN");
        env::set_var("SHELL_SESSION_ID", "325768vd");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "325768vd" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as SHELL_SESSION_ID",
            ));
        }
    }

    #[test]
    fn test_ttyuuid_gnome_terminal_screen() -> Result<(), Box<dyn Error>> {
        env::set_var("GNOME_TERMINAL_SCREEN", "32576821");
        let ttyuuid = terminal_uuid()?;
        if ttyuuid.is_empty() {
            return Err(From::from("ttyuuid shouldn't be empty"));
        } else if ttyuuid == "32576821" {
            return Ok(());
        } else {
            return Err(From::from(
                "Test Failed: should have been the same number as GNOME_TERMINAL_SCREEN",
            ));
        }
    }
}
