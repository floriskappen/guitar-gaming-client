use bevy::prelude::*;

pub const TIMELINE_LENGTH: f32 = 30.0;

// Z coordinate
pub const STRING_CENTERS: [f32; 6] = [
    0.0,
    0.5,
    1.0,
    1.5,
    2.0,
    2.5
];

pub const STRING_COLORS: [Color; 6] = [
    Color::srgb(1.0, 0.2, 0.2),
    Color::srgb(0.2, 1.0, 0.2),
    Color::srgb(1.0, 1.0, 0.2),
    Color::srgb(0.2, 0.2, 1.0),
    Color::srgb(0.2, 1.0, 0.9),
    Color::srgb(0.9, 0.17, 1.0),
];

pub const FRET_AMOUNT: usize = 25;

// Y coordinate
pub const FRET_CENTERS: [f32; FRET_AMOUNT] = [
    -14.4,
    -13.2,
    -12.0,
    -10.8,
    -9.6,
    -8.4,
    -7.2,
    -6.0,
    -4.8,
    -3.6,
    -2.4,
    -1.2,
    0.0,
    1.2,
    2.4,
    3.6,
    4.8,
    6.0,
    7.2,
    8.4,
    9.6,
    10.8,
    12.0,
    13.2,
    14.4
];

pub const CAMERA_Y_RANGE: [f32; 2] = [-3.0, 25.6];
