use std::collections::VecDeque;

const BOARD_WIDTH: i32 = 300;
const BOARD_HEIGHT: i32 = 300;

pub struct Snake {
    start: Point2D,
    direction: Direction,
    segments: VecDeque<SnakeSegment>,
}

struct SnakeSegment {
    direction: Direction,
    length: i32,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}
impl Point2D {
    fn add_direction(&self, direction: &Direction, distance: i32) -> (Point2D, bool) {
        let new_x = self.x + direction.x_offset() * distance;
        let new_y = self.y + direction.y_offset() * distance;

        (Point2D {
            x: (new_x + BOARD_WIDTH) % BOARD_WIDTH,
            y: (new_y + BOARD_HEIGHT) % BOARD_HEIGHT,
        }, new_x < BOARD_WIDTH && new_x >= 0 && new_y < BOARD_HEIGHT && new_y <= 0)
    }
}

pub enum TurnDirection {
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
            start: Point2D{ x: 5, y: 5},
            direction: Direction::XNegative,
            segments: VecDeque::from([SnakeSegment {
                direction: Direction::XPositive,
                length: 20,
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
            let (new_start, crossed_board) = self.start.add_direction(&self.direction, 1);
            self.start = new_start;
            if self.direction.opposite() == first.direction && !crossed_board {
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
            current = current.add_direction(&segment.direction, segment.length).0;
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

        assert_eq!(snake.segments(), vec![Point2D{x:5, y:5}, Point2D{ x:7, y:5}]);
    }

    #[test]
    fn snake_move_() {
        let mut snake = Snake::new();
        snake.move_forward();

        assert_eq!(snake.segments(), vec![Point2D{x: 4, y:5}, Point2D{ x:6,y: 5}]);
    }

    #[test]
    fn snake_move_move() {
        let mut snake = Snake::new();
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.segments(), vec![Point2D{x:3, y:5}, Point2D{x:5, y:5}]);
    }

    #[test]
    fn snake_turn_left_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Left);
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YPositive);
        assert_eq!(
            snake.segments(),
            vec![Point2D{x:5, y:6}, Point2D{x:5, y:5}, Point2D{x:6, y:5}]
        );
    }

    #[test]
    fn snake_turn_left_move_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Left);
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YPositive);
        assert_eq!(snake.segments(), vec![Point2D{x:5, y:7}, Point2D{x:5, y:5}]);
    }

    #[test]
    fn snake_turn_right_move_move() {
        let mut snake = Snake::new();
        snake.turn(TurnDirection::Right);
        snake.move_forward();
        snake.move_forward();

        assert_eq!(snake.direction, Direction::YNegative);
        assert_eq!(snake.segments(), vec![Point2D{x:5, y:3}, Point2D{x:5, y:5}]);
    }
}
