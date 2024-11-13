mod helpers;

use macroquad::prelude::*;
use eqrs::{calculate::calc, eval, post_process::post_process, tokenize::tokenize, variable::VarTable};
use helpers::*;

const BACKSPACE_INTERVAL: f32 = 0.1;

static mut X_SCL: f32 = 20.0;
static mut Y_SCL: f32 = 20.0;

#[macroquad::main("Graphing Calculator")]
async fn main() {

    let mut eq = "y = ".to_string();
    let mut backspace_timer = 0.0;
    let mut dt;

    loop {
        clear_background(WHITE);

        dt = 1.0 / get_fps() as f32;
        backspace_timer = min(backspace_timer - dt, 0.0);

        for key in get_keys_pressed() {
            let char_to_add = match key_code_to_char(key, is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) {
                Some(c) => String::from(c),
                None => String::new(),
            };
            eq += char_to_add.as_str();
        }

        if is_key_down(KeyCode::Backspace) && eq.len() > 4 && backspace_timer <= 0.0 {
            eq = String::from(&eq[0..eq.len() - 1]);
            backspace_timer = BACKSPACE_INTERVAL;
        }

        if is_key_down(KeyCode::Down) {
            unsafe {
                X_SCL *= 1.01;
            }
        }
        if is_key_down(KeyCode::Up) {
            unsafe {
                X_SCL *= 0.99;
            }
        }

        if eq.len() > 4 {
            graph(&eq[4..]);
        }

        draw_text(eq.as_str(), 20.0, 30.0, 30.0, BLACK);

        draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 2.0, BLACK);
        draw_line(0.0, screen_height() / 2.0, screen_width(), screen_height() / 2.0, 2.0, BLACK);

        next_frame().await
    }
}

fn graph(eq: &str) {
    let expr;
    match tokenize(eq) {
        Ok(tokens) => expr = post_process(&tokens),
        Err(_) => return,
    }

    let mut vt = VarTable::new();
    let mut vec = Vec::with_capacity(screen_width() as usize);
    for x in 0..screen_width() as u32 {
        unsafe {
            vt.set('x', (x as f64  - screen_width() as f64 / 2.0) * (X_SCL / screen_width() / 2.0) as f64);
        }
        match calc(&expr, Some(&vt)) {
            Ok(y) => vec.push(y),
            Err(_) => return,
        }
        
        if x != 0 {
            let x1 = x as f32;
            let x2 = x as f32 - 1.0;
            let mut y1 = vec[x as usize] as f32;
            let mut y2 = vec[x as usize - 1] as f32;

            unsafe {
                y1 = y1 / Y_SCL * screen_height();
                y1 = screen_height() / 2.0 - y1;
                y2 = y2 / Y_SCL * screen_height();
                y2 = screen_height() / 2.0 - y2;
            }

            draw_line(x1, y1, x2, y2, 3.0, RED);
        }
    }
}
