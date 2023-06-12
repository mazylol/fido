use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

use crate::api::call::random;

pub async fn run(_options: &[CommandDataOption]) -> String {
    random().await.message
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("random").description("Random dog picture!")
}
