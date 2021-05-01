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
use std::os::unix::process::CommandExt;
use std::process::Command;

use log::debug;
use pam_client::conv_cli::Conversation;
use pam_client::Session;

/// Structure to keep the result of the extraction of the command give in the command-line interface
pub(crate) struct CmdData<'a> {
    /// Name of the program
    pub(crate) program: String,
    /// The arguments of the program
    pub(crate) args: Vec<&'a str>,
}

impl<'a> CmdData<'a> {
    /// Create the new Command with the command supply by the user with the command-line interface
    pub(crate) fn new(mut command: Vec<&'a str>) -> Result<Self, Box<dyn Error>> {
        let mut program = String::new();
        // Extract the first word then remove it after verifying its existence
        debug!("Extract the first word then remove it after verifying its existence");
        let data = match command.get(0) {
            Some(data) => data,
            None => {
                return Err(From::from(
                    "Command is empty! Please give Rudo something to launch",
                ))
            }
        };
        program.push_str(data);
        command.remove(0);
        // Copy the rest of the value and return it
        let args = command;
        debug!("Return the new Command structure");
        return Ok(Self { program, args });
    }
}

/// Function that start a new command with the pass arguments
pub(crate) fn start_command(
    data: CmdData<'_>,
    session: &Session<'_, Conversation>,
    user: &users::User,
) -> Result<(), Box<dyn Error>> {
    // Start of the command
    debug!("Start of the command");
    let mut child = Command::new(data.program)
        .args(data.args)
        .envs(session.envlist().iter_tuples()) // Pass the Pam session to the new process
        .uid(user.uid()) // Necessary to have full access
        .gid(user.primary_group_id()) // Necessary to have full access
        .spawn()?;

    // Wait for the command to finish, or the program end before the command
    child.wait()?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::{CmdData, Error};

    #[test]
    fn test_command_new() -> Result<(), Box<dyn Error>> {
        let command = CmdData::new(vec!["test"]);
        if command.is_ok() {
            return Ok(());
        } else {
            return Err(From::from("Test failed to create structure"));
        }
    }

    #[test]
    fn test_command_new_empty() -> Result<(), Box<dyn Error>> {
        let command = CmdData::new(vec![]);
        if command.is_err() {
            return Ok(());
        } else {
            return Err(From::from("Test failed to see an empty vector"));
        }
    }
    #[test]
    fn test_command_new_full() -> Result<(), Box<dyn Error>> {
        let command = CmdData::new(vec!["test", "command", "full"])?;
        if command.program == "test" && command.args == vec!["command", "full"] {
            return Ok(());
        } else {
            return Err(From::from("Test failed to reproduced structure"));
        }
    }
}
