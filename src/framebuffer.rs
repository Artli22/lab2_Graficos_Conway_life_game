use raylib::prelude::*;

use crate::grid::Grid;

pub struct Framebuffer {
    background_color: Color,
    alive_color: Color,
    dead_color: Color,
    grid_color: Color,
    show_grid: bool,
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            background_color: Color::BLACK,
            alive_color: Color::WHITE,
            dead_color: Color::BLACK,
            grid_color: Color::new(40, 40, 40, 255),
            show_grid: true,
        }
    }

    pub fn set_alive_color(&mut self, color: Color) {
        self.alive_color = color;
    }

    pub fn set_dead_color(&mut self, color: Color) {
        self.dead_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_grid_visible(&mut self, visible: bool) {
        self.show_grid = visible;
    }

    pub fn render(
        &self,
        renderer: &mut RaylibDrawHandle,
        grid: &Grid,
        screen_width: i32,
        screen_height: i32,
    ) {
        renderer.clear_background(self.background_color);

        let cell_width =
            screen_width as f32 / grid.width() as f32;

        let cell_height =
            screen_height as f32 / grid.height() as f32;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let color = if grid.is_alive(x, y) {
                    self.alive_color
                } else {
                    self.dead_color
                };

                let left = (x as f32 * cell_width).floor() as i32;
                let top = (y as f32 * cell_height).floor() as i32;

                let right =
                    ((x + 1) as f32 * cell_width).ceil() as i32;

                let bottom =
                    ((y + 1) as f32 * cell_height).ceil() as i32;

                let width = right - left;
                let height = bottom - top;

                renderer.draw_rectangle(
                    left,
                    top,
                    width,
                    height,
                    color,
                );
            }
        }

        if self.show_grid {
            self.draw_grid(
                renderer,
                grid,
                screen_width,
                screen_height,
            );
        }
    }

    fn draw_grid(
        &self,
        renderer: &mut RaylibDrawHandle,
        grid: &Grid,
        screen_width: i32,
        screen_height: i32,
    ) {
        let cell_width =
            screen_width as f32 / grid.width() as f32;

        let cell_height =
            screen_height as f32 / grid.height() as f32;

        for x in 0..=grid.width() {
            let screen_x =
                (x as f32 * cell_width).round() as i32;

            renderer.draw_line(
                screen_x,
                0,
                screen_x,
                screen_height,
                self.grid_color,
            );
        }

        for y in 0..=grid.height() {
            let screen_y =
                (y as f32 * cell_height).round() as i32;

            renderer.draw_line(
                0,
                screen_y,
                screen_width,
                screen_y,
                self.grid_color,
            );
        }
    }
}