use egui::{Color32, Rounding};

pub const FONT_SIZE: f32 = 14.0;

pub const DEFAULT_MATCH_COLOR: Color32 = Color32::from_rgb(130, 163, 161);

pub const BACKGROUND: Color32 = Color32::from_rgb(35, 43, 47); // #232B2F

pub const TEXT_COLOR: Color32 = Color32::from_rgb(212, 212, 212);
pub const TEXT_COLOR_MATCHED: Color32 = Color32::from_rgb(35, 43, 47);

pub const TEXT_EDIT_BACKGROUND: Color32 = Color32::from_rgb(26, 31, 35);
pub const TEXT_EDIT_ACTIVE_STROKE: Color32 = Color32::from_rgb(85, 99, 107);
pub const TEXT_EDIT_INACTIVE_STROKE: Color32 = Color32::from_rgb(61, 75, 81);
pub const TEXT_EDIT_ROUNDING: Rounding = Rounding {
    nw: 5.0,
    ne: 5.0,
    sw: 5.0,
    se: 5.0,
};

pub const CORRECT_PATTERN_ACTIVE_STROKE_COLOR: Color32 = Color32::from_rgb(18, 102, 14);
pub const CORRECT_PATTERN_INACTIVE_STROKE_COLOR: Color32 = Color32::from_rgb(4, 74, 0);
pub const CORRECT_PATTERN_BG_COLOR: Color32 = Color32::from_rgb(2, 35, 0);
pub const INCORRECT_PATTERN_ACTIVE_STROKE_COLOR: Color32 = Color32::from_rgb(122, 17, 40);
pub const INCORRECT_PATTERN_INACTIVE_STROKE_COLOR: Color32 = Color32::from_rgb(90, 0, 19);
pub const INCORRECT_PATTERN_BG_COLOR: Color32 = Color32::from_rgb(53, 6, 16);

// TODO: cursor color

pub const COLORS: [Color32; 10] = [
    Color32::from_rgb(130, 163, 161),
    Color32::from_rgb(145, 245, 173),
    Color32::from_rgb(243, 255, 198),
    Color32::from_rgb(234, 210, 172),
    Color32::from_rgb(171, 218, 252),
    Color32::from_rgb(66, 129, 164),
    Color32::from_rgb(196, 144, 209),
    Color32::from_rgb(154, 76, 149),
    Color32::from_rgb(254, 147, 140),
    Color32::from_rgb(235, 94, 40),
];
