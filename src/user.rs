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

use log::{debug, error, info};
use users::{Group, Users, UsersCache};

/// Put the data of the actual user in a structure for later use
pub(crate) struct User {
    /// The Unix username of the actual user
    pub(crate) username: String,
    /// The Unix groups the user is part of
    group: Vec<Group>,
}

impl User {
    /// Function to create the user structure with all it's data
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        // Create the user, and it's data for later use
        debug!("Begin user data creation");
        let userscache = UsersCache::new();
        let uid = userscache.get_current_uid();
        let user = match userscache.get_user_by_uid(uid) {
            Some(user) => user,
            None => return Err(From::from("Couldn't extract user from its UID")),
        };
        let username = match user.name().to_str() {
            Some(name) => name.to_owned(),
            None => return Err(From::from("Couldn't convert &str to string")),
        };
        let group = match user.groups() {
            Some(group) => group,
            None => {
                return Err(From::from(
                    "Couldn't extract group membership from user data",
                ))
            }
        };
        Ok(Self { username, group })
    }
    /// Function that verify that the user is part of the list of authorized users as defined in the configuration file
    pub(crate) fn verify_user(&self, username: &str) -> Result<(), Box<dyn Error>> {
        debug!(
            "Begin to verify if {} is authorized to use Rudo",
            &self.username
        );
        let actualuser = &self.username;
        if actualuser == username {
            debug!("{} has been authorized", actualuser);
            Ok(())
        } else {
            let err = format!(
                "{} is not authorized to use Rudo! Will be report to administrator!",
                actualuser
            );
            error!("{}", err);
            Err(From::from(err))
        }
    }
    /// Function that take the vector containing the list of groups the user is a member,
    /// and search for the group supply in the configuration to determine the authorization of the user
    pub(crate) fn verify_group(&self, arggroup: &str) -> Result<(), Box<dyn Error>> {
        debug!(
            "Beginning to verify if the user is a member of {}",
            arggroup
        );
        let group = &self.group;
        let mut count = 0;

        // Compare the supply group with the list of the user membership
        for gr in group {
            if gr.name() == arggroup {
                count += 1;
            }
        }
        if count == 1 {
            info!("User is a member of the authorized group: {}", arggroup);
            Ok(())
        } else if count >= 2 {
            let err = format!("{} is list multiple time", arggroup);
            error!("{}", err);
            Err(From::from(err))
        } else {
            let error = format!("User is not a member of the authorized group: {}", arggroup);
            error!("{}", error);
            Err(From::from(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, User};

    #[test]
    fn test_verify_group() -> Result<(), Box<dyn Error>> {
        let userdata = User::new()?;
        if userdata.verify_group("test").is_err() {
            Ok(())
        } else {
            Err(From::from("The group should not correspond with test"))
        }
    }
    #[test]
    fn test_verify_user() -> Result<(), Box<dyn Error>> {
        let userdata = User::new()?;
        if userdata.verify_user("test").is_err() {
            Ok(())
        } else {
            Err(From::from("The user should not correspond with test"))
        }
    }
}
