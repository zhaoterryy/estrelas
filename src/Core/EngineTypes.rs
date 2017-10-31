use super::Math::Vector2::*;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Transform2D {
    pub Position: Vector2,
    pub Rotation: Vector2,
    pub Scale: Vector2
}

impl Transform2D {
    pub fn new(Position: Vector2, Rotation: Vector2, Scale: Vector2) -> Self {
        Transform2D {
            Position: Position,
            Rotation: Rotation,
            Scale: Scale
        }
    }
}