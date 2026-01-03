use poise::{
    command,
    serenity_prelude::{Mentionable, User},
};

use crate::discord::{Context, Error};

#[command(slash_command)]
pub async fn hello(
    ctx: Context<'_>,
    #[description = "say hello to user"] user: Option<User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    ctx.say(format!("Hello, {}!!!", user.mention())).await?;

    Ok(())
}
