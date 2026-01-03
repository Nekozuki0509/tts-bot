use std::sync::{Arc, RwLock};

use anyhow::Result;
use poise::{
    Framework, FrameworkOptions,
    builtins::register_globally,
    serenity_prelude::{Client, ClientBuilder, GatewayIntents, prelude::TypeMapKey},
};
use songbird::SerenityInit;

use crate::commands::{hello, join, voice};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Handler;

pub struct Data {}

pub struct DefaultCh;
impl TypeMapKey for DefaultCh {
    type Value = Arc<u64>;
}

pub struct ReadCh;
impl TypeMapKey for ReadCh {
    type Value = Arc<RwLock<u64>>;
}

pub async fn init(token: &str) -> Result<Client> {
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hello::hello(), join::join(), voice::voice()],
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await?;

    {
        let mut data = client.data.write().await;

        data.insert::<DefaultCh>(Arc::new(1455843738990542931));
        data.insert::<ReadCh>(Arc::new(RwLock::new(0)));
    }

    Ok(client)
}
