use egui::Color32;

pub const FONT_SIZE: f32 = 13.0;

pub const DEFAULT_MATCH_COLOR: Color32 = Color32::from_rgb(241, 171, 134);

pub const BACKGROUND: Color32 = Color32::from_rgb(35, 43, 47); // #232B2F
pub const TEXT_EDIT_BACKGROUND: Color32 = Color32::from_rgb(26, 31, 35);
pub const TEXT_EDIT_ACTIVE_STROKE: Color32 = Color32::YELLOW;
pub const TEXT_EDIT_INACTIVE_STROKE: Color32 = Color32::LIGHT_RED;

pub const CORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_GREEN; // TODO: define
pub const INCORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_RED; // TODO: define

// TODO: cursor color

pub const COLORS: [Color32; 10] = [
    Color32::LIGHT_BLUE,
    Color32::LIGHT_RED,
    Color32::LIGHT_GREEN,
    Color32::LIGHT_YELLOW,
    Color32::LIGHT_GRAY,
    Color32::DARK_BLUE,
    Color32::DARK_RED,
    Color32::DARK_GREEN,
    Color32::BROWN,
    Color32::GOLD,
];
