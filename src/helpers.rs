use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
    process::Command,
};

use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

pub enum TextType {
    Arabic,
    Urdu,
    English,
}

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
    let file = File::open("resources/text/urdu.xml").unwrap();
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

pub fn make_short(bg_image: &str, text: &str, audio_path: &str) {
    let output_path = "generated-videos/output.mp4";
    let draw_text_filter = &format!(
                "drawtext=text={}:fontfile=resources/fonts/{}:fontcolor=white:fontsize=50:x=(w-text_w)/2:y=(h-text_h)/2",
                wrap_text(text, 30),
                "arabic.ttf"
            );
    Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-i",
            bg_image,
            "-i",
            audio_path,
            "-shortest",
            "-vf",
            draw_text_filter,
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-pix_fmt",
            "yuv420p",
            output_path,
        ])
        .output()
        .expect("Failed to execute FFmpeg command");
}
