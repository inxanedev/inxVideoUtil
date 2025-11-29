use std::{path::PathBuf, process::Command};

fn create_timestamp(value: i64) -> String {
    let seconds = value / 1000;
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let ms = value % 1000;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, ms)
}

pub fn trim_and_compress(filename: &PathBuf, start: i64, end: i64, crf: u32) {
    let mut command = Command::new("ffmpeg");
    command
        .arg("-i")
        .arg(filename);

    if crf != 0 {
        command
            .arg("-c:v")
            .arg("libx264")
            .arg("-crf")
            .arg(crf.to_string());
    } else {
        command
            .arg("-c:v")
            .arg("copy");
    }

    command
        .arg("-ss")
        .arg(create_timestamp(start))
        .arg("-to")
        .arg(create_timestamp(end))
        .arg("-y")
        .arg("-c:a")
        .arg("copy")
        .arg(filename.with_file_name(format!("{}-compressed.{}", filename.file_stem().unwrap().display(), filename.extension().unwrap().display())));
    command.spawn().expect("ffmpeg failed").wait().expect("ffmpeg failed");
}