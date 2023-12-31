use std::{
    fs::{self, File},
    io::BufReader,
    process::Command,
};

use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

use crate::audio_helper::*;

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
    let title = "drawtext=text=Al-Quran:fontcolor=white:fontsize=60:x=(w-text_w)/2:y=80";
    let sub_title = "drawtext=text=Translation:fontcolor=white:fontsize=40:x=(w-text_w)/2:y=135";
    let seperator =
        "drawtext=text=❀----------❤----------❀:fontcolor=white:fontsize=30:x=(w-text_w)/2:y=175";
    let _ = subtitle_generator(text, audio_path);
    let subtitle_filter = "subtitles=filename=output.ass:fontsdir=resources/fonts/";
    Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-i",
            bg_image,
            "-i",
            audio_path,
            "-vf",
            &format!("{},{},{},{}", title, sub_title, seperator, subtitle_filter),
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-pix_fmt",
            "yuv420p",
            "-shortest",
            output_path,
        ])
        .output()
        .expect("Failed to execute FFmpeg command");
    let _ = fs::remove_file("output.ass");
}
