use crate::{Context, Error};

use super::api::call;

/// Show help about command(s)
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Made by @mazylol",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Get a random dog picture
#[poise::command(slash_command)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Breed of dog to get a random picture of"] breed: Option<String>,
    #[description = "Sub-breed of dog to get a random picture of"] sub_breed: Option<String>,
) -> Result<(), Error> {
    if let Some(sub_breed) = sub_breed {
        let url = call::random_by_sub_breed(breed.unwrap(), sub_breed)
            .await
            .message;
        ctx.say(url).await?;
    } else if let Some(breed) = breed {
        let url: String = call::random_by_breed(breed).await.message;
        ctx.say(url).await?;
    } else {
        let url = call::random().await.message;
        ctx.say(url).await?;
    }
    Ok(())
}
