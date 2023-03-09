#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn new(x: i32, y: i32) -> Vec2D {
        Vec2D { x, y }
    }

    pub fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
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
}
