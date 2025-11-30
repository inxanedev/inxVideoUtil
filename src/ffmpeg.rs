use std::{path::{PathBuf}, process::Command};

use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{egui::{Pos2, Vec2}};
use percent_encoding::{AsciiSet, CONTROLS};
use reqwest::blocking::{Client, multipart};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult};
use sha2::{Digest, Sha256};

fn create_timestamp(value: i64) -> String {
    let seconds = value / 1000;
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let ms = value % 1000;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, ms)
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn upload_file(filename: PathBuf, password: &String) {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let hash = hex::encode(result);
    let file_part = multipart::Part::file(&filename).expect("Couldn't load the file for uploading")
        .file_name(filename.to_string_lossy().into_owned());

    let form = multipart::Form::new()
        .text("hash", hash)
        .part("file", file_part);

    let client = Client::new();
    let url = "https://inxane.dev/upload";

    let response = client.post(url).multipart(form).send().expect("Error sending network request");

    if !response.status().is_success() {
        eprintln!("Server's response was an error: {}\n{}", response.status(), response.text().unwrap());
        return;
    }

    let body = response.text().unwrap();
    let encoded = percent_encoding::percent_encode(body.as_bytes(), FRAGMENT).to_string();

    let dialog_result = MessageDialog::new()
        .set_title("inxVideoUtil")
        .set_description(format!("File uploaded. URL:\n\n{}\n\nCopy to clipboard?", encoded))
        .set_buttons(MessageButtons::YesNo)
        .set_level(rfd::MessageLevel::Info)
        .show();

    match dialog_result {
        MessageDialogResult::Yes => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to acquire clipboard context");
            ctx.set_contents(encoded).expect("Failed to set clipboard contents");
        },
        _ => {}
    };
}

pub fn trim_and_compress(filename: &PathBuf, start: i64, end: i64, crf: u32, crop_rect_pos: Option<Pos2>, crop_rect_size: Option<Vec2>, upload: bool, password: Option<&String>) {
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
    } else if let Some(_) = crop_rect_pos {
        command
            .arg("-c:v")
            .arg("libx264");
    } else {
        command
            .arg("-c:v")
            .arg("copy");
    }

    if let Some(rect_pos) = crop_rect_pos && let Some(rect_size) = crop_rect_size {
        command
            .arg("-vf")
            .arg(
                format!("crop={}:{}:{}:{}",
                    rect_size.x, rect_size.y, rect_pos.x, rect_pos.y
                )
            );
    }

    let output_path = filename.with_file_name(format!("{}-inxvu.{}", filename.file_stem().unwrap().display(), filename.extension().unwrap().display()));

    command
        .arg("-ss")
        .arg(create_timestamp(start))
        .arg("-to")
        .arg(create_timestamp(end))
        .arg("-y")
        .arg("-c:a")
        .arg("copy")
        .arg(&output_path);

    println!("{:?}", command);
    command.spawn().expect("ffmpeg failed").wait().expect("ffmpeg failed");

    if upload {
        upload_file(output_path, password.unwrap());
    }
}