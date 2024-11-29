use std::f32::consts::PI;

use derive_getters::Getters;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, VertexBuffer};
use nalgebra::{Rotation3, Vector3};

use crate::vertex::Vertex;

#[derive(Debug, Getters)]
pub struct Block {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
}

impl Block {
    pub fn generate(divisions_count: u16, display: &Display<WindowSurface>) -> Self {
        let mut z_vertices = Vec::new();
        let mut z_indices = Vec::new();
        let radius = 1f32;
        let len = 5f32;
        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let normal = [0f32, 0f32, 1f32];
            let position = [radius * x, radius * y, 0f32];
            let color = [0f32, 0f32, 1f32];
            z_vertices.push(Vertex::new(position, normal, color));

            z_indices.push(i);
            z_indices.push((i + 1) % divisions_count);
            z_indices.push(divisions_count);
        }
        z_vertices.push(Vertex::new(
            [0f32, 0f32, 0f32],
            [0f32, 0f32, 1f32],
            [0f32, 0f32, 1f32],
        ));

        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let normal = [x, y, 0f32];
            let position = [radius * x, radius * y, 0f32];
            let color = [0f32, 0f32, 1f32];
            z_vertices.push(Vertex::new(position, normal, color));

            z_indices.push(divisions_count + 1 + i);
            z_indices.push(2 * divisions_count + 1 + i);
            z_indices.push(divisions_count + 1 + (i + 1) % divisions_count);

            z_indices.push(divisions_count + 1 + (i + 1) % divisions_count);
            z_indices.push(2 * divisions_count + 1 + i);
            z_indices.push(2 * divisions_count + 1 + (i + 1) % divisions_count);
        }

        for i in 0..divisions_count {
            let a = (i as f32 / (divisions_count - 1) as f32) * 2f32 * PI;
            let x = a.cos();
            let y = a.sin();
            let z = -len;
            let normal = [x, y, 0f32];
            let position = [radius * x, radius * y, z];
            let color = [0f32, 0f32, 1f32];
            z_vertices.push(Vertex::new(position, normal, color));

            z_indices.push(2 * divisions_count + 1 + i);
            z_indices.push(3 * divisions_count + 1);
            z_indices.push(2 * divisions_count + 1 + (i + 1) % divisions_count);
        }
        z_vertices.push(Vertex::new(
            [0f32, 0f32, -len - radius],
            [0f32, 0f32, 1f32],
            [0f32, 0f32, 1f32],
        ));

        let x_vertices = z_vertices
            .iter()
            .map(|v| {
                let r = Rotation3::from_euler_angles(0f32, PI / 2f32, 0f32);
                let p = Vector3::new(v.position()[0], v.position()[1], v.position()[2]);
                let n = Vector3::new(v.normal()[0], v.normal()[1], v.normal()[2]);

                let p = r * p;
                let n = r * n;

                let c = [1f32, 0f32, 0f32];

                Vertex::new(p.data.0[0], n.data.0[0], c)
            })
            .collect::<Vec<_>>();

        let x_indices = z_indices
            .iter()
            .map(|i| i + z_vertices.len() as u16)
            .collect::<Vec<_>>();

        let y_vertices = z_vertices
            .iter()
            .map(|v| {
                let r = Rotation3::from_euler_angles(PI / 2f32, 0f32, 0f32);
                let p = Vector3::new(v.position()[0], v.position()[1], v.position()[2]);
                let n = Vector3::new(v.normal()[0], v.normal()[1], v.normal()[2]);

                let p = r * p;
                let n = r * n;

                let c = [0f32, 1f32, 0f32];

                Vertex::new(p.data.0[0], n.data.0[0], c)
            })
            .collect::<Vec<_>>();

        let y_indices = z_indices
            .iter()
            .map(|i| i + 2 * z_vertices.len() as u16)
            .collect::<Vec<_>>();

        Self {
            vertices: VertexBuffer::new(display, &[z_vertices, x_vertices, y_vertices].concat())
                .unwrap(),
            indices: IndexBuffer::new(
                display,
                PrimitiveType::TrianglesList,
                &[z_indices, x_indices, y_indices].concat(),
            )
            .unwrap(),
        }
    }
}
