
use cgmath::Deg;
use cgmath::Vector2;

/// Represent a simple Ant!
pub struct Ant {

    /// The ant position (in 2D space)
    position: Vector2<f32>,
    
    // The ant orientation (in degree)
    orientation: Deg<f32>,

}