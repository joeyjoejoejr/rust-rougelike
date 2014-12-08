use std::cell::RefCell;
use std::rc::Rc;

use util::Point;
use game::Windows;
use rendering::RenderingComponent;
use movement::{
    MoveInfo,
    MovementComponent,
    UserMovementComponent,
    RandomMovementComponent,
    AgroMovementComponent
};

pub struct Actor {
    pub position: Point,
    pub display_char: char,
    pub is_pc: bool,
    movement_component: Box<MovementComponent + 'static>,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, movement_component: Box<MovementComponent + 'static>, is_pc: bool) -> Actor {
        Actor { position: Point { x: x, y: y }, display_char: dc, movement_component: movement_component , is_pc: is_pc }
    }

    pub fn dog(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor {
        let mc: Box<RandomMovementComponent> = box MovementComponent::new(move_info.clone());
        Actor::new(x, y, 'd', mc, false)
    }

    pub fn cat(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor {
        let mc: Box<RandomMovementComponent> = box MovementComponent::new(move_info.clone());
        Actor::new(x, y, 'c', mc, false)
    }

    pub fn kobold(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor {
        let mc: Box<AgroMovementComponent> = box MovementComponent::new(move_info.clone());
        Actor::new(x, y, 'k', mc, false)
    }

    pub fn heroine(move_info: Rc<RefCell<MoveInfo>>) -> Actor {
        let point = { move_info.borrow().deref().char_location };
        let mc: Box<UserMovementComponent> = box MovementComponent::new(move_info.clone());
        Actor::new(point.x, point.y, '@', mc, true)
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.position = self.movement_component.update(self.position, windows);
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

impl Clone for Actor {
    fn clone(&self) -> Actor {
        let mc = self.movement_component.box_clone();
        Actor::new(self.position.x, self.position.y, self.display_char, mc, self.is_pc)
    }
}
