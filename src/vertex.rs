use derive_getters::Getters;
use derive_new::new;
use glium::implement_vertex;

#[derive(Debug, Clone, Copy, Getters, new)]
pub struct SimpleVertex {
    position: [f32; 3],
}

implement_vertex!(SimpleVertex, position);

#[derive(Debug, Clone, Copy, Getters, new)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, normal, color);
