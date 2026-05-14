#![allow(warnings)]

use rand::random_bool;

use crate::game_loop::{AIModel, Apple, Orientation, Snake};

pub struct AIModelNaiveRandom {}

impl AIModel for AIModelNaiveRandom {
    fn decide_next_move(
        &self,
        snake: &Snake,
        apple: &Apple,
        _grid_width: u32,
        _grid_height: u32,
    ) -> Option<Orientation> {
        let dx = snake.get_head().unwrap().get_x() - apple.get_coordinates().get_x();
        let dy = snake.get_head().unwrap().get_y() - apple.get_coordinates().get_y();

        let rand_value =
            random_bool(f64::abs(dy as f64) / (f64::abs(dx as f64) + f64::abs(dy as f64)));

        if dx > 0 && dy > 0 {
            if rand_value {
                Some(Orientation::North)
            } else {
                Some(Orientation::West)
            }
        } else if dx > 0 && dy < 0 {
            if rand_value {
                Some(Orientation::South)
            } else {
                Some(Orientation::West)
            }
        } else if dx < 0 && dy > 0 {
            if rand_value {
                Some(Orientation::North)
            } else {
                Some(Orientation::East)
            }
        } else if dx < 0 && dy < 0 {
            if rand_value {
                Some(Orientation::South)
            } else {
                Some(Orientation::East)
            }
        } else {
            let rand_value = random_bool(0.5);
            if dx == 0 {
                if dy == 1 {
                    Some(Orientation::North)
                } else if dy == -1 {
                    Some(Orientation::South)
                } else if rand_value {
                    Some(Orientation::East)
                } else {
                    Some(Orientation::West)
                }
            } else if dy == 0 {
                if dx == 1 {
                    Some(Orientation::West)
                } else if dx == -1 {
                    Some(Orientation::East)
                } else if rand_value {
                    Some(Orientation::North)
                } else {
                    Some(Orientation::South)
                }
            } else {
                None
            }
        }
    }
}

impl AIModelNaiveRandom {
    pub fn new() -> Self {
        Self {}
    }
}
