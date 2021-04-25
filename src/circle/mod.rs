use cgmath::{Vector3, Vector2};
use crate::Vertex;
use crate::Mesh;
use crate::Object2D;

pub struct Circle {
    resolution: u8,
    radius: f32,
}

impl Circle {
    pub fn new(resolution: u8, radius: f32) -> Circle {
        Circle {
            resolution: resolution,
            radius: radius,
        }
    }
}

impl Object2D<f32, f32, u8> for Circle {
    fn build_mesh(&self) -> Mesh<f32, f32, u8> { 
        let deg2rad = 0.0174533f32;

        let mut vertices = Vec::<Vertex<f32, f32>>::with_capacity(self.resolution as usize + 1);
        let mut indices  = Vec::<Vector3<u8>>::with_capacity(self.resolution as usize);
        let steps = 360.0 / (self.resolution as f32);

        for i in 0..self.resolution {
            let angle   = i as f32 * steps * deg2rad;
            let pos     = Vector3::new(angle.cos() * self.radius, angle.sin() * self.radius, 0.);
            let vert    = Vertex::new(pos, Vector2::new(0., 0.));
            let index   = Vector3::new(i, (i + 1) % self.resolution, self.resolution);

            indices.push(index);
            vertices.push(vert);
        }
        
        vertices.push(Vertex::new(Vector3::new(0., 0., 0.), Vector2::new(0., 0.)));

        Mesh::new(vertices, indices, false)
    }
}