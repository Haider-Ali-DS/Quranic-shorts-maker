use std::{error::Error, fs};

use helpers::*;

mod helpers;

pub const AUDIO_DIR: &str = "resources/audios";
pub const BG_DIR: &str = "resources/backgrounds";
pub const TEXT_DIR: &str = "resources/texts";
pub const FONT_DIR: &str = "resources/fonts";

pub enum AudioType {
    Arabic,
    English,
    Urdu,
}
pub enum TextType {
    Arabic,
    English,
    Urdu,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let surah = "1";
    let start_aya = "1";
    let end_aya = "7";
    let temp_file_path = "mp3files.txt";
    let concatenated_audio = "audio.mp3";
    let subtitle_path = "subtitle.ass";
    let input_image = format!("{}/desert.jpg", BG_DIR);

    let subtitles = get_full_audio_and_text(surah, start_aya, end_aya, temp_file_path);
    concatenate_mp3(temp_file_path, concatenated_audio).unwrap();
    make_subtitle_file(subtitles, subtitle_path);
    make_short(&input_image, subtitle_path, concatenated_audio);

    fs::remove_file(temp_file_path).unwrap();
    fs::remove_file(concatenated_audio).unwrap();
    fs::remove_file(subtitle_path).unwrap();
    Ok(())
}
