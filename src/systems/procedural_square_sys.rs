use crate::core::gl::shader::Shader;

use cgmath::{
    Vector2,
    Vector3,
};

use crate::core::{
    sys::Sys,
    world_state::WorldState,
    component_tuple::ComponentTuple,
};

use crate::components::{
    mesh_2d::Vertex2D,
    mesh_2d::Mesh2D,
    window::Window,
};

pub struct ProceduralSquareSys;
impl Sys for ProceduralSquareSys {
    
    fn on_start(&self, world_state: &mut WorldState) {
        let e = world_state.create_entity().unwrap();

        let shader = Shader::new("./shaders/colorfull/vert.glsl", "./shaders/colorfull/frag.glsl").ok().unwrap();

        let vertices = vec![
            Vertex2D::new(Vector2::new(-1.0, -1.0), Vector2::new(-0.5, -0.5), Vector2::new(0.0, 0.0)),
            Vertex2D::new(Vector2::new(-1.0,  1.0), Vector2::new(-0.5,  0.5), Vector2::new(0.0, 1.0)),
            Vertex2D::new(Vector2::new( 1.0,  1.0), Vector2::new( 0.5,  0.5), Vector2::new(1.0, 1.0)),
            Vertex2D::new(Vector2::new( 1.0, -1.0), Vector2::new( 0.5, -0.5), Vector2::new(1.0, 0.0)),
        ];

        let indices: Vec<Vector3<u16>> = vec![
            Vector3::new(0, 1, 2),
            Vector3::new(0, 2, 3),
        ];

        let mesh2d = Mesh2D::new(vertices, indices, shader, false);

        world_state.add_component(&e, mesh2d);
    }

    fn on_update(&self, world_state: &mut WorldState) {
        // Retrieve the window component from the window entity...
        let window_entity = <(Window,)>::get_single_entity(world_state).unwrap();
        let window = world_state.get_component_ref::<Window>(&window_entity).unwrap();

        // Get time and resolution from the window
        let time = window.glfw.get_time() as f32;
        let res  = window.raw.get_framebuffer_size();

        // set the time and resolution value to the shader of all entities
        // that have the Mesh2D component !
        for e in <(Mesh2D,)>::get_entities(world_state).iter() {
            let mesh = world_state.get_component_ref::<Mesh2D>(&e).unwrap();

            mesh.shader.set_float("time", time);
            mesh.shader.set_vec2("screen_resolution", &[Vector2::new(res.0 as f32, res.1 as f32)]);
        }
    }
}