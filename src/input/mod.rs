extern crate tcod;
use self::tcod::{ KeyState };
use self::Key::{ Printable, SpecialKey };

pub enum Key {
    Printable(char),
    SpecialKey(KeyCode)
}

pub struct KeyboardInput {
    pub key: Key
}

pub enum KeyCode {
    // Arrow Keys
    Up,
    Down,
    Left,
    Right,

    // Special
    Shift,
    Escape,

    // Default
    None
}

pub trait InputComponent<T> {
    fn new() -> Self;
    fn translate_input(&self, T) -> KeyboardInput;
}

pub struct TcodInputComponent;

impl InputComponent<KeyState> for TcodInputComponent {
    fn new() -> TcodInputComponent { TcodInputComponent }
     fn translate_input(&self, key_state: KeyState) -> KeyboardInput {
         let key: Key = if key_state.shift {
            match key_state.key {
                self::tcod::Key::Special(tcod::KeyCode::Number5) => Printable('%'),
                self::tcod::Key::Special(tcod::KeyCode::Number6) => Printable('^'),
                self::tcod::Key::Special(tcod::KeyCode::Number8) => Printable('*'),
                _ => SpecialKey(KeyCode::None)
            }
         } else {
             match key_state.key {
                self::tcod::Key::Printable('/') => Printable('/'),
                self::tcod::Key::Special(tcod::KeyCode::Up) => SpecialKey(KeyCode::Up),
                self::tcod::Key::Special(tcod::KeyCode::Down) => SpecialKey(KeyCode::Down),
                self::tcod::Key::Special(tcod::KeyCode::Left) => SpecialKey(KeyCode::Left),
                self::tcod::Key::Special(tcod::KeyCode::Right) => SpecialKey(KeyCode::Right),
                self::tcod::Key::Special(tcod::KeyCode::Shift) => SpecialKey(KeyCode::Shift),
                self::tcod::Key::Special(tcod::KeyCode::Escape) => SpecialKey(KeyCode::Escape),
                _ => SpecialKey(KeyCode::None)
             }

         };

         KeyboardInput { key: key }
     }
}
