//! Evolution will be a 2D genetic algorithm simulation.

use ggez::nalgebra::Point2;

pub mod fish;
pub mod food;

/// Used by the Generic Function `Fish.consume()` to represent a piece of food or a fish
pub trait Entity {
    /// Returns the currently location of the entity
    fn pos(&self) -> Point2<f32>;
    /// Returns the radius of the entity
    fn radius(&self) -> f32;
}

/// Mapping function based on the `map()` function in Processing.
/// This map function will give the inverse result though so things can be inversely
/// proportional.
pub fn inverse_map_range(value: f32, range1: (f32, f32), range2: (f32, f32)) -> f32 {
    range2.1 - (range2.1 - range2.0) * ((value - range1.0) / (range1.1 - range1.0))
}

/// Testing the inverse map function
#[test]
fn test_inverse_map_range() {
    // The mapped inverse of 1/3 onto 3-12, is 2/3 of the total, or 6/9 resulting in the value 9
    // due to the offset of 3
    let value = inverse_map_range(1.0, (0.0, 3.0), (3.0, 12.0));
    assert_eq!(value, 9.0);
}
