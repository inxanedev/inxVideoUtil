use std::process;

use eframe::egui::{self, Vec2};
use egui_video::Player;

use crate::{args::Args, ffmpeg};

pub struct InxVideoUtilApp {
    args: Args,
    player: Option<Player>,
    audio_device: egui_video::AudioDevice,
    start: i64,
    end: i64,
    crf: u32
}

impl InxVideoUtilApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, args: crate::args::Args) -> Self {
        Self {
            args,
            player: None,
            audio_device: egui_video::AudioDevice::new().expect("Failed to create AudioDevice"),
            start: 0, end: 0, crf: 30
        }
    } 
}

impl eframe::App for InxVideoUtilApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.label(format!("filename: {}", self.args.filename));
            if self.player.is_none() {
                self.player = Some(
                    Player::new(ctx, &self.args.filename.to_str().unwrap().to_owned())
                        .expect("Invalid video file path.")
                        .with_audio(&mut self.audio_device)
                        .expect("Failed to add AudioDevice to video player"));
                self.player.as_mut().unwrap().start();
                self.end = self.player.as_ref().unwrap().duration_ms;
            }
            let orig_width = self.player.as_ref().unwrap().size.x;
            let orig_height = self.player.as_ref().unwrap().size.y;

            let scale_w = 650.0 / orig_width;
            let scale_h = 450.0 / orig_height;

            let scale = f32::min(scale_w, scale_h);

            let width = orig_width * scale;
            let height = orig_height * scale;

            self.player.as_mut().unwrap().ui(ui, Vec2::new(width, height));
            
            ui.horizontal(|ui| {
                if ui.button("Set start").clicked() {
                    self.start = self.player.as_ref().unwrap().elapsed_ms()
                }
                if ui.button("Set end").clicked() {
                    self.end = self.player.as_ref().unwrap().elapsed_ms();
                }
                if ui.button("Trim and compress").clicked() {
                    ffmpeg::trim_and_compress(&self.args.filename, self.start, self.end, self.crf);
                    process::exit(0);
                }
            });

            ui.label(format!("Start pos: {}", self.start));
            ui.label(format!("End pos: {}", self.end));
            ui.add(egui::Slider::new(&mut self.crf, 0..=30).text("CRF"));
        });
    }
}

