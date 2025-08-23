use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use ::serenity::all::Timestamp;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;

/// Embed (test) command
#[poise::command(slash_command)]
pub async fn embed(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("Embed")
        .thumbnail("https://chinese.freecodecamp.org/news/content/images/2021/02/rust-mascot.png")
        .description("Use any resources below for help!")
        .footer(CreateEmbedFooter::new("Using Processor"))
        .colour(Colour::from_rgb(0, 255, 0))
        .url("https://docs.rs/serenity/latest/serenity/")
        .timestamp(Timestamp::now());

    // Defer the response to buy us some time
    ctx.defer_ephemeral().await?;

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

