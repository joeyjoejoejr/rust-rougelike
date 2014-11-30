extern crate tcod;
extern crate dwemthys;

use tcod::{Console, KeyCode, Special};
use dwemthys::util::{ Point, Bound };
use dwemthys::game::Game;
use dwemthys::traits::Updates;
use dwemthys::character::Character;
use dwemthys::npc::NPC;

fn update(objs: &mut Vec<Box<Updates>>, keypress: tcod::KeyState, game: Game) {
    for obj in objs.iter_mut() {
        obj.update(keypress, game);
    }
}

fn render(objs: &mut Vec<Box<Updates>>,  con: &mut Console) {
    con.clear();
    for obj in objs.iter() {
        obj.render(con);
    }
    Console::flush();
}

fn main() {
    let mut game = Game { window_bounds: Bound { min: Point { x: 0, y: 0 }, max: Point { x: 80, y: 50 } }, exit: false };
    let mut con = Console::init_root(
        (game.window_bounds.max.x + 1) as int,
        (game.window_bounds.max.y + 1) as int,
        "libtcod Rust tutorial", false
    );

    let ch = box Character::new(40,  25, '@') as Box<Updates>;
    let dog = box NPC::new(10, 10, 'd') as Box<Updates>;
    let mut objs: Vec<Box<Updates>> = vec! [ ch, dog ];

    render(&mut objs, &mut con);

    while !(Console::window_closed() || game.exit) {
        let keypress = Console::wait_for_keypress(true);

        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {
                update(&mut objs, keypress, game);
            }
        }

        render(&mut objs, &mut con);
    }
}
