use metadata::media_file::MediaFileMetadata;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn subtitle_generator(text: &str, mp3_file_path: &str) -> io::Result<()> {
    // Read the duration of the MP3 file
    let duration = get_audio_duration(mp3_file_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Format the duration as hours:minutes:seconds,milliseconds
    let duration_str = format_duration(duration);

    // Create the ASS file content
    let ass_content = format!(
        "[Script Info]\n\
        ScriptType: v4.00+\n\
        \n\
        [V4+ Styles]\n\
        Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n\
        Style: Default, Noto Naskh Arabic, 18, &H00FFFFFF, &H000000FF, &H00000000, &H00000000, 0, 0, 0, 0, 100, 100, 0, 0, 1, 1, 0, 2, 10, 10, 10, 1\n\
        \n\
        [Events]\n\
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n\
        Dialogue: 0,0:00:00.00,{},Default,,0,0,0,,{}\n",
        duration_str, text
    );

    // Write to the ASS file
    let mut file = File::create("output.ass")?;
    file.write_all(ass_content.as_bytes())?;

    Ok(())
}

fn get_audio_duration(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = MediaFileMetadata::new(&Path::new(file_path))?;
    Ok(metadata.duration.unwrap().trim().into())
}

fn format_duration(duration: String) -> String {
    let parts: Vec<&str> = duration.split(':').collect();
    let inner_parts: Vec<&str> = parts[2].split('.').collect();
    format!(
        "{:02}:{:02}:{:02},{:0>3}",
        parts[0], parts[1], inner_parts[0], inner_parts[1]
    )
}
