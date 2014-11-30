extern crate tcod;

use std;
use std::rand::Rng;

use self::tcod::{BackgroundFlag, Console};

use util::Point;
use util::Contains::{DoesContain, DoesNotContain};
use traits::Updates;
use game::Game;

pub struct NPC {
    position: Point,
    display_char: char
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for NPC {
    fn update(&mut self, _keypress: tcod::KeyState, game: Game) {
        let offset_x: i32 = std::rand::task_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain => self.position = self.position.offset_x(offset_x),
            DoesNotContain => {}
        }

        let offset_y: i32 = std::rand::task_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain => self.position = self.position.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, con: &mut Console) {
        con.put_char(self.position.x as int, self.position.y as int, self.display_char, BackgroundFlag::Set);
    }
}
