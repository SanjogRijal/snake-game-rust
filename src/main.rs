extern crate piston_window;
extern crate rand;

pub mod game;
pub mod interfaces;

use game::Game;
use interfaces::interface::to_coordinates;
use piston_window::{
    clear, types::Color, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

fn main() {
    let (width, height) = (20, 30);
    const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0]; // Opaque background color.

    let mut window: PistonWindow = WindowSettings::new(
        "SNAKE GAME - Developed by Sanjog",
        [to_coordinates(width), to_coordinates(height)],
    )
    .exit_on_esc(true)
    .build()
    .expect("Failed to create Piston window");

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("Key pressed: {:?}", key); // Debug key input.
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, g2d, _| {
            clear(BACK_COLOR, g2d);
            game.draw(&context, g2d);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
