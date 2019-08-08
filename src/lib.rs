//! Evolution will be a 2D genetic algorithm simulation.

pub mod fish;
pub mod food;

pub fn inverse_map_range(value: f32, range1: (f32, f32), range2: (f32, f32)) -> f32 {
    range2.1 - (range2.1 - range2.0) * ((value - range1.0) / (range1.1 - range1.0))
}

#[test]
fn test_inverse_map_range() {
    let value = inverse_map_range(1.0, (0.0, 3.0), (3.0, 12.0));
    assert_eq!(value, 9.0);
}
