use crate::game::coordinates::Coordinates;
use crate::game::orientation::Orientation;
use crate::game::snake::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Snake {
    body: Vec<Coordinates>, // body[0] = head
    head_orientation: Orientation,
}

// constructors
impl Snake {
    pub fn new(coordinates: Coordinates, orientation: Orientation) -> Self {
        Self {
            body: vec![coordinates],
            head_orientation: orientation,
        }
    }

    pub fn from_body(body: Vec<Coordinates>, head_orientation: Orientation) -> Result<Self> {
        // Use .iter() and .zip() to simulate windows without cloning or make_contiguous
        let is_contiguous = body
            .iter()
            .zip(body.iter().skip(1))
            .all(|(current, next)| current.is_contiguous_to(next));

        if !is_contiguous {
            return Err(Error::SnakeBodyNotContiguous);
        }

        // todo: handle case when snake has collided with itself
        // todo: handle case when head orientation is opposite to the body
        Ok(Self {
            body,
            head_orientation,
        })
    }
}

// helper methods
impl Snake {
    fn shorten_tail(&mut self) -> Result<()> {
        // .map(|_| ()) converts the Option<T> (the popped element) into an Option<()>
        self.body.pop().map(|_| ()).ok_or(Error::SnakeBodyEmpty)
    }

    fn extend_head(&mut self) -> Result<()> {
        if let Some(current_head_coordinates) = self.body.get(0) {
            let new_head_coordinates = match self.head_orientation {
                Orientation::North => Coordinates::new(
                    current_head_coordinates.get_x(),
                    current_head_coordinates.get_y() - 1,
                ),
                Orientation::South => Coordinates::new(
                    current_head_coordinates.get_x(),
                    current_head_coordinates.get_y() + 1,
                ),
                Orientation::East => Coordinates::new(
                    current_head_coordinates.get_x() + 1,
                    current_head_coordinates.get_y(),
                ),
                Orientation::West => Coordinates::new(
                    current_head_coordinates.get_x() - 1,
                    current_head_coordinates.get_y(),
                ),
            };

            self.body.insert(0, new_head_coordinates);
            Ok(())
        } else {
            return Err(Error::SnakeBodyEmpty);
        }
    }

    pub fn get_head(&self) -> Result<&Coordinates> {
        if let Some(head) = self.body.get(0) {
            return Ok(head);
        }
        Err(Error::SnakeBodyEmpty)
    }
}

// public methods
impl Snake {
    pub fn set_head_orientation(&mut self, orientation: Orientation) -> Result<()> {
        // Prevent 180-degree turns (opposite directions)
        let is_opposite = matches!(
            (self.head_orientation, orientation),
            (Orientation::North, Orientation::South)
                | (Orientation::South, Orientation::North)
                | (Orientation::East, Orientation::West)
                | (Orientation::West, Orientation::East)
        );

        if !is_opposite {
            self.head_orientation = orientation;
        }

        Ok(())
    }

    pub fn advance(&mut self, grow: bool) -> Result<()> {
        self.extend_head()?;
        if !grow {
            self.shorten_tail()?;
        }
        Ok(())
    }

    pub fn get_body(&self) -> &Vec<Coordinates> {
        &self.body
    }

    pub fn has_collided(&self, grid_width: u32, grid_height: u32) -> Result<bool> {
        let head = self.get_head()?;

        Ok((head.get_x() < 0)
            || (head.get_y() < 0)
            || (head.get_x() >= grid_width as i32)
            || (head.get_y() >= grid_height as i32))
    }

    pub fn has_collided_with_itself(&self) -> Result<bool> {
        let head = self.get_head()?;
        Ok(self.body.iter().skip(1).any(|x| x == head))
    }
}
