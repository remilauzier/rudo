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
use libc::{isatty, ttyname};
use std::env;
use std::error::Error;
use std::ffi::CStr;

/// Safe wrapper to get the name of the current TTY
/// and return it as a Rust string for further use
pub fn get_tty_name() -> Result<String, Box<dyn Error>> {
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
        debug!("Terminal: {}", ttyname_rust);
        Ok(ttyname_rust)
    }
}

/// WINDOWID is the least trust because of is small size and don't change for different tabs.
/// It only change the last five number most of the time.
/// But it is used by st, xterm, sakura, kitty, xfce terminal, mate terminal and terminology.
/// Rox terminal use a value that change only the last six number but change for tabs.
/// Qterminal is insecure as it put 0 in WINDOWID and Rudo will refuse to consider it.
/// Guake, lxterminal, elementary terminal and deepin terminal as no UUID to use for now.
pub fn tty_uuid() -> Result<String, Box<dyn Error>> {
    if env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        let uuid = env::var("GNOME_TERMINAL_SCREEN")?;
        debug!("GNOME_TERMINAL_SCREEN: {}", uuid);
        Ok(uuid)
    } else if env::var("SHELL_SESSION_ID").is_ok() {
        let uuid = env::var("SHELL_SESSION_ID")?;
        debug!("SHELL_SESSION_ID: {}", uuid);
        Ok(uuid)
    } else if env::var("TERMINATOR_UUID").is_ok() {
        let uuid = env::var("TERMINATOR_UUID")?;
        debug!("TERMINATOR_UUID: {}", uuid);
        Ok(uuid)
    } else if env::var("TILIX_ID").is_ok() {
        let uuid = env::var("TILIX_ID")?;
        debug!("TILIX_ID: {}", uuid);
        Ok(uuid)
    } else if env::var("ROXTERM_ID").is_ok() {
        let uuid = env::var("ROXTERM_ID")?;
        debug!("ROXTERM_ID: {}", uuid);
        Ok(uuid)
    } else if env::var("WINDOWID").is_ok() {
        let uuid = env::var("WINDOWID")?;
        debug!("WINDOWID: {}", uuid);
        if uuid.parse::<u32>().unwrap() == 0 {
            error!("Error: terminal has a UUID of zero");
            return Err(From::from("Error: terminal has a UUID of zero"));
        }
        Ok(uuid)
    } else {
        error!("Couldn't determine the terminal UUID");
        Err(From::from("Couldn't determine the terminal UUID"))
    }
}
