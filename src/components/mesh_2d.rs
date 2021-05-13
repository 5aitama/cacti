
use crate::core::gl::shader::Shader;
use gl::types::{
    GLsizei,
    GLsizeiptr,
};

use cgmath::{
    Vector2,
    Vector3,
};

/// A two dimensional vertex representation.
pub struct Vertex2D {
    /// The vertex position.
    pub pos:    Vector2<f32>,
    
    /// The vertex normal.
    pub norm:   Vector2<f32>,

    /// The vertex uv coordinates.
    pub uv:     Vector2<f32>,
}

impl Vertex2D {
    /// Create new `Vertex2D`
    /// # Arguments
    /// * `pos` - The vertex position.
    /// * `norm` - The vertex normal.
    /// * `uv` - The vertex uv coordinates.
    pub fn new(pos: Vector2<f32>, norm: Vector2<f32>, uv: Vector2<f32>) -> Self {
        Self {
            pos,
            norm,
            uv,
        }
    }
}

pub struct Mesh2D {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<Vector3<u16>>,
    pub vbo: u32,
    pub vao: u32,
    pub ebo: u32,
    pub shader: Shader,
}

impl Mesh2D {
    pub fn new(vertices: Vec<Vertex2D>, indices: Vec<Vector3<u16>>, shader: Shader, is_dynamic: bool) -> Self {
        let mut mesh2d = Self {
            vertices,
            indices,
            shader,
            vbo: 0,
            vao: 0,
            ebo: 0,
        };

        unsafe { mesh2d.init_buffers(is_dynamic); }
        mesh2d
    }

    unsafe fn init_buffers(&mut self, is_dynamic: bool) {
        
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        let i_buff_size = self.i_buff_size();
        let buff_size = self.vertices.len() * std::mem::size_of::<Vertex2D>();
        let draw_mode = if is_dynamic { gl::DYNAMIC_DRAW } else { gl::STATIC_DRAW };

        //let v_size = std::mem::size_of::<Vector2<f32>>();
        //let n_size = std::mem::size_of::<Vector2<f32>>();
        //let u_size = std::mem::size_of::<Vector2<f32>>();
        let stride = std::mem::size_of::<Vertex2D>() as GLsizei; //(v_size + n_size + u_size) as GLsizei;

        gl::BufferData(gl::ARRAY_BUFFER, buff_size as isize, std::ptr::null(), draw_mode);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, buff_size as isize, &self.vertices[0] as *const Vertex2D as *const gl::types::GLvoid);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, i_buff_size, &self.indices[0] as *const Vector3::<u16> as *const gl::types::GLvoid, draw_mode);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::TRUE, stride, std::mem::size_of::<Vector2<f32>>() as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::TRUE, stride, (std::mem::size_of::<Vector2<f32>>() * 2) as *const gl::types::GLvoid);
        gl::EnableVertexAttribArray(2);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    /// The index buffer size.
    fn i_buff_size(&self) -> GLsizeiptr {
        (self.indices.len() * std::mem::size_of::<Vector3::<u16>>()) as GLsizeiptr
    }

    pub fn draw(&self) {
        unsafe {
            self.shader.use_it();
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32 * 3, gl::UNSIGNED_SHORT, std::ptr::null());
        }
    }
}