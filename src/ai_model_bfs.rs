use crate::game_loop::{AIModel, Apple, Coordinates, Orientation, Snake};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct AIModelBFS {}

impl AIModel for AIModelBFS {
    fn decide_next_move(
        &self,
        snake: &Snake,
        apple: &Apple,
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation> {
        let head = snake.get_head().ok()?;
        let apple_pos = apple.get_coordinates();

        if head == apple_pos {
            return None; // Already at apple, no move needed
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back((head.get_x(), head.get_y()));
        visited.insert((head.get_x(), head.get_y()));

        let mut found = false;
        let mut goal = None;

        while let Some((x, y)) = queue.pop_front() {
            if x == apple_pos.get_x() && y == apple_pos.get_y() {
                found = true;
                goal = Some((x, y));
                break;
            }

            for orient in [
                Orientation::North,
                Orientation::South,
                Orientation::East,
                Orientation::West,
            ] {
                let (dx, dy) = orient.get_delta();
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < grid_width as i32 && ny >= 0 && ny < grid_height as i32 {
                    let pos = Coordinates::new(nx, ny);
                    if !snake.get_body().contains(&pos) && !visited.contains(&(nx, ny)) {
                        visited.insert((nx, ny));
                        queue.push_back((nx, ny));
                        parent.insert((nx, ny), (x, y));
                    }
                }
            }
        }

        if !found {
            return None;
        }

        // Backtrack to find the path
        let mut path = vec![];
        let mut current = goal.unwrap();
        while current != (head.get_x(), head.get_y()) {
            path.push(current);
            current = *parent.get(&current).unwrap();
        }
        path.reverse();

        // The first step in the path
        let next = path[0];
        let dx = next.0 - head.get_x();
        let dy = next.1 - head.get_y();

        let orientation = match (dx, dy) {
            (0, -1) => Orientation::North,
            (0, 1) => Orientation::South,
            (1, 0) => Orientation::East,
            (-1, 0) => Orientation::West,
            _ => unreachable!(),
        };

        Some(orientation)
    }
}

impl AIModelBFS {
    pub fn new() -> Self {
        Self {}
    }
}
