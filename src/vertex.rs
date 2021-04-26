use cgmath::{Vector2, Vector3};

/// Represent a vertex.
pub struct Vertex<V: num::Num, U: num::Num> {
    /// The vertex position.
    pub position: Vector3<V>,

    /// The vertex uv coordinates.
    pub uv: Vector2<U>,
}

impl<V: num::Num, U: num::Num> Vertex<V, U> {
    /// Create new Vertex.
    /// 
    /// # Arguments
    /// * `position` - The vertex position.
    /// * `uv` - The vertex uv coordinates.
    pub fn new(position: Vector3<V>, uv: Vector2<U>) -> Vertex<V, U> {
        Vertex {
            position: position,
            uv: uv,
        }
    }
}

impl<V: num::Num + Copy, U: num::Num + Copy> std::clone::Clone for Vertex<V, U> {
    fn clone(&self) -> Self {
        Vertex {
            position: self.position,
            uv: self.uv,
        }
    }
}