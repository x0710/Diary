#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod service;
mod model;
use std::sync::Arc;
use eframe::{run_native, NativeOptions};
use eframe::egui::{FontData, FontDefinitions, IconData, ViewportBuilder, Visuals};
use eframe::epaint::FontFamily;
use diary_core::base::executor::Executor;
use crate::app::app::App;
use crate::service::executor::GuiService;

fn main() -> eframe::Result {// 16x16 图标
    let dbmgr = diary_core::base::env::open_with_default_database()
        .expect("Could not open database");
    let exec = Executor::from(dbmgr);
    let exec = GuiService::new(exec);
    const WIDTH: u32 = 16;
    const HEIGHT: u32 = 16;

    // RGBA 数据
    let mut rgba: Vec<u8> = Vec::with_capacity(WIDTH as usize * HEIGHT as usize * 4);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // 红色渐变 + 半透明
            let r = (x * 255 / (WIDTH - 1)) as u8;
            let g = (y * 255 / (HEIGHT - 1)) as u8;
            let b = 128;
            let a = 255;
            rgba.extend_from_slice(&[r, g, b, a]);
        }
    }
    let icon = IconData {
        rgba,
        width: WIDTH,
        height: HEIGHT,
    };
    let no = NativeOptions {
        viewport: ViewportBuilder::default().with_icon(icon),
        ..Default::default()
    };
    run_native("Diary", no, Box::new(|x| {
        x.egui_ctx.set_visuals(Visuals::default());
        let mut fd = FontDefinitions::default();
        fd.font_data.insert(
            "CN-F".to_string(),
            Arc::new(FontData::from_static(include_bytes!("../assets/LXGWWenKaiMonoLite-Regular.ttf"))),
        );
        egui_extras::install_image_loaders(&x.egui_ctx);
        fd.families.get_mut(&FontFamily::Monospace).unwrap()
            .insert(0, "CN-F".to_string());
        fd.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "CN-F".to_string());
        x.egui_ctx.set_fonts(fd);
        Ok(Box::new(App::new(exec)))
    }))
}