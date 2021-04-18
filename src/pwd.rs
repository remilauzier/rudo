use log::{debug, error};
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
use std::error::Error;

/// `Password_input` is a function that ask the user for there password.
/// Pam validate the password
pub(crate) fn password_input(
    password: bool,
    context: &mut Context<Conversation>,
) -> Result<(), Box<dyn Error>> {
    // Don't ask for password if false in the configuration
    if password {
        // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
        debug!("Password will be ask a maximum of 3 time to the user");
        let mut count = 0;
        while count < 3 {
            if let Ok(()) = context.authenticate(Flag::DISALLOW_NULL_AUTHTOK) {
                break;
            }
            error!("Password was incorrect! Will be report to administrator!");
            eprintln!("Password was incorrect! Will be report to administrator!");
            count += 1
        }
        if count == 3 {
            return Err(From::from("You have made three mistake! Rudo's Out!"));
        }
    }
    Ok(())
}
