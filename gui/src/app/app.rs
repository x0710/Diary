use std::process::exit;
use chrono::{Duration, NaiveDate};
use eframe::egui;
use egui::*;
use eframe::Frame;
use egui_extras::DatePickerButton;
use diary_core::base::error::Error;
use diary_core::model::day::Day;
use diary_core::storage::io::export::Exporter;
use diary_core::storage::io::import::{DuplicateStrategy, Importer};
use diary_core::storage::io::mode::Format::JSON;
use crate::app::component::face::mood_to_face;
use crate::service::executor::GuiService;
use crate::model::date::Date;
use crate::model::day::GuiDayState;

pub struct App {
    executor: GuiService,
    day: GuiDayState,
    date_selected: NaiveDate,

    error: Option<Error>,

    in_about_page: bool,
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
            in_about_page: false,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            MenuBar::new() .ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.separator();
                    if ui.button("Export").clicked() {
                        if let Some(folder) = rfd::FileDialog::new().save_file() {
                            let mut exp = Exporter::new(self.executor.executor.conn_mut(), folder, JSON);
                            self.error = exp.all_export().err();
                        }else {
                            eprintln!("Please select a folder");
                        }
                    }
                    if ui.button("Import").clicked() {
                        if let Some(file) = rfd::FileDialog::new().pick_file() {
                            let mut imp = Importer::new(self.executor.executor.conn_mut());
                            match Importer::read_from_file(file, JSON) {
                                Ok(r) => {
                                    let mut ers = String::new();
                                    r.1.iter().for_each(|x| ers.push_str(x));
                                    if !ers.is_empty() {
                                        self.error = Some(Error::InvalidDate(ers.into()));
                                    }
                                    self.error = imp.import_to_db(r.0, DuplicateStrategy::Replace).err();
                                }
                                Err(err) => self.error = Some(err),
                            }
                        }
                    }
                });
                // ui.menu_button("Settings", |ui| {});
                ui.menu_button("Help", |ui| {
                    if ui.button("About Diary").clicked() {
                        self.in_about_page = true;
                    }
                });
            })
        });
        SidePanel::left("side_panel").resizable(false)
            .min_width(130.)
            .show(ctx, |ui| {
                ui.add(DatePickerButton::new(&mut self.date_selected));
                ui.horizontal_wrapped(|ui| {
                    if ui.add(Button::new("Commit")).clicked() {
                        match self.executor.update_day(&self.day) {
                            Ok(_) => {println!("{}, COMMIT COMPLETE", self.date_selected)},
                            Err(e) => self.error = Some(e.into()),
                        }
                    }
                    if ui.add(Button::new("Exit")).clicked() {
                        exit(0);
                    }
                });
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
                    let face = Label::new(RichText::new(face).color(color));
                    let face_rec = ui.allocate_exact_size([65., 15.].into(), Sense::empty());
                    ui.put(face_rec.0, face)
                });
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
                ScrollArea::both()
                    .auto_shrink([true; 2])
                    .show(ui, |ui| {
                    if ui.text_edit_multiline(&mut self.day.event.instruct).changed() {
                        // When input text, what happen?
                    }
                })
            });
        });
        self.may_modal(ctx);
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
    /// enter modal mode
    fn open_modal(&mut self, ctx: &Context) {
        let scn_rec = ctx.content_rect();
        // block
        Area::new(egui::Id::from("modal"))
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
    }
    fn may_modal(&mut self, ctx: &Context) {
        self.error_modal(ctx);
        self.about_modal(ctx);
    }
    fn about_modal(&mut self, ctx: &Context) {
        if !self.in_about_page {return}
        Window::new("About")
            .resizable(false)
            .collapsible(false)
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Diary");
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let cur_rec = ui.available_rect_before_wrap();
                            let github_icon = Image::new(include_image!("../../assets/github-mark-white.svg"));
                            let icon_rec = Rect::from_min_size(
                                [cur_rec.right()-55., cur_rec.top()+10.].into(),
                                [50., 50.].into(),
                            );

                            if ui.put(icon_rec, ImageButton::new(github_icon).frame(false)).clicked() {
                                println!("Clicked!");
                                ctx.open_url(OpenUrl::new_tab("https://github.com/x0710/Diary"))
                            }

                        });
                    });
                    ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).weak());
                    ui.separator();
                    ui.add_space(4.0);
                    ui.label("A high-performance tool built with Rust and egui.");
                    ui.label("BTW, it's always used in writing diary.");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Created by");
                        ui.hyperlink_to("JinhangGao", "https://github.com/x0710");
                    });
                    ui.add_space(20.0);
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        if ui.button("  OK  ").clicked() {
                            self.in_about_page = false;
                        }
                    });
                });
            });
    }
    fn error_modal(&mut self, ctx: &Context) {
        if self.error.is_none() {return}
        self.open_modal(ctx);

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