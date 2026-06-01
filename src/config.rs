use std::sync::LazyLock;

use crate::element::ElementKind;

use macroquad::{color::Color, color_u8};

pub const GRID_WIDTH: u16 = 240;
pub const GRID_HEIGHT: u16 = 160;
pub const CELL_SIZE: u16 = 6;

// Simulation parameters
pub const FPS: u16 = 1;
pub const CREATURE_COUNT: u16 = 150;
pub const ELEMENT_COUNT: u16 = 80;
pub const SPEED_BASE_MULTIPLIER: f32 = 10.0;
pub const SENSE_BASE_MULTIPLIER: f32 = 50.0;
pub const SIZE_BASE_MULTIPLIER: f32 = 5.0;
pub const FUNDAMENTAL_ELEMENT: ElementKind = ElementKind::SUGAR;
pub const FUNDAMENTAL_ELEMENT_INITIAL_AMOUNT: u16 = 5;
pub const FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT: u16 = 1;
pub const ELEMENTS_MIN: u16 = 2;
pub const ELEMENTS_MAX: u16 = 10;

// Gene configuration
pub const GENE_MIN: u16 = 2;
pub const GENE_MAX: u16 = 5;
pub const ELEMENT_NEEDED_MIN: u16 = 1;
pub const ELEMENT_NEEDED_MAX: u16 = 5;
pub const ELEMENT_PRODUCED_MIN: u16 = 0;
pub const ELEMENT_PRODUCED_MAX: u16 = 5;
pub const ELEMENT_EJECTED_MIN: u16 = 0;
pub const ELEMENT_EJECTED_MAX: u16 = 4;
pub const FREQUENCY_EXPONENT_MIN: i16 = -2;
pub const FREQUENCY_EXPONENT_MAX: i16 = 2;
pub const SIZE_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SIZE_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const SPEED_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SPEED_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const SENSE_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SENSE_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const CRITICAL_PROBABILITY: f32 = 0.1;
pub const CRITICAL_THRESHOLD_MULTIPLIER: f32 = 2.0;
pub const REPRODUCE_PROBABILITY: f32 = 0.4;
pub const REPRODUCTION_THRESHOLD_MULTIPLIER: f32 = 0.5;

// Rendering
pub const BACKGROUND_COLOR: Color = color_u8!(20, 20, 20, 255);
pub const GRID_COLOR: Color = color_u8!(30, 30, 30, 255);
pub const SCREEN_WIDTH: u16 = GRID_WIDTH * CELL_SIZE;
pub const SCREEN_HEIGHT: u16 = GRID_HEIGHT * CELL_SIZE;

pub static ELEMENT_COLORS: LazyLock<[(ElementKind, Color); 10]> = LazyLock::new(|| {
    [
        (ElementKind::SUGAR, color_u8!(100, 255, 100, 255)),
        (ElementKind::SALT, color_u8!(255, 255, 100, 255)),
        (ElementKind::WATER, color_u8!(100, 100, 255, 255)),
        (ElementKind::ACID, color_u8!(255, 100, 100, 255)),
        (ElementKind::BASE, color_u8!(255, 100, 255, 255)),
        (ElementKind::OXIDE, color_u8!(100, 255, 255, 255)),
        (ElementKind::FLUORIDE, color_u8!(255, 255, 100, 255)),
        (ElementKind::CARBONATE, color_u8!(255, 100, 255, 255)),
        (ElementKind::NITRATE, color_u8!(100, 255, 255, 255)),
        (ElementKind::PHOSPHATE, color_u8!(255, 255, 100, 255)),
    ]
});

pub const VERBOSE: bool = false;

pub const THREADS_COUNT: usize = 4;