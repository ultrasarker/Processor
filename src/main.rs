#![allow(non_camel_case_types)]

mod commands;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
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
    // Load .env file
    dotenv().ok();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::ping::ping(),
            commands::embed::embed(),
            commands::test::test(),
            commands::random::animal(),
            commands::random::animu(),
            commands::random::animu_quote(),
            commands::random::joke(),
            commands::tools::colour(),
            commands::weather::weather(),
            commands::scrape::scrape(),
        ],
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got {:?} in event handler", event.snake_case_name());
                Ok(())
            })
        },
        ..Default::default()
    };
    let guild_id = env::var("GUILD_ID").expect("Expected a guild id in the environment");

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands in a specific guild
                let guild_id = guild_id.parse().expect("Invalid guild id");
                use serenity::gateway::ActivityData;
                use serenity::model::user::OnlineStatus;

                let activity = ActivityData::custom("❤️ ultrasarker");
                let status = OnlineStatus::Online;

                ctx.set_presence(Some(activity), status);
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId::new(guild_id),
                )
                .await?;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
