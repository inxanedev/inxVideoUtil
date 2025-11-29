use std::process;

use eframe::egui::{self, Pos2, Rounding, Vec2};
use egui_video::Player;

use crate::{args::Args, ffmpeg};

pub struct InxVideoUtilApp {
    args: Args,
    player: Option<Player>,
    audio_device: egui_video::AudioDevice,
    start: i64,
    end: i64,
    crf: u32,
    cropping: bool,
    rect_color: egui::Color32,
    rect_start: Option<egui::Pos2>,
    rect_end: Option<egui::Pos2>
}

impl InxVideoUtilApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, args: crate::args::Args) -> Self {
        Self {
            args,
            player: None,
            audio_device: egui_video::AudioDevice::new().expect("Failed to create AudioDevice"),
            start: 0, end: 0, crf: 30, cropping: false,
            rect_color: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 100),
            rect_start: None,
            rect_end: None
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
            
            let player_pos = ui.next_widget_position();
            self.player.as_mut().unwrap().ui(ui, Vec2::new(width, height));

            ui.input(|i| {
                if i.pointer.any_click() && self.cropping {
                    let pos = i.pointer.interact_pos().unwrap();
                    if !(
                        pos.x >= player_pos.x && pos.x <= player_pos.x + width &&
                        pos.y >= player_pos.y && pos.y <= player_pos.y + height) {
                            return;
                        }
                    if self.rect_start.is_none() {
                        self.rect_start = Some(pos);
                    } else if self.rect_end.is_none() {
                        if pos.x >= self.rect_start.unwrap().x && pos.y >= self.rect_start.unwrap().y {
                            self.rect_end = Some(pos);
                            self.cropping = false;
                        }
                    }
                }
            });

            if self.rect_start.is_some() && self.rect_end.is_some() {
                let painter = ui.painter();
                let rect = egui::Rect::from_min_size(
                    self.rect_start.unwrap(),
                    egui::vec2(
                        self.rect_end.unwrap().x - self.rect_start.unwrap().x,
                        self.rect_end.unwrap().y - self.rect_start.unwrap().y
                    )
                );
                let shape = egui::Shape::rect_filled(
                    rect, Rounding::ZERO, self.rect_color
                );
                painter.add(shape);
            }
            
            ui.horizontal(|ui| {
                if !self.cropping && ui.button("Crop").clicked() {
                    self.cropping = true;
                }
                if self.cropping { return };
                if ui.button("Set start").clicked() {
                    self.start = self.player.as_ref().unwrap().elapsed_ms()
                }
                if ui.button("Set end").clicked() {
                    self.end = self.player.as_ref().unwrap().elapsed_ms();
                }
                if ui.button("Trim and compress").clicked() {
                    let mut video_rect_start = None;
                    let crop_rect_size = if let Some(rect_end) = self.rect_end {
                        video_rect_start = Some(Pos2::new(
                            (self.rect_start.unwrap().x - player_pos.x) / scale,
                            (self.rect_start.unwrap().y - player_pos.y) / scale
                        ));
                        Some(egui::vec2(
                            (rect_end.x - self.rect_start.unwrap().x) / scale,
                            (rect_end.y - self.rect_start.unwrap().y) / scale
                        ))
                    } else {None};

                    ffmpeg::trim_and_compress(
                        &self.args.filename,
                        self.start,
                        self.end,
                        self.crf,
                        video_rect_start,
                        crop_rect_size
                    );
                    process::exit(0);
                }
            });

            ui.label(format!("Start pos: {}", self.start));
            ui.label(format!("End pos: {}", self.end));
            ui.add(egui::Slider::new(&mut self.crf, 0..=30).text("CRF"));
        });
    }
}

