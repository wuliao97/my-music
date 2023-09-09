use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::user::User;

pub mod invite;
pub mod spotify;
pub mod about;



pub fn parse(msg: ApplicationCommandInteraction) -> User {
    match msg.data.options.get(0) {
        Some(option) => {
            if let CommandDataOptionValue::User(_user, _) = option.resolved.as_ref().unwrap() {
                _user.clone()
            } else {
                msg.user.clone()
            }
        }
        _ => { msg.user.clone() }
    }
}