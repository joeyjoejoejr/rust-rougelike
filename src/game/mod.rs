extern crate tcod;
use self::tcod::KeyState;

use util::{ Point, Bound };
use rendering::{
    WindowComponent,
    RenderingComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMessagesWindowComponent,
    TcodMapWindowComponent
};
use actor::Actor;

static mut LAST_KEYPRESS: Option<KeyState> = None;
static mut CHAR_LOCATION: Point = Point { x: 40, y: 25 };

pub struct Game<'a> {
    pub window_bounds: Bound,
    pub exit: bool,
    pub rendering_component: Box<RenderingComponent + 'a>,
    pub stats_window: Box<WindowComponent + 'a>,
    pub input_window: Box<WindowComponent + 'a>,
    pub message_window: Box<WindowComponent + 'a>,
    pub map_window: Box<WindowComponent + 'a>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let total_bounds   = Bound::new(0,  0, 99, 61);
        let stats_bounds   = Bound::new(79, 0, 99, 49);
        let input_bounds   = Bound::new(0, 50, 99, 52);
        let message_bounds = Bound::new(0, 53, 99, 61);
        let map_bounds     = Bound::new(0,  0, 78, 49);

        let rc: Box<RenderingComponent> = box RenderingComponent::new(total_bounds);
        let sw: Box<TcodStatsWindowComponent> = box WindowComponent::new(stats_bounds);
        let iw: Box<TcodInputWindowComponent> = box WindowComponent::new(input_bounds);
        let mw: Box<TcodMessagesWindowComponent> = box WindowComponent::new(message_bounds);
        let maw: Box<TcodMapWindowComponent> = box WindowComponent::new(map_bounds);

        Game {
            exit: false,
            window_bounds: total_bounds,
            rendering_component: rc,
            stats_window: sw,
            input_window: iw,
            message_window: mw,
            map_window: maw
        }
    }

    pub fn update(&self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
        c.update();
        Game::set_character_location(c.position);
        for npc in npcs.iter_mut() {
            npc.update();
        }
    }

    pub fn render(&mut self, npcs: &mut Vec<Box<Actor>>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        self.rendering_component.attach_window(&mut self.stats_window);
        self.rendering_component.attach_window(&mut self.input_window);
        self.rendering_component.attach_window(&mut self.message_window);
        self.rendering_component.attach_window(&mut self.map_window);

        for npc in npcs.iter() {
            npc.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn wait_for_keypress(&mut self) -> KeyState {
        let ks = self.rendering_component.wait_for_keypress();
        Game::set_last_keypress(ks);
        ks
    }

    pub fn get_last_keypress() -> Option<KeyState> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(ks: KeyState) {
        unsafe { LAST_KEYPRESS = Some(ks);}
    }

    pub fn get_character_location() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_location(point: Point) {
        unsafe { CHAR_LOCATION = point; }
    }
}
