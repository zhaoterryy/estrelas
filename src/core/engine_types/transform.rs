use estrelas_math::vector2::Vector2;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Transform2D {
    pub position: Vector2,
    pub rotation: Vector2,
    pub scale: Vector2
}

impl Transform2D {
    pub fn new(position: Vector2, rotation: Vector2, scale: Vector2) -> Self {
        Transform2D {
            position: position,
            rotation: rotation,
            scale: scale
        }
    }
}