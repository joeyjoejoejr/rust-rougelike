use util::{Point, Bound};

pub trait MovementComponent {
    fn new(Bound) -> Self;
    fn update(&self, Point) -> Point;
}
