//! トーク

use std::{fs::File, io::Write, panic};

use anyhow::{Context as _, Ok};
use const_format::concatcp;

use voicevox_core::{
    CharacterMeta, StyleMeta,
    blocking::{Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile},
};

// ダウンローダーにて`onnxruntime`としてダウンロードできるもの
const VVORT: &str = concatcp!(
    "./voicevox_core/onnxruntime/lib/",
    Onnxruntime::LIB_VERSIONED_FILENAME,
);

// ダウンローダーにて`dict`としてダウンロードできるもの
const OJT_DIC: &str = "./voicevox_core/dict/open_jtalk_dic_utf_8-1.11";

// ダウンローダーにて`models`としてダウンロードできるもの
const VVM: &str = "./voicevox_core/models/vvms/0.vvm";

const TARGET_CHARACTER_NAME: &str = "ずんだもん";
const TARGET_STYLE_NAME: &str = "ノーマル";
const TEXT: &str = "こんにちは";

fn main() -> anyhow::Result<()> {
    let synth = {
        let ort = Onnxruntime::load_once().filename(VVORT).perform()?;
        let ojt = OpenJtalk::new(OJT_DIC)?;
        Synthesizer::builder(ort).text_analyzer(ojt).build()?
    };

    dbg!(synth.is_gpu_mode());

    synth.load_voice_model(&VoiceModelFile::open(VVM)?)?;

    let StyleMeta { id: style_id, .. } = synth
        .metas()
        .into_iter()
        .filter(|CharacterMeta { name, .. }| name == TARGET_CHARACTER_NAME)
        .flat_map(|CharacterMeta { styles, .. }| styles)
        .find(|StyleMeta { name, .. }| name == TARGET_STYLE_NAME)
        .with_context(|| {
            format!("could not find \"{TARGET_CHARACTER_NAME} ({TARGET_STYLE_NAME})\"")
        })?;

    eprintln!("Synthesizing");
    let wav = &synth.tts(TEXT, style_id).perform()?;

    eprintln!("Playing the WAV");
    let mut file = File::create("test.wav")?;
    file.write_all(wav)?;

    Ok(())
}

fn play(wav: &[u8]) -> anyhow::Result<()> {
    let tempfile = tempfile::Builder::new().suffix(".wav").tempfile()?;
    (&tempfile).write_all(wav)?;
    let tempfile = &tempfile.into_temp_path();
    open::that_in_background(tempfile)
        .join()
        .unwrap_or_else(|e| panic::resume_unwind(e))?;
    Ok(())
}
