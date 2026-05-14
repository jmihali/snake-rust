#[derive(Debug, Clone, Default, PartialEq, Hash, Eq)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        Self {
            x: rand::random_range(0..max_x),
            y: rand::random_range(0..max_y),
        }
    }
}

impl Coordinates {
    #[allow(unused)]
    pub fn is_contiguous_to(&self, other: &Coordinates) -> bool {
        (self.x - other.x == 0 && i32::abs(self.y - other.y) == 1)
            || (self.y - other.y == 0 && i32::abs(self.x - other.x) == 1)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_new() {
        let x = 4;
        let y = 10;
        let coordinates = Coordinates::new(x, y);

        assert!(coordinates.get_x() == x);
        assert!(coordinates.get_y() == y);
    }

    #[test]
    fn coordinates_random() {
        let max_x = 4;
        let max_y = 10;
        let coordinates = Coordinates::random(max_x, max_y);

        assert!(coordinates.get_x() <= max_x);
        assert!(coordinates.get_y() <= max_y);
    }

    #[test]
    fn contiguous_xaxis() {
        let c1 = Coordinates::new(2, 3);
        let c2 = Coordinates::new(3, 3);

        assert!(c1.is_contiguous_to(&c2));
        assert!(c2.is_contiguous_to(&c1));
    }

    #[test]
    fn contiguous_yaxis() {
        let c1 = Coordinates::new(2, 3);
        let c2 = Coordinates::new(2, 2);

        assert!(c1.is_contiguous_to(&c2));
        assert!(c2.is_contiguous_to(&c1));
    }

    #[test]
    fn not_contiguous_xaxis() {
        let c1 = Coordinates::new(2, 3);
        let c2 = Coordinates::new(4, 3);

        assert!(!c1.is_contiguous_to(&c2));
        assert!(!c1.is_contiguous_to(&c1));
    }

    #[test]
    fn not_contiguous_yaxis() {
        let c1 = Coordinates::new(2, 3);
        let c2 = Coordinates::new(2, 5);

        assert!(!c1.is_contiguous_to(&c2));
        assert!(!c1.is_contiguous_to(&c1));
    }
}
