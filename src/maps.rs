use std::cell::RefCell;
use std::rc::Rc;

use util::{ Bound, Point };
use rendering::RenderingComponent;
use actor::Actor;
use game::Windows;
use movement::MoveInfo;

pub struct Maps<'a> {
    pub terrain: Box<Map<'a>>,
    pub enemies: Box<Map<'a>>,
    pub friends: Box<Map<'a>>,
    pub pcs: Box<Map<'a>>
}

impl<'a> Maps<'a> {
    pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> Maps<'a> {
        let terrain = box Map::new(move_info.clone());
        let enemies = box Map::new(move_info.clone());
        let friends = box Map::new(move_info.clone());
        let pcs = box Map::new(move_info.clone());

        Maps {
            friends: friends,
            enemies: enemies,
            terrain: terrain,
            pcs: pcs
        }
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.pcs.update(windows);
        self.terrain.update(windows);
        self.friends.update(windows);
        self.enemies.update(windows);
    }

    pub fn render(&mut self, renderer: &mut Box<RenderingComponent>) {
        self.terrain.render(renderer);
        self.friends.render(renderer);
        self.enemies.render(renderer);
        self.pcs.render(renderer);
    }

    pub fn enemy_at(&self, point: Point) -> Option<&Box<Actor>> {
        let enemies_at_point = &self.enemies.content[point.x as uint][point.y as uint];
        if enemies_at_point.len() > 0 {
            Some(&enemies_at_point[0])
        } else {
            None
        }
    }
}

pub struct Map<'a> {
    pub content: Vec<Vec<Vec<Box<Actor>>>>,
    size: Bound,
    move_info: Rc<RefCell<MoveInfo>>
}

impl<'a> Map<'a> {
    pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> Map<'a> {
        let size = {
            move_info.borrow().deref().bounds
        };
        let content = Map::init_contents(size);
        Map {
            content: content,
            size: size,
            move_info: move_info
        }
    }

    pub fn init_contents(size: Bound) -> Vec<Vec<Vec<Box<Actor>>>> {
        let mut contents: Vec<Vec<Vec<Box<Actor>>>> = vec![];
        for _ in range(0, size.max.x) {
            let mut x_vec: Vec<Vec<Box<Actor>>> = vec![];
            for _ in range(0, size.max.y) {
                let y_vec: Vec<Box<Actor>> = vec![];
                x_vec.push(y_vec);
            }
            contents.push(x_vec);
        }

        contents
    }

    pub fn push_actor(&mut self, point: Point, actor: Box<Actor>) {
        self.content[point.x as uint][point.y as uint].push(actor);
    }

    pub fn update(&mut self, windows: &mut Windows) {
        let mut new_content = Map::init_contents(self.size);
        for x_iter in self.content.iter_mut() {
            for y_iter in x_iter.iter_mut() {
                for actor in y_iter.iter_mut() {
                    actor.update(windows);
                    if actor.is_pc {
                        { self.move_info.borrow_mut().deref().char_location = actor.position };
                    }
                    let point = actor.position;
                    let new_actor = actor.clone();
                    new_content[point.x as uint][point.y as uint].push(new_actor);
                }
            }
        }
        self.content = new_content;
    }

    pub fn render(&mut self, renderer: &mut Box<RenderingComponent>) {
        for (x, x_iter) in  self.content.iter_mut().enumerate() {
            for (y, y_iter) in  x_iter.iter_mut().enumerate() {
                for actor in y_iter.iter_mut() {
                    let point = Point::new(x as i32, y as i32);
                    renderer.render_object(point, actor.display_char);
                }
            }
        }
    }
}
