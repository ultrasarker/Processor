use crate::{Context, Error};
use std::time::Instant;

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();

    // Defer the response to buy us some time
    ctx.defer_ephemeral().await?;

    let end_time = Instant::now();
    let latency = end_time.duration_since(start_time);

    // Send the response
    ctx.say(format!("Pong! Latency: {}ms", latency.as_millis()))
        .await?;

    Ok(())
}
