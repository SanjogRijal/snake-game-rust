use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::interfaces::interface::{draw_block, draw_rectangle};
use crate::interfaces::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.80, 0.00, 0.00, 1.0];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 7,
            food_y: 5,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let (direction) = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _ => Some(self.snake.head_direction()),
        };
        if let Some(direction) = direction {
            if direction == self.snake.head_direction().opposite() {
                return;
            }
        }
        self.update_snake(direction);
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        self.snake.draw(context, g2d);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g2d);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g2d);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            g2d,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g2d);
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            g2d,
        );

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, g2d);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..(self.width - 1));
        let mut new_y = rng.gen_range(1..=(self.height - 1));

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..(self.width - 1));
            new_x = rng.gen_range(1..(self.height - 1));
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_snake_alive(&self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(direction);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_snake_alive(direction) {
            self.snake.forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 7;
        self.food_y = 5;
        self.game_over = false;
    }
}
