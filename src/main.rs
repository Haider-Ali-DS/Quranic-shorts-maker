use clap::{Parser, ValueEnum};
use helpers::*;
use std::str::FromStr;
use std::{error::Error, fs};

mod helpers;

pub const AUDIO_DIR: &str = "resources/audios";
pub const BG_DIR: &str = "resources/backgrounds";
pub const TEXT_DIR: &str = "resources/texts";
pub const FONT_DIR: &str = "resources/fonts";

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub surah: String,
    #[clap(short, long)]
    pub start_aya: String,
    #[clap(short, long)]
    pub end_aya: String,
    #[clap(value_enum, short, long, default_value_t = AudioType::Arabic)]
    pub audio_type: AudioType,
    #[clap(value_enum, short, long, default_value_t= TextType::None)]
    pub text_type: TextType,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum AudioType {
    Arabic,
    English,
    Urdu,
}

impl FromStr for AudioType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "arabic" => Ok(AudioType::Arabic),
            "english" => Ok(AudioType::English),
            "urdu" => Ok(AudioType::Urdu),
            _ => Err("no match"),
        }
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TextType {
    Arabic,
    English,
    Urdu,
    None,
}

impl FromStr for TextType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "arabic" => Ok(TextType::Arabic),
            "english" => Ok(TextType::English),
            "urdu" => Ok(TextType::Urdu),
            "none" => Ok(TextType::None),
            _ => Err("no match"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let surah = "1";
    let start_aya = "1";
    let end_aya = "1";
    let text_type = TextType::Urdu;
    let audio_type = AudioType::Arabic;

    let temp_file_path = "mp3files.txt";
    let concatenated_audio = "audio.mp3";
    let subtitle_path = "subtitle.ass";
    let input_image = format!("{}/desert.jpg", BG_DIR);
    let surah_title = get_surah_title(surah);

    let text_file = match text_type {
        TextType::Arabic => Some("arabic.xml"),
        TextType::English => Some("english.xml"),
        TextType::Urdu => Some("urdu.xml"),
        TextType::None => None,
    };

    let audio_folder = match audio_type {
        AudioType::Arabic => "arabic",
        AudioType::English => "english",
        AudioType::Urdu => "urdu",
    };

    let subtitles = get_full_audio_and_text(
        surah,
        start_aya,
        end_aya,
        text_file,
        audio_folder,
        temp_file_path,
    );
    concatenate_mp3(temp_file_path, concatenated_audio).unwrap();
    make_subtitle_file(subtitles, subtitle_path);
    make_short(
        &input_image,
        &surah_title,
        start_aya,
        end_aya,
        subtitle_path,
        concatenated_audio,
    );

    fs::remove_file(temp_file_path).unwrap();
    fs::remove_file(concatenated_audio).unwrap();
    fs::remove_file(subtitle_path).unwrap();
    Ok(())
}
