use std::process::exit;
use chrono::{Duration, NaiveDate};
use eframe::egui;
use egui::*;
use eframe::Frame;
use egui_extras::DatePickerButton;
use diary_core::base::error::Error;
use diary_core::model::day::Day;
use crate::service::executor::GuiService;
use crate::model::date::Date;
use crate::model::day::GuiDayState;

pub struct App {
    executor: GuiService,
    day: GuiDayState,
    date_selected: NaiveDate,

    error: Option<Error>,
}
impl App {
    pub fn new(executor: GuiService) -> Self {
        let td = Date::default().into();
        // 初始化 day.date 为一个特殊值，保证第一次刷新会读取数据库
        let day = Day::default().with_date(Date::from(td-Duration::days(1)).into()).into();
        Self {
            executor,
            day,
            date_selected: td,
            error: None,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.set_visuals(Visuals::default());
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Close").clicked() {
                    exit(0);
                }
            })
        });
        SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            ui.add(DatePickerButton::new(&mut self.date_selected));
            if ui.add(Button::new("Commit")).clicked() {
                // Commit
                match self.executor.update_day(&self.day) {
                    Ok(_) => {
                        println!("{}, COMMIT COMPLETE", self.date_selected);
                    },
                    Err(e) => self.error = Some(e.into()),
                }
            }
            /*
            if ui.button("ERROR").clicked() {
                self.error = Some(Error::UnknownCommand("Error-Test".to_string()));
            }
             */
            ui.horizontal_wrapped(|ui| {
                if ui.add(Button::new("Before")).clicked() {
                    self.date_selected -= Duration::days(1);
                }
                if ui.add(Button::new("Next")).clicked() {
                    self.date_selected += Duration::days(1);
                }
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if ui.text_edit_multiline(&mut self.day.event.instruct).changed() {
                    // When input text, what happen?
                }
            });
        });
        Window::new("MISC")
            .default_pos(Pos2::new(0.0, 150.0))
            // .default_open(false)
            .show(ctx, |ui| {
                // Weather
                let weather_input = TextEdit::singleline(&mut self.day.weather)
                    .hint_text("Weather")
                    .desired_width(100.);
                ui.add(weather_input);

                // Mood
                ui.horizontal(|ui| {
                    let (face, color) = mood_to_face(self.day.mood);

                    let mood_input = DragValue::new(&mut self.day.mood)
                        .max_decimals(1)
                        .range(0..=10)
                        .speed(0.015)
                        .prefix("Mood: ");
                    ui.add(mood_input);
                    let face = Label::new(RichText::new(face).color(color).monospace());
                    let face_rec = ui.allocate_exact_size([47., 15.].into(), Sense::empty());
                    ui.put(face_rec.0, face)
                });
        });
        self.error_modal(ctx);
        self.update_day();
    }
}
impl App {
    fn update_day(&mut self) {
        let date = Date::from(self.date_selected).into();
        if self.day.date != date {
            self.day = self.executor.read_day(date).ok().unwrap_or_default()
                .unwrap_or(Day::default().with_date(date).into());
        }
    }
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
fn mood_to_face(mood: f32) -> (&'static str, Color32) {
    let (face, color) = match mood {
        v if v == 0.0 => ("(・_・?)", Color32::WHITE), // 未知

        0.0..=0.5 => ("(._.)", Color32::DARK_GRAY),
        0.5..=1.0 => ("(-_-)", Color32::GRAY),

        1.0..=1.5 => ("(>_<)", Color32::LIGHT_RED),
        1.5..=2.0 => ("(¬_¬)", Color32::LIGHT_RED),

        2.0..=2.5 => ("(・_・)", Color32::WHITE),
        2.5..=3.0 => ("(=_=)", Color32::WHITE),

        3.0..=3.5 => ("(´_ゝ`)", Color32::LIGHT_GREEN),
        3.5..=4.0 => ("(｀_´)", Color32::LIGHT_GREEN),

        4.0..=4.5 => ("(´ω`)", Color32::GREEN),
        4.5..=5.0 => ("(^_^)", Color32::GREEN),

        5.0..=5.5 => ("(＾_＾)", Color32::from_rgb(100, 200, 255)),
        5.5..=6.0 => ("(^_^)", Color32::from_rgb(100, 200, 255)),

        6.0..=6.5 => ("(≧ω≦)", Color32::from_rgb(120, 220, 200)),
        6.5..=7.0 => ("(≧▽≦)", Color32::from_rgb(120, 220, 200)),

        7.0..=7.5 => ("(≧∀≦)", Color32::from_rgb(180, 240, 180)),
        7.5..=8.0 => ("(☆▽☆)", Color32::from_rgb(180, 240, 180)),

        8.0..=8.5 => ("(*^_^*)", Color32::from_rgb(200, 255, 160)),
        8.5..=9.0 => ("(*≧▽≦*)", Color32::from_rgb(200, 255, 160)),

        9.0..=9.5 => ("(^o^)", Color32::from_rgb(220, 255, 120)),
        9.5..=10.0 => ("(＾▽＾)", Color32::from_rgb(220, 255, 120)),

        _ => ("(?)", Color32::GRAY),
    };

    (face, color)
}