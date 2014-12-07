#![feature(macro_rules)]

macro_rules! window_component_getters(
    () => {
        fn get_console(&mut self) -> &mut Console { &mut self.console }
        fn get_bounds(&self) -> Bound { self.bounds }
        fn get_bg_color(&self) -> Color { self.background_color }
        fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> { &mut self.messages }
        fn get_messages(&self) -> Vec<Box<String>> { self.messages.clone() }
        fn get_max_messages(&self) -> uint { self.max_messages }
    }
)

macro_rules! window_component_def(
    ($name:ident) => {
        pub struct $name {
            pub console: Console,
            pub background_color: Color,
            bounds: Bound,
            messages: Vec<Box<String>>,
            max_messages: uint
        }
    }
)

macro_rules! window_component_init(
    ($name:ident, $color:expr, $max_messages:expr) => {
        fn new(bounds: Bound) -> $name {
            let height = bounds.max.y - bounds.min.y + 1;
            let width = bounds.max.x - bounds.min.x + 1;
            let console = Console::new(width as int, height as int);

            $name {
                console: console,
                background_color: $color,
                bounds: bounds,
                messages: vec![],
                max_messages: $max_messages
            }
        }
    }
)

pub mod util;
pub mod game;
pub mod traits;
pub mod actor;
pub mod rendering;
pub mod movement;
pub mod input;
