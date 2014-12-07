extern crate tcod;
extern crate dwemthys;

use tcod::{Console, KeyCode, Special};
use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::WindowComponent;


fn main() {
    let mut game = Game::new();

    let mut ch = Actor::heroine(game.map_window.get_bounds());
    let mut npcs: Vec<Box<Actor>> = vec! [
        box Actor::dog(10, 10, game.map_window.get_bounds()),
        box Actor::cat(40, 25, game.map_window.get_bounds()),
        box Actor::kobold(20, 20, game.map_window.get_bounds())
    ];

    game.render(&mut npcs, &ch);

    while !(Console::window_closed() || game.exit) {
        let keypress = game.wait_for_keypress();

        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }

        game.update(&mut npcs, &mut ch);
        game.render(&mut npcs, &ch);
    }
}
