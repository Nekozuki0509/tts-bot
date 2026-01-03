use std::sync::{Arc, RwLock};

use poise::{
    CreateReply, command,
    serenity_prelude::{ChannelId, Mentionable},
};

use crate::discord::{Context, Error, ReadCh};

#[command(slash_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "voice channel to join"] channel: Option<ChannelId>,
    #[description = "forcibly join the channel"] force: Option<bool>,
) -> Result<(), Error> {
    let (guild_id, channel) = {
        let guild = ctx.guild().unwrap();
        let channel = channel.or(guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|state| state.channel_id));

        (guild.id, channel)
    };

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    if !force.unwrap_or(false) && manager.get(guild_id).is_some() {
        ctx.send(
            CreateReply::default()
                .content("すでにボイスチャンネルに接続していますi\n-# tip: forceオプションをtrueにすると強制的に接続します")
                .ephemeral(true)
                .reply(true),
        )
        .await?;

        return Ok(());
    }

    let channel_id = if let Some(channel) = channel {
        channel
    } else {
        ctx.send(
            CreateReply::default()
                .content("ボイスチャンネルに入るかチャンネルを指定してください")
                .ephemeral(true)
                .reply(true),
        )
        .await?;

        return Ok(());
    };

    manager.join(guild_id, channel_id).await?;

    {
        let mut data = ctx.serenity_context().data.write().await;
        data.insert::<ReadCh>(Arc::new(RwLock::new(channel_id.get())));
    }

    ctx.say(format!("{}に接続しました！", channel_id.mention()))
        .await?;

    Ok(())
}
