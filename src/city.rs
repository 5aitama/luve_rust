
use cgmath::Vector2;

pub struct City {
    pub position: Vector2<f32>,
}

impl City {
    pub fn new(position: Vector2::<f32>) -> City {
        City { position: position }
    }
}

impl std::clone::Clone for City {
    
    fn clone(&self) -> Self {
        City { position: self.position }
    }
}