use eframe::egui::Color32;

pub fn mood_to_face(mood: f32) -> (&'static str, Color32) {
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
