#[derive(Debug, PartialEq)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn rotate_left(&self) -> Vec2D {
        let x_negative = -self.x;

        Vec2D {
            x: self.y,
            y: x_negative,
        }
    }

    fn rotate_right(&self) -> Vec2D {
        let y_negative = -self.y;

        Vec2D {
            x: y_negative,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec2D { x: 1, y: 10 };
        let b = Vec2D { x: 2, y: 11 };
        assert_eq!(a.add(&b), Vec2D { x: 3, y: 21 });
    }

    #[test]
    fn rotate_left() {
        let v = Vec2D { x: 1, y: 0 };
        assert_eq!(v.rotate_left(), Vec2D { x: 0, y: -1 });
    }

    #[test]
    fn rotate_right() {
        let v = Vec2D { x: 0, y: -1 };
        assert_eq!(v.rotate_right(), Vec2D { x: 1, y: 0 });
    }
}
