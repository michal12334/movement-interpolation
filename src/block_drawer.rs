use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Program, Surface};
use nalgebra::{Matrix4, Vector3};

use crate::block::Block;

pub struct BlockDrawer {
    program: Program,
}

impl BlockDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 410 core

            in vec3 position;
            in vec3 normal;
            in vec3 color;

            out vec3 normal_out;
            out vec3 color_out;
            out vec3 world;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
                normal_out = mat3(model) * normal;
                color_out = color;
                world = (model * vec4(position, 1.0)).xyz;
            }
        "#;

        let fragment_shader_src = r#"
            #version 410 core

            in vec3 normal_out;
            in vec3 color_out;
            in vec3 world;

            out vec4 frag_color;

            const vec3 light_pos = vec3(10.0, 100.0, 10.0);

            uniform vec3 cam_pos;

            void main() {
                vec3 to_cam = normalize(cam_pos - world);
                vec3 to_light = normalize(light_pos - world);

                float ambient = 0.3;
                float diffuse =  max(dot(normal_out, to_light), 0.0);
                vec3 reflected = normalize(reflect(-to_light, normal_out));
                float specular = pow(max(dot(reflected, to_cam), 0.0), 50.0);

                frag_color = vec4((ambient + diffuse + specular) * color_out, 1.0);
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Self { program }
    }

    pub fn draw(
        &self,
        target: &mut glium::Frame,
        perspective: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
        camera_position: Vector3<f32>,
        block: &Block,
        drawing_parameters: &DrawParameters,
    ) {
        target
            .draw(
                block.vertices(),
                block.indices(),
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    view: view.data.0,
                    model: model.data.0,
                    cam_pos: camera_position.data.0[0],
                },
                &drawing_parameters,
            )
            .unwrap();
    }
}
