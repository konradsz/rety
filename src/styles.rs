use egui::{Color32, Rounding};

pub const FONT_SIZE: f32 = 14.0;

pub const DEFAULT_MATCH_COLOR: Color32 = Color32::from_rgb(241, 171, 134);

pub const BACKGROUND: Color32 = Color32::from_rgb(35, 43, 47); // #232B2F

pub const TEXT_EDIT_BACKGROUND: Color32 = Color32::from_rgb(26, 31, 35);
pub const TEXT_EDIT_ACTIVE_STROKE: Color32 = Color32::YELLOW;
pub const TEXT_EDIT_INACTIVE_STROKE: Color32 = Color32::LIGHT_RED;
pub const TEXT_EDIT_ROUNDING: Rounding = Rounding {
    nw: 5.0,
    ne: 5.0,
    sw: 5.0,
    se: 5.0,
};

pub const CORRECT_PATTERN_STROKE_COLOR: Color32 = Color32::from_rgb(4, 74, 0);
pub const CORRECT_PATTERN_BG_COLOR: Color32 = Color32::from_rgb(2, 35, 0);
pub const INCORRECT_PATTERN_STROKE_COLOR: Color32 = Color32::from_rgb(90, 0, 19);
pub const INCORRECT_PATTERN_BG_COLOR: Color32 = Color32::from_rgb(45, 0, 10);

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
