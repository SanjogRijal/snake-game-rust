use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

//pub keyword makes the function public to whole application
pub fn to_coordinates(game_coordinate: i32) -> f64 {
    (game_coordinate as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, g2d: &mut G2d) {
    let gui_x = to_coordinates(x);
    let gui_y: f64 = to_coordinates(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        g2d,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    context: &Context,
    g2d: &mut G2d,
) {
    let gui_x = to_coordinates(x);
    let gui_y: f64 = to_coordinates(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        context.transform,
        g2d,
    );
}
