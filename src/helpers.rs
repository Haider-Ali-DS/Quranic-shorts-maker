use std::{
    fs::{self, File},
    io::{self, BufReader, Write},
    process::Command,
    time::Duration,
};

use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

use crate::{AUDIO_DIR, TEXT_DIR};

#[derive(Debug)]
pub struct SubtitleData {
    pub start_time: String,
    pub end_time: String,
    pub text: String,
}

pub fn read_arabic_quran(
    sura_index_target: &str,
    aya_index_target: &str,
    file_name: Option<&str>,
) -> String {
    let Some(file_name) = file_name else {
        return String::new();
    };
    let file = File::open(format!("{}/{}", TEXT_DIR, file_name)).unwrap();
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

pub fn get_surah_title(surah_index: &str) -> String {
    let file = File::open(format!("{}/metadata.xml", TEXT_DIR)).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut surah_name = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "sura" {
                    let mut is_index_matched = false;

                    for attr in &attributes {
                        if attr.name.local_name == "index" && attr.value == surah_index {
                            is_index_matched = true;
                        }

                        if is_index_matched && attr.name.local_name == "tname" {
                            surah_name = attr.value.clone();
                            break;
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
    surah_name
}

fn get_attribute_value(attributes: &[OwnedAttribute], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == name)
        .map(|attr| attr.value.clone())
}

pub fn get_full_audio_and_text(
    surah: &str,
    start_aya: &str,
    end_aya: &str,
    text_file: Option<&str>,
    audio_type: &str,
    temp_file_path: &str,
) -> Vec<SubtitleData> {
    let start = start_aya.parse::<u16>().unwrap();
    let end = end_aya.parse::<u16>().unwrap();
    let mut file = File::create(temp_file_path).unwrap();
    let mut subtitle_timestamps: Vec<SubtitleData> = vec![];
    let mut current_start_time = Duration::new(0, 0);

    for i in start..=end {
        let audio_path = format!("{}/{}/{:0>3}{:0>3}.mp3", AUDIO_DIR, audio_type, surah, i);
        writeln!(file, "file '{}'", audio_path).unwrap();

        let text = read_arabic_quran(surah, &format!("{}", i), text_file);
        let duration_f64 = get_audio_duration(&audio_path).unwrap();
        let duration = parse_duration(duration_f64);
        let end_time = duration;

        subtitle_timestamps.push(SubtitleData {
            start_time: format_duration(current_start_time),
            end_time: format_duration(end_time),
            text,
        });

        current_start_time = current_start_time + end_time;
    }
    subtitle_timestamps
}

fn parse_duration(duration_secs: f64) -> Duration {
    let whole_seconds = duration_secs.trunc() as u64;
    let nanoseconds = ((duration_secs.fract() * 1_000_000_000.0) as u32) % 1_000_000_000;

    Duration::new(whole_seconds, nanoseconds)
}

fn format_duration(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    let milliseconds = duration.subsec_millis();

    format!(
        "{:02}:{:02}:{:02}.{}",
        hours,
        minutes,
        seconds,
        milliseconds
            .to_string()
            .get(0..2)
            .unwrap_or("00")
            .to_string()
    )
}

fn get_audio_duration(file_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()?;

    let binding = String::from_utf8(output.stdout)?;
    let duration_str = binding.trim();
    let duration = duration_str.parse::<f64>()?;
    Ok(duration)
}

pub fn concatenate_mp3(list_file_path: &str, output_path: &str) -> io::Result<()> {
    Command::new("ffmpeg")
        .args([
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            list_file_path,
            "-c",
            "copy",
            output_path,
        ])
        .output()
        .expect("Failed to execute FFmpeg command");

    Ok(())
}

pub fn make_subtitle_file(subtitles: Vec<SubtitleData>, output: &str) {
    let mut dialogues = String::new();
    for subtitle in subtitles {
        let dialogue = format! {
            "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
            subtitle.start_time, subtitle.end_time, subtitle.text
        };
        dialogues.push_str(&dialogue);
    }
    let ass_content = format!(
        "[Script Info]\n\
        ScriptType: v4.00+\n\
        \n\
        [V4+ Styles]\n\
        Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n\
        Style: Default, Noto Naskh Arabic, 18, &H00FFFFFF, &H000000FF, &H00000000, &H00000000, 0, 0, 0, 0, 100, 100, 0, 0, 1, 1, 0, 5, 10, 10, 10, 1\n\
        \n\
        [Events]\n\
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n\
        {}",
        dialogues
    );

    // Write to the ASS file
    let mut file = File::create(output).unwrap();
    file.write_all(ass_content.as_bytes()).unwrap();
}

pub fn make_short(
    bg_image: &str,
    surah_title: &str,
    start_aya: &str,
    end_aya: &str,
    subtitle_path: &str,
    audio_path: &str,
) {
    let output_path = "generated-videos/output.mp4";
    let title = "drawtext=text=Al-Quran:fontfile=resources/fonts/english-bold.ttf:fontcolor=white:fontsize=60:x=(w-text_w)/2:y=80";
    let surah_title = format!("drawtext=text={} {}-{}:fontfile=resources/fonts/english.ttf:fontcolor=white:fontsize=38:x=(w-text_w)/2:y=135", surah_title, start_aya, end_aya);
    let sub_title = "drawtext=text=Translation:fontfile=resources/fonts/english.ttf:fontcolor=white:fontsize=35:x=(w-text_w)/2:y=175";
    let seperator =
        "drawtext=text=❀----------❤----------❀:fontfile=resources/fonts/english.ttf:fontcolor=white:fontsize=30:x=(w-text_w)/2:y=215";
    let subtitle_filter = format!(
        "subtitles=filename={}:fontsdir=resources/fonts/",
        subtitle_path
    );
    let output = Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-i",
            bg_image,
            "-i",
            audio_path,
            "-vf",
            &format!(
                "{},{},{},{},{}",
                title, surah_title, sub_title, seperator, subtitle_filter
            ),
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
    println!("output {:?}", output);
    let _ = fs::remove_file("output.ass");
}
