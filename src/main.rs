use std::error::Error;

use helpers::*;

mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let surah = "1";
    let aya = "1";
    let audio_path = format!("resources/audios/arabic/{:0>3}{:0>3}.mp3", surah, aya);
    let input_text = read_arabic_quran(surah, aya);
    let input_image = "resources/backgrounds/desert.jpg";
    make_short(input_image, &input_text, &audio_path);
    Ok(())
}
