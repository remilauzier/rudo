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
use std::{error::Error, fs::File, io::Write, os::unix::fs::PermissionsExt, path::Path};

use log::debug;

/// `vec_to_string` take a vector of str and put each str in a string for later use
pub(crate) fn vec_to_string(data: Vec<&str>) -> String {
    let mut buffer = String::new();
    for buf in data {
        buffer.push_str(buf);
    }
    buffer
}

/// Function that create a file with a path, a mode and with data
pub(crate) fn create_file(path: &Path, mode: u32, data: &str) -> Result<(), Box<dyn Error>> {
    // Creating the file
    debug!("Creating the file");
    let mut file = File::create(path)?;

    // Write the data in the file
    debug!("Writing the string in the file");
    file.write_all(data.as_bytes())?;

    // Sync data to be sure everything is written on drive
    debug!("Syncing data on drive");
    file.sync_all()?;

    // Put file permission to 600 to restraint access
    debug!("Set file permission");
    let mut perms = file.metadata()?.permissions();
    perms.set_mode(mode);
    file.set_permissions(perms)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::{create_file, vec_to_string, Path};

    #[test]
    fn test_vec_to_string() -> Result<(), Box<dyn Error>> {
        let data = vec!["test", "case"];
        let buffer = vec_to_string(data);
        if buffer.is_empty() {
            Err(From::from("Test failed. Shouldn't be empty"))
        } else if buffer == "testcase" {
            Ok(())
        } else {
            Err(From::from("Test failed to convert vec to string correctly"))
        }
    }

    #[test]
    fn test_create_file() -> Result<(), Box<dyn Error>> {
        let path = Path::new("test.txt");
        create_file(path, 0o600, "1234")?;
        let data = fs::read_to_string(path)?;
        if data == "1234" {
            Ok(())
        } else {
            Err(From::from(
                "Test failed! file was not create with the right data",
            ))
        }
    }
}
