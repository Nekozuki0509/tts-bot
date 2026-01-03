use poise::command;
use songbird::{input::File, tracks::Track};

use crate::discord::{Context, Error};

#[command(slash_command)]
pub async fn voice(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if manager.get(guild_id).is_none() {
        ctx.reply("ボイスチャンネルにいれてください").await.unwrap();
        return Ok(());
    }

    let handler_lock = manager.get(guild_id).unwrap();
    let mut handler = handler_lock.lock().await;

    let source = File::new("./voice.mp3");

    let handle = handler.play_only(Track::from(source));

    handle.enable_loop().unwrap();

    ctx.reply("start playing").await.unwrap();

    Ok(())
}
