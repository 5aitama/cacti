use cacti::core::{
    world::World,
};

use cacti::systems::{
    window_sys::WindowSys,
    before_render_sys::BeforeRenderSys,
    render_sys::RenderSys,
    after_render_sys::AfterRenderSys,
    procedural_square_sys::ProceduralSquareSys,
};

extern crate glfw;


fn main() {

    let mut world = World::new(64, 64, 64, 5);
    world.add_system(BeforeRenderSys);
    world.add_system(RenderSys);
    world.add_system(AfterRenderSys);

    world.add_system(WindowSys::new("Color Gradient", (800, 600)));
    world.add_system(ProceduralSquareSys);
    world.start_loop()
}