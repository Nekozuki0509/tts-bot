use poise::serenity_prelude::{ChannelId, Context, EventHandler, Message, Ready, async_trait};

use crate::discord::{DefaultCh, Handler, ReadCh};

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let channel_id = {
            let data_read = ctx.data.read().await;
            data_read.get::<DefaultCh>().unwrap().clone()
        };

        ChannelId::new(*channel_id)
            .say(&ctx.http, format!("{} is connected!", ready.user.name))
            .await
            .unwrap();
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let guild = msg.guild(&ctx.cache).unwrap().clone();
        let manager = songbird::get(&ctx).await.unwrap().clone();
        let Some(call) = manager.get(guild.id) else {
            return;
        };

        let read_ch_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<ReadCh>().unwrap().clone()
        };
    }
}
