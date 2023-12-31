use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
    process::Command,
};

use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

pub fn wrap_text(text: &str, max_width: usize) -> String {
    let mut wrapped = String::new();
    let mut line_length = 0;

    for word in text.split_whitespace() {
        let word_length = word.chars().count();

        // Check if adding the next word exceeds the max line width
        if line_length + word_length > max_width {
            wrapped.push('\n');
            line_length = 0;
        }

        wrapped.push_str(word);
        wrapped.push(' ');
        line_length += word_length + 1;
    }

    wrapped.trim_end().to_string()
}

pub fn read_arabic_quran(sura_index_target: &str, aya_index_target: &str) -> String {
    let file = File::open("resources/quran.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut in_sura = false;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "sura" {
                    in_sura = get_attribute_value(&attributes, "index")
                        == Some(sura_index_target.to_string());
                }
                if name.local_name == "aya" && in_sura {
                    if get_attribute_value(&attributes, "index")
                        == Some(aya_index_target.to_string())
                    {
                        if let Some(aya_text) = get_attribute_value(&attributes, "text") {
                            return aya_text;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    "".into()
}

fn get_attribute_value(attributes: &[OwnedAttribute], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == name)
        .map(|attr| attr.value.clone())
}
pub async fn download_ayat_translation(
    recitation_id: &str,
    ayat_key: &str,
) -> Result<String, Box<dyn Error>> {
    let file_name = format!("{}.mp3", ayat_key);
    let url = format!(
        "https://api.quran.com/api/v4/recitations/{}/by_ayah/{}",
        recitation_id, ayat_key
    );
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        println!("response {:#?}", response);
        let bytes = response.bytes().await?;
        let mut file = File::create(file_name.clone())?;
        file.write_all(&bytes)?;
        println!("Saved to output.mp3");
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }
    Ok(file_name)
}

pub fn make_empty_video(bg_image: &str) {
    let output_path = "generated-videos/output.mp4";
    Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-i",
            &format!("{}", bg_image),
            "-c:v",
            "libx264",
            "-t",
            "10",
            "-pix_fmt",
            "yuv420p",
            &format!("{}", output_path),
        ])
        .output()
        .expect("Failed to execute FFmpeg command");
}

pub fn add_text_in_image(text: &str) {
    let input_video = "generated-videos/output.mp4";
    let output_video = "generated-videos/output1.mp4";
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            &format!("{}", input_video),
            "-vf",
            &format!(
                "drawtext=text={}:fontfile=resources/arabic.ttf:fontcolor=white:fontsize=50:x=(w-text_w)/2:y=(h-text_h)/2",
                wrap_text(text, 30)
            ),
            "-codec:a",
            "copy",
            &format!("{}", output_video),
        ])
        .output()
        .expect("Failed to execute FFmpeg command");
    println!("{:?}", output);
}
