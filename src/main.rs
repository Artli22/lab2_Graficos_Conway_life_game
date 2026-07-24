
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
    PULSAR,
    LWSS,
    EASTER_EGG,
};

use raylib::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;
const INTERVAL: f32 = 0.6;

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

    framebuffer.set_background_color(Color::new(18, 34, 60, 255));
    framebuffer.set_dead_color(Color::new(18, 34, 60, 255));
    framebuffer.set_alive_color(Color::new(220, 245, 255, 255));
    framebuffer.set_grid_color(Color::new(60, 90, 120, 255));
    framebuffer.set_grid_visible(true);

    let posiciones_block = [
    (10, 10),
    (20, 20),
    (35, 40),
    ];
    for (x, y) in posiciones_block {
    grid.place_pattern(&BLOCK, x, y);
    }

    let posiciones_beehive = [
    (10, 50),
    (50, 10),
    (50, 60),
    ];
    for (x, y) in posiciones_beehive {
        grid.place_pattern(&BEEHIVE, x, y);
    }

    let posiciones_pulsar = [
    (30, 30),
    (60, 60),
    (30, 60),
    (60, 30),
    ];
    for (x, y) in posiciones_pulsar {
        grid.place_pattern(&PULSAR, x, y);
    }

    let posiciones_lwss = [
    (10, 70),
    (40, 10),
    (70, 40),
    ];
    for (x, y) in posiciones_lwss {
        grid.place_pattern(&LWSS, x, y);
    }

    let posiciones_glider = [
    (10, 30),
    (30, 10),
    (50, 50),
    ];
    for (x, y) in posiciones_glider {
        grid.place_pattern(&GLIDER, x, y);
    }

    let posiciones_toad = [
    (25, 30),
    (45, 10),
    (65, 50),
    ];
    for (x, y) in posiciones_toad {
        grid.place_pattern(&TOAD, x, y);
    }

    let posiciones_EASTER_EGG = [
    (70, 70),
    (50, 70),
    (70, 30),
    ];

    for (x, y) in posiciones_EASTER_EGG {
        grid.place_pattern(&EASTER_EGG, x, y);
    }

    let posiciones_blinker = [
    (10, 90),
    (30, 70),
    (50, 90),
    ];
    for (x, y) in posiciones_blinker {
        grid.place_pattern(&BLINKER, x, y);
    }

    let mut elapsed_time = 0.0;
    while !window.window_should_close() {
        let frame_time = window.get_frame_time();
        elapsed_time += frame_time;

        if elapsed_time >= INTERVAL {
            grid.next_generation();
            elapsed_time = 0.0;
        }

        let screen_width = window.get_screen_width().max(1);
        let screen_height = window.get_screen_height().max(1);
        let mut renderer =
            window.begin_drawing(&raylib_thread);

        framebuffer.render( &mut renderer, &grid, screen_width, screen_height);
    }
}