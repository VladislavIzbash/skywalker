#![feature(assert_matches)]

use std::{env, f32::consts::{FRAC_PI_2, PI}, fs::File};

use anyhow::Context;
use level::{Cell, Level};
use pancurses::{curs_set, endwin, initscr, noecho};
use render::Renderer;

mod level;
mod render;

const MOVE_SPEED: f32 = 0.2;
const ROTATE_SPEED: f32 = 0.1;

fn main() -> anyhow::Result<()> {
    let level = env::args().nth(1).context("First arg (level file) is missing")?;
    let level = File::open(&level).context("Cannot open level file")?;
    let level = Level::load(level).context("Failed to load level file")?;

    let window = initscr();
    noecho();
    curs_set(0);

    let renderer = Renderer::new(FRAC_PI_2, f32::INFINITY);

    let mut x = 1.5;
    let mut y = 1.5;
    let mut next_x = x;
    let mut next_y = y;
    let mut angle = PI;
    let mut show_hud = true;

    renderer.render(&window, &level, x, y, angle, show_hud);
    
    loop {
        if let Some(pancurses::Input::Character(ch)) = window.getch() {
            match ch {
                'q' => angle -= ROTATE_SPEED,
                'e' => angle += ROTATE_SPEED,
                _ => {}
            };

            let forward_x = f32::sin(angle) * MOVE_SPEED;
            let forward_y = -f32::cos(angle) * MOVE_SPEED;
            let right_x = f32::cos(angle) * MOVE_SPEED;
            let right_y = f32::sin(angle) * MOVE_SPEED;

            match ch {
                'w' => {
                    next_x = x + forward_x;
                    next_y = y + forward_y;
                }
                'a' => {
                    next_x = x - right_x;
                    next_y = y - right_y;
                }
                's' => {
                    next_x = x - forward_x;
                    next_y = y - forward_y;
                }
                'd' => {
                    next_x = x + right_x;
                    next_y = y + right_y;
                }
                'r' => break,
                'h' => show_hud = !show_hud,
                _ => {}
            }

            if let Some(Cell::None) = level.cell_at(next_x, next_y) {
                x = next_x;
                y = next_y;
            }
        }
        renderer.render(&window, &level, x, y, angle, show_hud);
    }

    endwin();

    Ok(())
}
