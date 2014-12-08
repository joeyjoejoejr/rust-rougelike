extern crate tcod;
use self::tcod::{ Console, KeyState, BackgroundFlag };

use util::{Point, Bound};
use input::{ InputComponent, TcodInputComponent, KeyboardInput};
use window::WindowComponent;

pub struct TcodRenderingComponent<'a> {
    pub console: Console,
    pub input_component: Box<InputComponent<KeyState> + 'a>
}

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyboardInput;
    fn attach_window(&mut self, &mut Box<WindowComponent>);
}

impl<'a> RenderingComponent for TcodRenderingComponent<'a> {
    fn new(bound: Bound) -> TcodRenderingComponent<'a> {
        let console = Console::init_root(
            (bound.max.x + 1) as int,
            (bound.max.y + 1) as int,
            "libtcod Rust tutorial",
            false
        );
        let input_component: Box<TcodInputComponent> = box InputComponent::new();
        TcodRenderingComponent {
            console: console,
            input_component: input_component
        }
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

    fn wait_for_keypress(&mut self) -> KeyboardInput {
        let key_state = Console::wait_for_keypress(true);
        self.input_component.translate_input(key_state)
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
