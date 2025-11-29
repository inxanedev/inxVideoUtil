use clap::Parser;
use eframe::egui;
use inx_video_util::{args, ui};

fn main() -> eframe::Result<()> {
    let args = args::Args::parse();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native("inxVideoUtil", options, Box::new(|cc| Ok(Box::new(ui::InxVideoUtilApp::new(cc, args)))))
}
