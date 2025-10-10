use crate::{Context, Error};

#[poise::command(slash_command, subcommands("hex"))]
pub async fn colour(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Show a colour in HEX format RRGGBB.
#[poise::command(slash_command)]
pub async fn hex(
    ctx: Context<'_>,
    #[description = "Colour to display in the format RRGGBB"] colour: String,
) -> Result<(), Error> {
    let url: String = format!(
        "https://api.some-random-api.com/canvas/colorviewer?hex={}",
        colour
    );
    ctx.say(url).await?;
    Ok(())
}
