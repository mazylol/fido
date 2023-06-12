use crate::{Context, Error};

/// Show help about command(s)
#[poise::command(prefix_command, track_edits, slash_command)]
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
            extra_text_at_bottom: "Mazy by @mazylol",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Get a random dog picture
#[poise::command(prefix_command, slash_command)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("{}", crate::api::call::random().await.message))
        .await?;
    Ok(())
}
