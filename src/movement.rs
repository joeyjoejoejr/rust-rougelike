extern crate tcod;
use std;
use std::rand::Rng;
use util::{Bound, Point, XPointRelation, YPointRelation, PointRelation };
use util::Contains::{DoesContain, DoesNotContain};
use game::{ Game, Windows };
use input::{ KeyCode };
use input::Key::SpecialKey;

pub trait MovementComponent {
    fn new(Bound) -> Self;
    fn update(&self, Point, &mut Windows) -> Point;
}

pub struct RandomMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for RandomMovementComponent {
    fn new(window_bounds: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: window_bounds }
    }

    fn update(&self, point: Point, _: &mut Windows) -> Point {
        let mut offset = Point { x: point.x, y: point.y };

        let offset_x: i32 = std::rand::task_rng().gen_range(0, 3) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => {}
        }

        let offset_y: i32 = std::rand::task_rng().gen_range(0, 3) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => {}
        }

        offset
    }
}

pub struct UserMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for UserMovementComponent {
    fn new(window_bounds: Bound) -> UserMovementComponent {
        UserMovementComponent { window_bounds: window_bounds }
    }

    fn update(&self, point: Point, windows: &mut Windows) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match keypress.key {
                    SpecialKey(KeyCode::Up) => {
                        offset.offset_y(-1)
                    },
                    SpecialKey(KeyCode::Down) => {
                        offset.offset_y(1)
                    },
                    SpecialKey(KeyCode::Left) => {
                        offset.offset_x(-1)
                    },
                    SpecialKey(KeyCode::Right) => {
                        offset.offset_x(1)
                    },
                    _ => { offset }
                }
            },
            None => { offset }
        };

        match self.window_bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => {
                windows.messages.buffer_message("You can't move that way!");
                point
            }
        }
    }
}

pub struct AgroMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for AgroMovementComponent {
    fn new(window_bounds: Bound) -> AgroMovementComponent {
        AgroMovementComponent { window_bounds: window_bounds }
    }

    fn update(&self, point: Point, _: &mut Windows) -> Point {
        let char_point = Game::get_character_location();
        let mut offset = Point { x: 0, y: 0 };

        match point.compare_x(char_point) {
            XPointRelation::RightOfPoint => offset = offset.offset_x(-1),
            XPointRelation::LeftOfPoint => offset = offset.offset_x(1),
            XPointRelation::OnPointX => {}
        }

        match point.compare_y(char_point) {
            YPointRelation::BelowPoint => offset = offset.offset_y(-1),
            YPointRelation::AbovePoint => offset = offset.offset_y(1),
            YPointRelation::OnPointY => {}
        }

        match point.offset(offset).compare(char_point) {
            PointRelation::PointsEqual => { point },
            PointRelation::PointsNotEqual => {
                match self.window_bounds.contains(point.offset(offset)) {
                    DoesContain => { point.offset(offset) },
                    DoesNotContain => { point }
                }
            }
        }
    }
}
