use macroquad::prelude::*;

use crate::config::{BACKGROUND_COLOR, CELL_SIZE, ELEMENT_COLORS, GRID_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::world::World;

mod config;
mod element;
mod gene;
mod creature;
mod world;
mod utils;

fn draw_grid() {
    for x in (0..SCREEN_WIDTH).step_by(CELL_SIZE as usize) {
        draw_line(x as f32, 0.0, x as f32, SCREEN_HEIGHT as f32, 1.0, GRID_COLOR);
    }
    for y in (0..SCREEN_HEIGHT).step_by(CELL_SIZE as usize) {
        draw_line(0.0, y as f32, SCREEN_WIDTH as f32, y as f32, 1.0, GRID_COLOR);
    }
}

fn draw_elements(world: &World) {
    for element in world.get_elements() {
        let color = ELEMENT_COLORS.iter().find(|(kind, _)| *kind == element.get_kind()).expect(&format!("ELEMENT COLOR NOT PRESENT FOR ELEMENT KIND {}", element.get_kind())).1;
        draw_triangle(
            Vec2::new(*element.get_x() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32 / 2.0, *element.get_y() as f32 * CELL_SIZE as f32),
            Vec2::new(*element.get_x() as f32 * CELL_SIZE as f32, *element.get_y() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32),
            Vec2::new(*element.get_x() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32, *element.get_y() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32),
            color
        );
    }
}

fn draw_creatures(world: &World) {
    for creature in world.get_creatures() {
        draw_circle(
            creature.get_x() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32 / 2.0,
            creature.get_y() as f32 * CELL_SIZE as f32 + CELL_SIZE as f32 / 2.0,
            CELL_SIZE as f32 / 2.0,
            creature.get_color()
        );
    }
}

fn draw_overlay(world: &World) {
    let text = format!("Creatures: {}  Elements: {}  (R to reset)", world.get_creatures().len(), world.get_elements().len());
    draw_text(&text, 10.0, 20.0, 20.0, WHITE);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: i32::from(SCREEN_WIDTH),
        window_height: i32::from(SCREEN_HEIGHT),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut delta = 0.0;
    let mut world = World::new();
    world.reset();
    loop {
        clear_background(BACKGROUND_COLOR);
        
        draw_grid();
        draw_elements(&world);
        draw_creatures(&world);
        draw_overlay(&world);

        if delta == 0.0 {
            world.update();
        }
        delta += get_frame_time();
        if delta >= 1.0 / (crate::config::FPS as f32) {
            delta = 0.0;
        }

        if is_key_pressed(KeyCode::R) {
            world.reset();
        }

        next_frame().await
    }
}