use std::process::exit;
use chrono::NaiveDate;
use eframe::egui;
use egui::*;
use eframe::Frame;
use egui_extras::DatePickerButton;
use diary_core::base::error::Error;
use diary_core::base::executor::Executor;
use crate::model::date::Date;

pub struct App {
    executor: Executor,
    editor_text: String,
    date_selected: NaiveDate,

    error: Option<Error>

}
impl App {
    pub fn new(executor: Executor) -> Self {
        let td = Date::default();
        Self {
            executor,
            editor_text: String::new(),
            date_selected: td.into(),
            error: None,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_visuals(Visuals::default());
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.button("Close").clicked() {
                    exit(0);
                }
            })
        });
        SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.add(DatePickerButton::new(&mut self.date_selected)).changed() {
                let day = self.executor.conn().read_day(
                    Date::from(self.date_selected).into()
                ).expect("Error reading date from database");
                self.editor_text = day.unwrap_or_default().to_string();
            }
            if ui.button("ERROR").clicked() {
                self.error = Some(Error::UnknownCommand("sd".to_string()));
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                let tem = ui.text_edit_multiline(&mut self.editor_text);
            });
        });

        self.error_modal(ctx);
    }
}
impl App {
    fn error_modal(&mut self, ctx: &Context) {
        if self.error.is_none() {return}
        let scn_rec = ctx.content_rect();
        // block
        Area::new(egui::Id::from("err_modal"))
            .order(Order::Foreground)
            .fixed_pos(scn_rec.min)
            .show(ctx, |ui| {
                ui.set_min_size(scn_rec.size());
                ui.allocate_rect(scn_rec, Sense::all());

                ui.painter().rect_filled(
                    scn_rec,
                    0.,
                    Color32::from_black_alpha(130),
                );
            });

        Window::new("Error")
            .resizable(false)
            .collapsible(false)
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(format!("{}", self.error.as_ref().unwrap()));
                if ui.button("OK").clicked() {
                    self.error.take();
                }
        });
    }
}