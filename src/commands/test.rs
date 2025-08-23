use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Test command
#[poise::command(slash_command, subcommands("child1", "child2"))]
pub async fn test(
    _ctx: Context<'_>,
    #[description = "Description of arg1 here"] _arg1: serenity::Member,
) -> Result<(), Error> {
    println!("Test was run");
    Ok(())
}

/// Let me test child1
#[poise::command(prefix_command, slash_command)]
pub async fn child1(_ctx: Context<'_>, _arg: String) -> Result<(), Error> {
    Ok(())
}

/// Let me test child2
#[poise::command(prefix_command, slash_command)]
pub async fn child2(_ctx: Context<'_>, _arg: String) -> Result<(), Error> {
    Ok(())
}
