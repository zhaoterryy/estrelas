
pub const PRETTY_SMALL_NUMBER: f32 = 1e-8f32;
pub const KINDA_SMALL_NUMBER: f32 = 1e-4f32;

use super::num::cast::ToPrimitive;
pub fn invsqrt<T: ToPrimitive>(n: T) -> f32
{
    1.0 / n.to_f32().unwrap().sqrt()
}