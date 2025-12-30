use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::OnceLock,
};

use anyhow::{Context, Result};
use const_format::concatcp;

use voicevox_core::{
    StyleId,
    blocking::{Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile},
};

static SYNTH: OnceLock<Synthesizer<OpenJtalk>> = OnceLock::new();

pub fn init(voicevox_path: &Path) -> Result<()> {
    let synth = {
        let ort = Onnxruntime::load_once()
            .filename(voicevox_path.join(concatcp!(
                "onnxruntime/lib/",
                Onnxruntime::LIB_VERSIONED_FILENAME
            )))
            .perform()?;
        let ojt = OpenJtalk::new(
            voicevox_path
                .join("dict/open_jtalk_dic_utf_8-1.11")
                .as_path()
                .to_str()
                .context("")?,
        )?;
        Synthesizer::builder(ort).text_analyzer(ojt).build()?
    };

    for vvm in fs::read_dir(voicevox_path.join("models/vvms"))? {
        let path = vvm?.path();
        let vvm = path.to_str().context("")?;
        println!("loading {}", vvm);
        synth.load_voice_model(&VoiceModelFile::open(vvm)?)?;
    }

    SYNTH.set(synth).ok().context("")?;

    Ok(())
}

pub fn tts(msg: &str, style_id: u32) -> Result<()> {
    let mut file = File::create("test.wav")?;
    file.write_all(
        &SYNTH
            .get()
            .context("")?
            .tts(msg, StyleId::new(style_id))
            .perform()?,
    )?;

    Ok(())
}
