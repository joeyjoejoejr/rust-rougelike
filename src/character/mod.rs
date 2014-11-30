extern crate tcod;

use self::tcod::{BackgroundFlag, Console, KeyCode, Special};
use util::Point;
use util::Contains::{ DoesNotContain, DoesContain };
use game::Game;
use traits::Updates;

pub struct Character {
    position: Point,
    display_char: char
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Character {
        Character { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for Character {
    fn update(&mut self, keypress: tcod::KeyState, game: Game) {
        let mut offset = Point { x: 0, y: 0 };
        match keypress.key {
            Special(KeyCode::Up) => {
                offset.y -= 1;
            },
            Special(KeyCode::Down) => {
                offset.y += 1;
            },
            Special(KeyCode::Left) => {
                offset.x -= 1;
            },
            Special(KeyCode::Right) => {
                offset.x += 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.offset(offset)) {
            DoesContain => self.position = self.position.offset(offset),
            DoesNotContain => {}
        }
    }

    fn render(&self, con: &mut Console) {
        con.put_char(self.position.x as int, self.position.y as int, self.display_char, BackgroundFlag::Set);
    }
}
