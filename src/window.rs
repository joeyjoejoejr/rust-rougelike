extern crate tcod;
use self::tcod::{Console, KeyState, BackgroundFlag, TextAlignment, Color};

use util::Bound;
use input::InputComponent;

pub struct TcodRenderingComponent<'a> {
    pub console: Console,
    pub input_component: Box<InputComponent<KeyState> + 'a>
}

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

