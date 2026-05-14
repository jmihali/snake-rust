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
            return None;
        }

        // 1. Try to find path to apple
        if let Some(orientation) =
            self.find_path_to_target(snake, apple_pos, grid_width, grid_height)
        {
            // Check if move is safe (only check tail reachability if length > 1)
            if snake.get_body().len() < 2
                || self.is_move_safe(snake, orientation, apple_pos, grid_width, grid_height)
            {
                return Some(orientation);
            }
        }

        // 2. Fallback: Follow tail if we have one
        if snake.get_body().len() > 1 {
            if let Some(tail_move) = self.find_path_to_tail(snake, grid_width, grid_height) {
                return Some(tail_move);
            }
        }

        // 3. Final Fallback: Just don't hit a wall or yourself
        self.find_any_valid_move(snake, grid_width, grid_height)
    }
}

impl AIModelBFS {
    fn find_path_to_target(
        &self,
        snake: &Snake,
        target: &Coordinates,
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation> {
        let head = snake.get_head().ok()?;
        self.bfs(
            (head.get_x(), head.get_y()),
            (target.get_x(), target.get_y()),
            snake.get_body(),
            grid_width,
            grid_height,
        )
    }

    fn is_move_safe(
        &self,
        snake: &Snake,
        orientation: Orientation,
        apple_pos: &Coordinates,
        grid_width: u32,
        grid_height: u32,
    ) -> bool {
        let head = snake.get_head().ok().unwrap();
        let (dx, dy) = orientation.get_delta();
        let new_head = Coordinates::new(head.get_x() + dx, head.get_y() + dy);

        let mut simulated_body: Vec<Coordinates> = snake.get_body().to_vec();
        simulated_body.insert(0, new_head.clone());

        if &new_head != apple_pos {
            simulated_body.pop();
        }

        if simulated_body.len() < 2 {
            return true;
        }

        let tail = simulated_body.last().unwrap();

        // Use logic that handles length 2 (where obstacles set becomes empty)
        let obstacles: HashSet<Coordinates> = if simulated_body.len() > 2 {
            simulated_body[1..simulated_body.len() - 1]
                .iter()
                .cloned()
                .collect()
        } else {
            HashSet::new()
        };

        self.can_reach(
            (new_head.get_x(), new_head.get_y()),
            (tail.get_x(), tail.get_y()),
            &obstacles,
            grid_width,
            grid_height,
        )
    }

    fn find_path_to_tail(
        &self,
        snake: &Snake,
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation> {
        let body = snake.get_body();
        if body.len() < 2 {
            return None;
        }

        let head = snake.get_head().ok()?;
        let tail = body.last().unwrap();

        // The tail segment will move, so we treat it as an empty space
        let body_without_tail: HashSet<Coordinates> =
            body[..body.len() - 1].iter().cloned().collect();

        self.bfs_with_obstacles(
            (head.get_x(), head.get_y()),
            (tail.get_x(), tail.get_y()),
            &body_without_tail,
            grid_width,
            grid_height,
        )
    }

    fn find_any_valid_move(&self, snake: &Snake, width: u32, height: u32) -> Option<Orientation> {
        let head = snake.get_head().ok()?;
        let body_set: HashSet<Coordinates> = snake.get_body().iter().cloned().collect();

        for orient in [
            Orientation::North,
            Orientation::South,
            Orientation::East,
            Orientation::West,
        ] {
            let (dx, dy) = orient.get_delta();
            let nx = head.get_x() + dx;
            let ny = head.get_y() + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let pos = Coordinates::new(nx, ny);
                if !body_set.contains(&pos) {
                    return Some(orient);
                }
            }
        }
        None
    }

    fn bfs(
        &self,
        start: (i32, i32),
        goal: (i32, i32),
        body: &[Coordinates],
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation> {
        let obstacles: HashSet<Coordinates> = body.iter().cloned().collect();
        self.bfs_with_obstacles(start, goal, &obstacles, grid_width, grid_height)
    }

    fn bfs_with_obstacles(
        &self,
        start: (i32, i32),
        goal: (i32, i32),
        obstacles: &HashSet<Coordinates>,
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == goal {
                return self.backtrack_first_move(start, goal, &parent);
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
                    if (!obstacles.contains(&pos) || (nx, ny) == goal)
                        && !visited.contains(&(nx, ny))
                    {
                        visited.insert((nx, ny));
                        queue.push_back((nx, ny));
                        parent.insert((nx, ny), (x, y));
                    }
                }
            }
        }
        None
    }

    fn can_reach(
        &self,
        start: (i32, i32),
        goal: (i32, i32),
        obstacles: &HashSet<Coordinates>,
        grid_width: u32,
        grid_height: u32,
    ) -> bool {
        if start == goal {
            return true;
        }

        self.bfs_with_obstacles(start, goal, obstacles, grid_width, grid_height)
            .is_some()
    }

    fn backtrack_first_move(
        &self,
        start: (i32, i32),
        goal: (i32, i32),
        parent: &HashMap<(i32, i32), (i32, i32)>,
    ) -> Option<Orientation> {
        let mut current = goal;
        while let Some(&prev) = parent.get(&current) {
            if prev == start {
                let dx = current.0 - start.0;
                let dy = current.1 - start.1;
                return match (dx, dy) {
                    (0, -1) => Some(Orientation::North),
                    (0, 1) => Some(Orientation::South),
                    (1, 0) => Some(Orientation::East),
                    (-1, 0) => Some(Orientation::West),
                    _ => None,
                };
            }
            current = prev;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn can_reach_when_start_and_goal_are_same() {
        let ai = AIModelBFS::new();
        assert!(ai.can_reach((2, 2), (2, 2), &HashSet::new(), 5, 5));
    }

    #[test]
    fn moving_into_tail_is_considered_safe() {
        let snake = Snake::from_body(
            vec![
                Coordinates::new(1, 1),
                Coordinates::new(1, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
            ],
            Orientation::North,
        )
        .unwrap();

        let ai = AIModelBFS::new();
        let apple = Apple::new(0, 0);

        assert!(ai.is_move_safe(&snake, Orientation::East, apple.get_coordinates(), 5, 5));
    }
}

impl AIModelBFS {
    pub fn new() -> Self {
        Self {}
    }
}
