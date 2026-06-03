use std::sync::LazyLock;

use crate::element::ElementKind;

use macroquad::{color::Color, color_u8};

pub const GRID_WIDTH: u16 = 720;
pub const GRID_HEIGHT: u16 = 480;
pub const CELL_SIZE: u16 = 2;

// Simulation parameters
pub const FPS: u16 = 2;
pub const CREATURE_COUNT: u16 = 5000;
pub const ELEMENT_COUNT: u16 = 500;
pub const SPEED_BASE_MULTIPLIER: f32 = 10.0;
pub const SENSE_BASE_MULTIPLIER: f32 = 200.0;
pub const SIZE_BASE_MULTIPLIER: f32 = 7.0;
pub const FUNDAMENTAL_ELEMENT: ElementKind = ElementKind::SUCROSE;
pub const FUNDAMENTAL_ELEMENT_INITIAL_AMOUNT: u16 = 20;
pub const FUNDAMENTAL_ELEMENT_CONSUMPTION_AMOUNT: u16 = 1;
pub const ELEMENTS_MIN: u16 = 0;
pub const ELEMENTS_MAX: u16 = 3;

// Gene configuration
pub const GENE_MIN: u16 = 4;
pub const GENE_MAX: u16 = 20;
pub const ELEMENT_NEEDED_MIN: u16 = 1;
pub const ELEMENT_NEEDED_MAX: u16 = 5;
pub const ELEMENT_PRODUCED_MIN: u16 = 0;
pub const ELEMENT_PRODUCED_MAX: u16 = 5;
pub const ELEMENT_EJECTED_MIN: u16 = 0;
pub const ELEMENT_EJECTED_MAX: u16 = 3;
pub const FREQUENCY_EXPONENT_MIN: i16 = -2;
pub const FREQUENCY_EXPONENT_MAX: i16 = 2;
pub const SIZE_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SIZE_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const SPEED_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SPEED_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const SENSE_MULTIPLIER_EXPONENT_MIN: i16 = -1;
pub const SENSE_MULTIPLIER_EXPONENT_MAX: i16 = 1;
pub const CRITICAL_PROBABILITY: f32 = 0.3;
pub const CRITICAL_THRESHOLD_MULTIPLIER: f32 = 2.0;
pub const REPRODUCE_PROBABILITY: f32 = 0.02;
pub const REPRODUCTION_THRESHOLD_MULTIPLIER: f32 = 0.002;
pub const MUTATION_PROBABILITY: f32 = 0.01;

// Rendering
pub const BACKGROUND_COLOR: Color = color_u8!(20, 20, 20, 255);
pub const GRID_COLOR: Color = color_u8!(30, 30, 30, 255);
pub const SCREEN_WIDTH: u16 = GRID_WIDTH * CELL_SIZE;
pub const SCREEN_HEIGHT: u16 = GRID_HEIGHT * CELL_SIZE;

pub static ELEMENT_COLORS: LazyLock<[(ElementKind, Color); 10]> = LazyLock::new(|| {
    [
        (ElementKind::SUCROSE, color_u8!(255, 255, 255, 255)), //rgb(255, 255, 255)
        (ElementKind::POTASSIUM_CHROMATE, color_u8!(247, 234, 71, 255)), //rgb(247, 234, 71)
        (ElementKind::WATER, color_u8!(16, 119, 187, 255)), //rgb(16, 119, 187)
        (ElementKind::MANGANESE_DIOXIDE, color_u8!(0, 0, 0, 255)), //rgb(0, 0, 0)
        (ElementKind::LEAD_TETROXIDE, color_u8!(228, 80, 1, 255)), //rgb(228, 80, 1)
        (ElementKind::NICKEL_CHLORIDE, color_u8!(47, 152, 33, 255)), //rgb(47, 152, 33)
        (ElementKind::COPPER_FLUORIDE, color_u8!(40, 193, 197 , 255)), //rgb(40, 193, 197)
        (ElementKind::COBALT_CARBONATE, color_u8!(214, 56, 196, 255)), //rgb(214, 56, 196)
        (ElementKind::MANGANESE_NITRATE, color_u8!(243, 207, 198, 255)), //rgb(243, 207, 198)
        (ElementKind::RED_PHOSPHORUS, color_u8!(136, 8, 8, 255)), //rgb(136, 8, 8)
    ]
});

pub const VERBOSE: bool = false;

pub const THREADS_COUNT: usize = 8;
