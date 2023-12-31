use std::error::Error;

use helpers::*;

mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let input_text =
    //     "this is a text with a text and this text is good very good too good awesome good";
    let input_text = read_arabic_quran("1", "1");
    let input_image = "bg_images/water_bg.jpg";
    // let file_name = download_ayat_translation("1", "1:1").await.unwrap();
    make_empty_video(input_image);
    add_text_in_image(&input_text);
    Ok(())
}
