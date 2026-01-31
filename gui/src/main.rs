use std::sync::Arc;
use eframe::{run_native, NativeOptions};
use eframe::egui::{FontData, FontDefinitions, IconData, ViewportBuilder, Visuals};
use eframe::epaint::FontFamily;
use diary_core::base::executor::Executor;
use diary_core::storage::db_mgr::DatabaseManager;
use diary_gui::app::app::App;
use diary_gui::service::executor::GuiService;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() -> eframe::Result {// 16x16 图标
    let prjdir = directories::ProjectDirs::from("x0710", "x0710", "diary")
        .expect("Could not find a valid home directory");
    let db_path = prjdir.data_dir().join("diary.db");
    std::fs::create_dir_all(prjdir.data_dir())
        .expect("Could not create data directory");
    if !db_path.exists() {
        std::fs::File::create(&db_path)
            .expect("Could not create database file");
    }
    let exec = Executor::from(DatabaseManager::from_path(&db_path)
        .expect("Could not load the database"));
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