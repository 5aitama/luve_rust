use crate::Vertex;
use gl::types::{ GLsizei, GLsizeiptr };
use cgmath::{ Vector2, Vector3 };

pub struct FSQ;

impl FSQ {
    pub fn new() -> FSQ {
        FSQ {}
    }
}

pub trait Object2D<V: num::Num, U: num::Num, I: num::Integer> {
    fn build_mesh(&self) -> Mesh<V, U, I>;
}

pub struct Mesh<V: num::Num, U: num::Num, I: num::Integer> {
    pub vertices : Vec<Vertex<V, U>>,
    pub indices  : Vec<Vector3<I>>,
    vbo: u32,
    vao: u32,
    ebo: u32,
}

impl Object2D<f32, f32, u8> for FSQ {
    fn build_mesh(&self) -> Mesh::<f32, f32, u8> {
        // Create a full screen quad (FSQ)
        // The vertices and uvs of our fsq...
        let vertices: Vec<Vertex<f32, f32>> = [
            Vertex::new(Vector3::new(-1.0, -1.0, 0.0), Vector2::new(0.0, 0.0)),
            Vertex::new(Vector3::new(-1.0,  1.0, 0.0), Vector2::new(0.0, 1.0)),
            Vertex::new(Vector3::new( 1.0,  1.0, 0.0), Vector2::new(1.0, 1.0)),
            Vertex::new(Vector3::new( 1.0, -1.0, 0.0), Vector2::new(1.0, 0.0)),
        ].to_vec();

        // The indices of our fsq geometry.
        let indices: Vec<Vector3<u8>> = [
            Vector3::new(0, 1, 2),
            Vector3::new(0, 2, 3),
        ].to_vec();
        
        // Build our fsq mesh from the vertices and indices...
        Mesh::<f32, f32, u8>::new(vertices, indices, false)
    }
}

impl<V: num::Num, U: num::Num, I: num::Integer> Mesh<V, U, I> {
    pub fn new(vertices : Vec<Vertex<V, U>>, indices: Vec<Vector3<I>>, is_dynamic: bool) -> Mesh<V, U, I> {
        let mut mesh = Mesh {
            vertices : vertices,
            indices  : indices,
            vbo: 0,
            vao: 0,
            ebo: 0,
        };
        
        unsafe { mesh.init_buffers(is_dynamic) };
        mesh
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
        }
    }

    unsafe fn init_buffers(&mut self, is_dynamic: bool) {
        
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        let i_buff_size = self.i_buff_size();
        let buff_size = self.vertices.len() * std::mem::size_of::<Vertex<V, U>>();
        let draw_mode = if is_dynamic { gl::DYNAMIC_DRAW } else { gl::STATIC_DRAW };

        let v_size = std::mem::size_of::<Vector3<V>>();
        let u_size = std::mem::size_of::<Vector2<U>>();
        let stride = (v_size + u_size) as GLsizei;

        gl::BufferData(gl::ARRAY_BUFFER, buff_size as isize, std::ptr::null(), draw_mode);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, buff_size as isize, &self.vertices[0] as *const Vertex::<V, U> as *const gl::types::GLvoid);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, i_buff_size, &self.indices[0] as *const Vector3::<I> as *const gl::types::GLvoid, draw_mode);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::TRUE, stride, v_size as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    /// The index buffer size.
    fn i_buff_size(&self) -> GLsizeiptr {
        (self.indices.len() * std::mem::size_of::<Vector3::<I>>()) as GLsizeiptr
    }
}