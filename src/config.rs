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
use clap::ArgMatches;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

static CONFIG_PATH: &str = "/etc/rudo.conf";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub group: String,
    pub password: bool,
    pub greeting: bool,
}

impl UserConfig {
    pub fn update(mut self, matches: &ArgMatches) -> Self {
        // Update greeting value if CLI option is present
        if matches.is_present("greeting") {
            debug!("Greeting value will be update");
            self.greeting = true;
        }
        self
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            group: String::from("wheel"),
            password: true,
            greeting: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RudoConfig {
    pub impuser: String,
}

impl Default for RudoConfig {
    fn default() -> Self {
        Self {
            impuser: String::from("root"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rudo: RudoConfig,
    pub user: Vec<UserConfig>,
}

impl Config {
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
        debug!("Writing to file");
        file.write_all(&config_file.as_bytes())?;
        // Sync data to drive
        debug!("Syncing data to drive");
        file.sync_all()?;
        // Set permissions of 640 to restraint access
        debug!("Set file permission");
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o640);
        file.set_permissions(perms)?;
        debug!("File permission has been set");

        Ok(())
    }
    fn read_config_file(&self) -> Result<Self, Box<dyn Error>> {
        // Create the path for the configuration
        let config_path = Path::new(CONFIG_PATH);
        // Open the existing configuration file
        debug!("Opening configuration file at {}", CONFIG_PATH);
        let mut file = File::open(config_path)?;
        // Put data in a buffer for later use
        debug!("Putting data in a string for lather use");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        // transform data to structure with serde
        debug!("Transform data to a structure with serde");
        let config: Config = serde_yaml::from_str(&buffer)?;
        // Return the configuration
        Ok(config)
    }
    pub fn update(mut self, matches: &ArgMatches) -> Self {
        // Update user value if CLI option is present
        if matches.value_of("user").is_some() {
            debug!("User value will be update");
            self.rudo.impuser = matches.value_of("user").unwrap().to_string();
        }
        self
    }
}
// Default value for configuration
impl Default for Config {
    fn default() -> Self {
        Self {
            rudo: RudoConfig::default(),
            user: vec![UserConfig::default()],
        }
    }
}
// Initialize the configuration
pub fn init_conf() -> Result<Config, Box<dyn Error>> {
    // Initialize configuration with defaults
    debug!("Begin initializing default configuration");
    let mut conf = Config::default();
    debug!("Finish initializing default configuration");

    // Verify that the file is there or write to it with the defaults
    let path = Path::new(CONFIG_PATH);
    debug!("Verifying that {} exist", CONFIG_PATH);
    if path.exists() && path.is_file() {
        // Load the file and verify it's validity
        debug!("Loading {}", CONFIG_PATH);
        let result = conf.read_config_file();
        if let Err(err) = result {
            eprintln!("{}", err);
            error!("{}", err);
            // Remove invalid file
            info!("Removing invalid file");
            fs::remove_file(path)?;
            // Create new file with defaults
            info!("Creating new file with defaults at {:?}", path);
            conf.create_config_file()?;
            return Ok(conf);
        } else {
            // Return the valid data of the configuration file
            debug!("Returning the content of the configuration file");
            conf = result.unwrap();
        }
        debug!("Finish loading configuration");
    } else if path.exists() && path.is_dir() {
        // Error if it's a directory and let the user decide what to do
        let err = format!("Error: {} is a directory", CONFIG_PATH);
        error!("{}", err);
        return Err(From::from(err));
    } else if !path.exists() {
        // Create a configuration file if it doesn't exist
        info!("{} doesn't exist! Creating it", CONFIG_PATH);
        eprintln!("{} doesn't exist! Creating it", CONFIG_PATH);
        conf.create_config_file()?;
        debug!("Creation has finish");
    }
    Ok(conf)
}

// Extract from the vector of UserConfig of rudo.conf the user presently accessing Rudo
// Pass all the information associate with it after
pub fn extract_userconf(
    conf: Vec<UserConfig>,
    username: &str,
) -> Result<UserConfig, Box<dyn Error>> {
    let mut user = UserConfig::default();
    for cf in conf {
        if cf.username == username {
            user = cf;
        }
    }
    Ok(user)
}
