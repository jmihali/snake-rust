#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    pub fn is_opposite(&self, other: Orientation) -> bool {
        matches!(
            (*self, other),
            (Orientation::North, Orientation::South)
                | (Orientation::South, Orientation::North)
                | (Orientation::East, Orientation::West)
                | (Orientation::West, Orientation::East)
        )
    }
}
