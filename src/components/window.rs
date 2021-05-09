extern crate glfw;
use std::sync::mpsc::Receiver;

pub struct Window {
    pub size: (u32, u32),
    pub title: String,
    pub event: Receiver<(f64, glfw::WindowEvent)>,
    pub raw: glfw::Window,
    pub glfw: glfw::Glfw,
}