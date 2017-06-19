
#[derive(Clone)]
pub enum Shape {
    O,
    I,
    Z,
    S,
    L,
    J,
    T,
}

pub static ALL_SHAPES: [Shape; 7] = [Shape::O, Shape::I, Shape::Z, Shape::S, Shape::L, Shape::J,
                                     Shape::T];
