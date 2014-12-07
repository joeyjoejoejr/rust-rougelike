extern crate tcod;
extern crate dwemthys;

use tcod::Console;
use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::WindowComponent;
use dwemthys::input::KeyCode;
use dwemthys::input::Key::SpecialKey;

fn main() {
    let mut game = Game::new();

    let mut ch = Actor::heroine(game.windows.map.get_bounds());
    let mut npcs: Vec<Box<Actor>> = vec! [
        box Actor::dog(10, 10, game.windows.map.get_bounds()),
        box Actor::cat(40, 25, game.windows.map.get_bounds()),
        box Actor::kobold(20, 20, game.windows.map.get_bounds())
    ];

    game.render(&mut npcs, &ch);

    while !(Console::window_closed() || game.exit) {
        let keypress = game.wait_for_keypress();

        match keypress.key {
            SpecialKey(KeyCode::Escape) => game.exit = true,
            _ => {}
        }

        game.update(&mut npcs, &mut ch);
        game.render(&mut npcs, &ch);
    }
}
