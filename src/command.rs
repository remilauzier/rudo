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
use std::error::Error;

/// Structure to keep the result of the extraction of the command give in the command-line interface
pub struct Command<'a> {
    /// Name of the program
    pub program: String,
    /// The arguments of the program
    pub args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    /// Create the new Command with the command supply by the user with the command-line interface
    pub fn new(mut command: Vec<&'a str>) -> Result<Self, Box<dyn Error>> {
        // Verify that it's not empty
        debug!("Verifying that command is not empty");
        if !command.is_empty() {
            debug!("command is not empty, proceeding");
            let mut program = String::new();
            // Extract the first word then remove it
            program.push_str(command[0]);
            command.remove(0);
            // Copy the rest of the value and return it
            let args = command;
            debug!("Return Command structure");
            Ok(Self { program, args })
        } else {
            // Error if the command is empty
            error!("Command is empty");
            Err(From::from("Command is empty"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() -> Result<(), Box<dyn Error>> {
        let command = Command::new(vec!["test"]);
        if command.is_ok() {
            Ok(())
        } else {
            Err(From::from("Test failed to create structure"))
        }
    }
    #[test]
    fn test_command_new_empty() -> Result<(), Box<dyn Error>> {
        let command = Command::new(vec![]);
        if command.is_err() {
            Ok(())
        } else {
            Err(From::from("Test failed to see an empty vector"))
        }
    }
    #[test]
    fn test_command_new_full() -> Result<(), Box<dyn Error>> {
        let command = Command::new(vec!["test", "command", "full"])?;
        if command.program == "test" && command.args == vec!["command", "full"] {
            Ok(())
        } else {
            Err(From::from("Test failed to reproduced structure"))
        }
    }
}
