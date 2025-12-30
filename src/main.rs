mod voicevox;
use std::path::Path;

use anyhow::Result;

fn main() -> Result<()> {
    println!("initialize voicevox");
    let voicevox_path = Path::new("./voicevox_core");
    voicevox::init(voicevox_path)?;

    println!("generating voice");
    voicevox::tts("こんにちわ", 0)?;

    Ok(())
}
