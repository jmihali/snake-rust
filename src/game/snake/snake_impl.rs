use crate::game::Apple;
use crate::game::Coordinates;
use crate::game::Orientation;
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
        let head = self.body.get(0).ok_or(Error::SnakeBodyEmpty)?;
        let (dx, dy) = self.head_orientation.get_delta();
        let new_head = Coordinates::new(head.get_x() + dx, head.get_y() + dy);
        self.body.insert(0, new_head);
        Ok(())
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
    pub fn set_head_orientation(&mut self, orientation: Orientation) {
        if !self.head_orientation.is_opposite(orientation) {
            self.head_orientation = orientation;
        }
    }

    pub fn advance(&mut self, grow: bool) -> Result<()> {
        self.extend_head()?;
        if !grow {
            self.shorten_tail()?;
        }
        Ok(())
    }

    pub fn get_body(&self) -> &[Coordinates] {
        &self.body
    }

    pub fn has_collided_with_grid(&self, grid_width: u32, grid_height: u32) -> Result<bool> {
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

    pub fn has_reached_apple(&self, apple: &Apple) -> Result<bool> {
        Ok(self.get_head()? == apple.get_coordinates())
    }
}
