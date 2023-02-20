use std::collections::VecDeque;

struct Snake {
    start: Point2D,
    direction: Direction,
    segments: VecDeque<SnakeSegment>,
}

struct SnakeSegment {
    direction: Direction,
    length: i32,
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct Point2D(i32, i32);

enum TurnDirection {
    Left,
    Right,
}
impl TurnDirection {
    fn turn(&self, current_direction: &Direction) -> Direction {
        match self {
            TurnDirection::Left => match current_direction {
                Direction::XNegative => Direction::YPositive,
                Direction::XPositive => Direction::YNegative,
                Direction::YNegative => Direction::XNegative,
                Direction::YPositive => Direction::XPositive,
            },
            TurnDirection::Right => match current_direction {
                Direction::XNegative => Direction::YNegative,
                Direction::XPositive => Direction::YPositive,
                Direction::YNegative => Direction::XPositive,
                Direction::YPositive => Direction::XNegative,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
}
impl Direction {
    fn x_offset(&self) -> i32 {
        match self {
            Direction::XNegative => -1,
            Direction::XPositive => 1,
            Direction::YNegative => 0,
            Direction::YPositive => 0,
        }
    }

    fn y_offset(&self) -> i32 {
        match self {
            Direction::YNegative => -1,
            Direction::YPositive => 1,
            Direction::XNegative => 0,
            Direction::XPositive => 0,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::YNegative => Self::YPositive,
            Direction::YPositive => Self::YNegative,
            Direction::XNegative => Self::XPositive,
            Direction::XPositive => Self::XNegative,
        }
    }
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            start: Point2D(5, 5),
            direction: Direction::XNegative,
            segments: VecDeque::from([SnakeSegment {
                direction: Direction::XPositive,
                length: 2,
            }]),
        }
    }

    pub fn move_forward(&mut self) {
        if let Some(mut last) = self.segments.back_mut() {
            last.length = last.length - 1;
            if last.length == 0 {
                self.segments.pop_back();
            }
        }
        if let Some(mut first) = self.segments.front_mut() {
            self.start = Point2D(
                self.start.0 + self.direction.x_offset(),
                self.start.1 + self.direction.y_offset(),
            );
            if self.direction.opposite() == first.direction {
                first.length = first.length + 1;
            } else {
                self.segments.push_front(SnakeSegment {
                    direction: self.direction.opposite(),
                    length: 1,
                });
            }
        }
    }

    pub fn turn(&mut self, turn_direction: TurnDirection) {
        self.direction = turn_direction.turn(&self.direction);
    }

    pub fn segments(&self) -> Vec<Point2D> {
        let mut points = Vec::new();
        let mut current = self.start.clone();

        points.push(current);
        for segment in &self.segments {
            current = Point2D(
                current.0 + segment.direction.x_offset() * segment.length,
                current.1 + segment.direction.y_offset() * segment.length,
            );
            points.push(current);
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_initial() {
        let snake = Snake::new();

        assert_eq!(snake.segments(), vec![Point2D(5, 5), Point2D(7, 5)]);
    }

    #[test]
    fn snake_move_() {
        let mut snake = Snake::new();
        snake.move_forward();

        assert_eq!(snake.segments(), vec![Point2D(4, 5), Point2D(6, 5)]);
    }

    #[test]
    fn snake_move_move() {
        let mut snake = Snake::new();
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.segments(), vec![Point2D(3, 5), Point2D(5, 5)]);
    }

    #[test]
    fn snake_turn_left_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Left);
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YPositive);
        assert_eq!(
            snake.segments(),
            vec![Point2D(5, 6), Point2D(5, 5), Point2D(6, 5)]
        );
    }

    #[test]
    fn snake_turn_left_move_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Left);
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YPositive);
        assert_eq!(snake.segments(), vec![Point2D(5, 7), Point2D(5, 5)]);
    }

    #[test]
    fn snake_turn_right_move_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Right);
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YNegative);
        assert_eq!(snake.segments(), vec![Point2D(5, 3), Point2D(5, 5)]);
    }
}
