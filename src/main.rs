mod commands;
mod discord;
mod events;
mod voicevox;

use std::{env, path::Path};

use anyhow::{Context, Result};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    println!("initialize voicevox");
    let voicevox_path = Path::new("./voicevox_core");
    //voicevox::init(voicevox_path)?;

    println!("initialize discord bot");
    dotenv()?;

    let mut client = discord::init(&env::var("TOKEN").context("").unwrap()).await?;

    client.start().await?;

    Ok(())
}
