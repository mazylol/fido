mod api;
mod commands;

use dotenv;

use poise::serenity_prelude as serenity;
use std::{env::var, time::Duration};

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::random()],
        prefix_options: poise::PrefixFrameworkOptions {
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        ..Default::default()
    };

    if var("PROD").is_ok() {
        println!("Running in production mode!");
        poise::Framework::builder()
            .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var!"))
            .setup(move |ctx, _ready, framework| {
                Box::pin(async move {
                    println!("Logged in as {}", _ready.user.name);
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {})
                })
            })
            .options(options)
            .intents(serenity::GatewayIntents::non_privileged())
            .run()
            .await
            .unwrap();
    } else {
        println!("Running in development mode!");
        poise::Framework::builder()
        .token(var("DEV_DISCORD_TOKEN").expect("Missing `DEV_DISCORD_TOKEN` env var!"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(
                        var("GUILD_ID")
                            .expect("GUILD_ID not found!")
                            .parse::<u64>()
                            .unwrap(),
                    ),
                )
                .await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged(),
        )
        .run()
        .await
        .unwrap();
    }
}
