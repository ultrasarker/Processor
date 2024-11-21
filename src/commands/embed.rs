use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use serenity::model::Colour;

/// Embed command
#[poise::command(slash_command)]
pub async fn embed(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .title("Embed")
        .field("Hi", "Hello", true)
        .footer(CreateEmbedFooter::new("Using Processor"))
        .colour(Colour::from_rgb(0, 255, 0));

    // Defer the response to buy us some time
    ctx.defer_ephemeral().await?;

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
