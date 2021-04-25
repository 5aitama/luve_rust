use gl::types::GLsizei;
use gl::types::GLsizeiptr;
use cgmath::{ Vector2, Vector3 };

pub struct Mesh<V: num::Num, U: num::Num, I: num::Integer> {
    pub vertices : Vec<Vector3<V>>,
    pub uvs      : Vec<Vector2<U>>,
    pub indices  : Vec<Vector3<I>>,
    vbo: u32,
    vao: u32,
    ebo: u32,
}

impl<V: num::Num, U: num::Num, I: num::Integer> Mesh<V, U, I> {
    pub fn new(vertices : Vec<Vector3<V>>, uvs: Vec<Vector2<U>>, indices: Vec<Vector3<I>>, is_dynamic: bool) -> Mesh<V, U, I> {
        let mut mesh = Mesh {
            vertices : vertices,
            indices  : indices,
            uvs      : uvs,
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
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32 * 3, gl::UNSIGNED_BYTE, std::ptr::null());
        }
    }

    unsafe fn init_buffers(&mut self, is_dynamic: bool) {
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        let v_buff_size = self.v_buff_size();
        let u_buff_size = self.u_buff_size();
        let i_buff_size = self.i_buff_size();
        let buff_size = v_buff_size + u_buff_size;
        let draw_mode = if is_dynamic { gl::DYNAMIC_DRAW } else { gl::STATIC_DRAW };

        let v_size = std::mem::size_of::<Vector3<V>>();
        let u_size = std::mem::size_of::<Vector2<U>>();
        let stride = (v_size + u_size) as GLsizei;

        gl::BufferData(gl::ARRAY_BUFFER, buff_size, std::ptr::null(), draw_mode);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, v_buff_size, &self.vertices[0] as *const Vector3::<V> as *const gl::types::GLvoid);
        gl::BufferSubData(gl::ARRAY_BUFFER, v_buff_size, u_buff_size, &self.uvs[0] as *const Vector2::<U> as *const gl::types::GLvoid);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, i_buff_size, &self.indices[0] as *const Vector3::<I> as *const gl::types::GLvoid, draw_mode);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::TRUE, stride, v_size as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    /// The vertex buffer size.
    fn v_buff_size(&self) -> GLsizeiptr {
        (self.vertices.len() * std::mem::size_of::<Vector3::<V>>()) as GLsizeiptr
    }

    /// The index buffer size.
    fn i_buff_size(&self) -> GLsizeiptr {
        (self.indices.len() * std::mem::size_of::<Vector3::<I>>()) as GLsizeiptr
    }

    /// The uv buffer size.
    fn u_buff_size(&self) -> GLsizeiptr {
        (self.uvs.len() * std::mem::size_of::<Vector2::<U>>()) as GLsizeiptr
    }
}