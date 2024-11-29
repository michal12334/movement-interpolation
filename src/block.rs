use std::f32::consts::PI;

use derive_getters::Getters;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, VertexBuffer};

use crate::vertex::Vertex;

#[derive(Debug, Getters)]
pub struct Block {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
}

impl Block {
    pub fn generate(divisions_count: u16, display: &Display<WindowSurface>) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let radius = 1f32;
        let len = 5f32;
        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let normal = [0f32, 0f32, -1f32];
            let position = [radius * x, radius * y, 0f32];
            let color = [0f32, 0f32, 1f32];
            vertices.push(Vertex::new(position, normal, color));

            indices.push(i);
            indices.push((i + 1) % divisions_count);
            indices.push(divisions_count);
        }
        vertices.push(Vertex::new(
            [0f32, 0f32, 0f32],
            [0f32, 0f32, -1f32],
            [0f32, 0f32, 1f32],
        ));

        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let normal = [x, y, 0f32];
            let position = [radius * x, radius * y, 0f32];
            let color = [0f32, 0f32, 1f32];
            vertices.push(Vertex::new(position, normal, color));

            indices.push(divisions_count + 1 + i);
            indices.push(divisions_count + 1 + (i + 1) % divisions_count);
            indices.push(2 * divisions_count + 1 + i);

            indices.push(divisions_count + 1 + (i + 1) % divisions_count);
            indices.push(2 * divisions_count + 1 + i);
            indices.push(2 * divisions_count + 1 + (i + 1) % divisions_count);
        }

        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let z = len;
            let normal = [x, y, 0f32];
            let position = [radius * x, radius * y, z];
            let color = [0f32, 0f32, 1f32];
            vertices.push(Vertex::new(position, normal, color));

            indices.push(2 * divisions_count + 1 + i);
            indices.push(2 * divisions_count + 1 + (i + 1) % divisions_count);
            indices.push(3 * divisions_count + 1);
        }
        vertices.push(Vertex::new(
            [0f32, 0f32, len + radius],
            [0f32, 0f32, 1f32],
            [0f32, 0f32, 1f32],
        ));

        Self {
            vertices: VertexBuffer::new(display, &vertices).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap(),
        }
    }
}
