mod framebuffer;
mod grid;
mod patrones;

use framebuffer::Framebuffer;
use grid::Grid;

use patrones::{
    BEEHIVE,
    BLINKER,
    BLOCK,
    GLIDER,
    TOAD,
    EASTER_EGG,
};

use raylib::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Juego de la vida de Conway")
        .resizable()
        .build();

    window.set_target_fps(60);

    let mut grid = Grid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
    );

    let mut framebuffer = Framebuffer::new();

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.set_dead_color(Color::BLACK);
    framebuffer.set_alive_color(Color::WHITE);
    framebuffer.set_grid_visible(true);

    // Vida estacionaria
    grid.place_pattern(&BLOCK, 10, 10);
    grid.place_pattern(&BEEHIVE, 25, 10);

    // Osciladores
    grid.place_pattern(&BLINKER, 10, 30);
    grid.place_pattern(&TOAD, 25, 30);

    // Planeador
    grid.place_pattern(&GLIDER, 50, 50);

    // Easter egg
    grid.place_pattern(&EASTER_EGG, 70, 70);

    while !window.window_should_close() {
        let screen_width = window.get_screen_width().max(1);
        let screen_height = window.get_screen_height().max(1);

        let mut renderer =
            window.begin_drawing(&raylib_thread);

        framebuffer.render(
            &mut renderer,
            &grid,
            screen_width,
            screen_height,
        );
    }
}