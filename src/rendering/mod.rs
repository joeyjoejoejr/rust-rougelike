extern crate tcod;
use self::tcod::{Console, BackgroundFlag, KeyState, TextAlignment, Color};

use util::{Point, Bound};

pub trait WindowComponent {
    fn new(Bound) -> Self;

    fn get_bounds(&self) -> Bound;
    fn get_bg_color(&self) -> Color;
    fn get_console(&mut self) -> &mut Console;
    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>>;
    fn get_messages(&self) -> Vec<Box<String>>;
    fn get_max_messages(&self) -> uint;

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

    fn buffer_message(&mut self, text: &str) {
        let max  = self.get_max_messages();
        let message = String::from_str(text);
        let messages = self.get_mut_messages();

        messages.insert(0, box message);
        messages.truncate(max);
    }

    fn flush_buffer(&mut self) {
        let max = self.get_max_messages();
        let messages = self.get_mut_messages();

        for _ in range(0, max) {
            messages.insert(0, box String::from_str(""));
        }
        messages.truncate(max);
    }
}

window_component_def!(TcodStatsWindowComponent)
impl WindowComponent for TcodStatsWindowComponent {
    window_component_init!(
        TcodStatsWindowComponent,
        Color::new(0u8, 0u8, 0u8),
        10u
    )
    window_component_getters!()
}

window_component_def!(TcodInputWindowComponent)
impl WindowComponent for TcodInputWindowComponent {
    window_component_init!(
        TcodInputWindowComponent,
        Color::new(0u8, 0u8, 0u8),
        2u
    )
    window_component_getters!()
}

window_component_def!(TcodMapWindowComponent)
impl WindowComponent for TcodMapWindowComponent {
    window_component_init!(
        TcodMapWindowComponent,
        Color::new(0u8, 0u8, 0u8),
        10u
    )
    window_component_getters!()
}

window_component_def!(TcodMessagesWindowComponent)
impl WindowComponent for TcodMessagesWindowComponent {
    window_component_init!(
        TcodMessagesWindowComponent,
        Color::new(0u8, 0u8, 0u8),
        10u
    )
    window_component_getters!()
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
        let mut line = 0i;
        let bounds  = window.get_bounds();
        let messages = window.get_messages();

        for message in messages.iter() {
            window.print(0, line, tcod::TextAlignment::Left, message.as_slice());
            line = line + 1;
        }

        let console = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as int) + 1,
        (bounds.max.y as int) + 1, &mut self.console, bounds.min.x as int,
        bounds.min.y as int, 1f32, 1f32);
    }
}
