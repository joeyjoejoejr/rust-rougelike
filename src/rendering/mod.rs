extern crate tcod;
use self::tcod::{Console, BackgroundFlag, KeyState, TextAlignment, Color};

use util::{Point, Bound};

pub trait WindowComponent {
    fn new(Bound) -> Self;

    fn get_bounds(&self) -> Bound;
    fn get_bg_color(&self) -> Color;
    fn get_console(&mut self) -> &mut Console;

    fn clear(&mut self) {
        let color = self.get_bg_color();
        let console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print(&mut self, x: int, y: int, alignment: tcod::TextAlignment, text: &str) {
        let mut console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }
}

pub struct TcodStatsWindowComponent {
    pub console: Console,
    pub background_color: Color,
    bounds: Bound
}

impl WindowComponent for TcodStatsWindowComponent {
    fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);

        let red = Color::new(255u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }
}

pub struct TcodInputWindowComponent {
    pub console: Console,
    pub background_color: Color,
    bounds: Bound
}

impl WindowComponent for TcodInputWindowComponent {
    fn new(bounds: Bound) -> TcodInputWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);

        let green = Color::new(0u8, 255u8, 0u8);
        TcodInputWindowComponent {
            console: console,
            background_color: green,
            bounds: bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }
}

pub struct TcodMessagesWindowComponent {
    pub console: Console,
    pub background_color: Color,
    bounds: Bound
}

impl WindowComponent for TcodMessagesWindowComponent {
    fn new(bounds: Bound) -> TcodMessagesWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);

        let blue = Color::new(0u8, 0u8, 255u8);
        TcodMessagesWindowComponent {
            console: console,
            background_color: blue,
            bounds: bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }
}

pub struct TcodMapWindowComponent {
    pub console: Console,
    pub background_color: Color,
    bounds: Bound
}

impl WindowComponent for TcodMapWindowComponent {
    fn new(bounds: Bound) -> TcodMapWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as int, height as int);

        let black = Color::new(0u8, 0u8, 0u8);
        TcodMapWindowComponent {
            console: console,
            background_color: black,
            bounds: bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }
}

pub struct TcodRenderingComponent {
    pub console: Console
}

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyState;
    fn attach_window(&mut self, &mut Box<WindowComponent>);
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(bound: Bound) -> TcodRenderingComponent {
        let con = Console::init_root(
            (bound.max.x + 1) as int,
            (bound.max.y + 1) as int,
            "libtcod Rust tutorial",
            false
        );
        TcodRenderingComponent { console: con }
    }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol:char) {
        self.console.put_char(position.x as int, position.y as int, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }

    fn wait_for_keypress(&mut self) -> KeyState {
        Console::wait_for_keypress(true)
    }

    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        window.print(0, 0, tcod::Left, "Sup foo!");
        window.print(0, 1, tcod::Left, "Nothin fool!");
        let bounds  = window.get_bounds();
        let console = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as int) + 1,
        (bounds.max.y as int) + 1, &mut self.console, bounds.min.x as int,
        bounds.min.y as int, 1f32, 1f32);
    }
}
