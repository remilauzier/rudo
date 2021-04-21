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
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use clap::ArgMatches;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::CONFIG_PATH;

#[derive(Serialize, Deserialize, Clone)]
/// `UserConf` structure is the representation of the data of a part of the configuration file
pub(crate) struct UserConf {
    /// The Unix username of an authorized user
    pub(crate) username: String,
    /// The group the user must be a member to have authorization to use Rudo
    pub(crate) group: String,
    /// A Boolean to determine if the user must give is password or not
    pub(crate) password: bool,
    /// A Boolean to determine if the user want to be saluted every time Rudo is invoked
    pub(crate) greeting: bool,
}

impl UserConf {
    /// Function to update the greeting Boolean if the "-g" option was given
    pub(crate) fn update(mut self, matches: &ArgMatches<'_>) -> Self {
        // Update greeting value if CLI option is present
        if matches.is_present("greeting") {
            debug!("Greeting value will be update");
            self.greeting = true;
        }
        self
    }
}

impl Default for UserConf {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            group: String::from("wheel"),
            password: true,
            greeting: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
/// `RudoConf` is where the program stock is configuration
pub(crate) struct RudoConf {
    /// impuser is the Unix name of the user you want to impersonate
    pub(crate) impuser: String,
}

impl Default for RudoConf {
    fn default() -> Self {
        Self {
            impuser: String::from("root"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
/// Config is the sum of `UserConf` and `RudoConf` as represent in the configuration file
pub(crate) struct Config {
    /// rudo is where the program stock is configuration
    pub(crate) rudo: RudoConf,
    /// user is where a vector of user configuration is stock to permit multiple user configuration
    pub(crate) user: Vec<UserConf>,
}

impl Config {
    /// Function to create the configuration file with the right permissions, and it's data
    fn create_config_file(&self) -> Result<(), Box<dyn Error>> {
        // Create the path for the configuration
        let config_path = Path::new(CONFIG_PATH);
        // Transform the structure to YAML
        debug!("Creating default data for configuration file");
        let config_file = serde_yaml::to_string(&self)?;
        // Create the configuration file
        debug!("Creating configuration file at {}", CONFIG_PATH);
        let mut file = File::create(config_path)?;
        // Write data in the file
        debug!("Writing data to file");
        file.write_all(config_file.as_bytes())?;
        // Sync data to drive
        debug!("Syncing data to drive");
        file.sync_all()?;
        // Set permissions of 640 to restraint access
        debug!("Set file permission to 640 to restreint access");
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o640);
        file.set_permissions(perms)?;

        Ok(())
    }
    /// Function to update the name of the impersonated user with the value give in the command-line
    pub(crate) fn update(mut self, matches: &ArgMatches<'_>) -> Result<Self, Box<dyn Error>> {
        // Update user value if CLI option is present
        if matches.is_present("user") {
            debug!("User value will be update");
            self.rudo.impuser = match matches.value_of("user") {
                Some(user) => user.to_owned(),
                None => return Err(From::from("user value couldn't be converted to a string")),
            };
        }
        Ok(self)
    }
}
// Default value for configuration
impl Default for Config {
    fn default() -> Self {
        Self {
            rudo: RudoConf::default(),
            user: vec![UserConf::default()],
        }
    }
}
/// Function to initialize the configuration with the default data if necessary
pub(crate) fn init_conf() -> Result<Config, Box<dyn Error>> {
    // Initialize configuration with defaults
    debug!("Begin initializing default configuration for further use");
    let mut conf = Config::default();

    // Verify that the file is there or write to it with the defaults
    let path = Path::new(CONFIG_PATH);
    debug!("Verifying that {} exist", CONFIG_PATH);
    if path.exists() && path.is_file() {
        // Load the file and verify its validity
        debug!("Loading {}", CONFIG_PATH);
        let result = read_config_file();
        if let Err(err) = result {
            eprintln!("{}", err);
            error!("{}", err);
            // Remove invalid file
            warn!("Removing invalid file");
            fs::remove_file(path)?;
            // Create new file with defaults
            info!("Creating new file with defaults at {}", CONFIG_PATH);
            conf.create_config_file()?;
            return Ok(conf);
        }
        // Return the valid data of the configuration file
        debug!("Returning the content of the configuration file");
        conf = result?;
    } else if path.exists() && path.is_dir() {
        // Error if it's a directory and let the user decide what to do
        let err = format!("Error: {} is a directory", CONFIG_PATH);
        error!("{}", err);
        return Err(From::from(err));
    } else {
        // Create a configuration file if it doesn't exist
        warn!("{} doesn't exist! Creating it", CONFIG_PATH);
        eprintln!("{} doesn't exist! Creating it", CONFIG_PATH);
        conf.create_config_file()?;
    }
    Ok(conf)
}

/// Function to read the configuration file and extract its data
pub(crate) fn read_config_file() -> Result<Config, Box<dyn Error>> {
    // Create the path for the configuration
    let config_path = Path::new(CONFIG_PATH);
    // Open the existing configuration file
    debug!("Opening configuration file at {}", CONFIG_PATH);
    let buffer = fs::read_to_string(config_path)?;
    // transform data to structure with serde
    debug!("Transform data to a structure with serde");
    let config: Config = serde_yaml::from_str(&buffer)?;
    // Return the configuration
    Ok(config)
}

/// Extract, from the vector of `UserConf` of the configuration file, the user presently accessing Rudo,
/// and pass all the information associate with it for later use
pub(crate) fn extract_userconf(conf: Vec<UserConf>, username: &str) -> UserConf {
    let mut user = UserConf::default();
    for cf in conf {
        if cf.username == username {
            user = cf;
        }
    }
    user
}

#[cfg(test)]
mod tests {
    use super::{extract_userconf, Error, UserConf};

    #[test]
    fn test_extract_userconf() -> Result<(), Box<dyn Error>> {
        let conf = UserConf::default();
        let conf = vec![conf];
        if extract_userconf(conf, "root").username == "root" {
            Ok(())
        } else {
            Err(From::from("Test failed when extracting the userconf"))
        }
    }
}
