mod animation_data;
mod infinite_grid_drawer;
mod vertex;

use animation_data::{AnimationData, QuternionInterpolationType};
use chrono::Local;
use egui::{
    emath, Button, Checkbox, DragValue, Label, RadioButton, RichText, ViewportId, WidgetText,
};
use egui_flex::{item, Flex};
use glium::{Blend, Rect, Surface};
use infinite_grid_drawer::InfiniteGridDrawer;
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use winit::event::{self, ElementState, MouseButton};

fn main() {
    let mut width = 1600;
    let mut height = 1200;

    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Movement interpolation")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium =
        egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let mut drawing_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    let mut perspective = Matrix4::new_perspective(
        width as f32 / height as f32,
        std::f32::consts::PI / 2.0,
        0.1,
        100.0,
    );

    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 5.0f32;
    let mut view = Matrix4::look_at_rh(
        &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
        &Point3::new(0.0, 0.0, 0.0),
        &camera_up,
    );
    let mut camera_move_button_pressed = false;

    let infinite_grid_drawer = InfiniteGridDrawer::new(&display);

    let mut animation_data = AnimationData::default();

    let mut previous_time = Local::now();

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        let mut redraw = || {
            let current_time = Local::now();
            let duration = current_time - previous_time;
            let duration_in_seconds = duration.num_microseconds().unwrap_or(1) as f64 / 1_000_000.0;
            let fps = 1.0 / duration_in_seconds;
            previous_time = current_time;

            build_ui(&mut egui_glium, &window, &mut animation_data, fps);

            window.request_redraw();

            let mut target = display.draw();

            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            drawing_parameters.viewport = Some(Rect {
                left: 0,
                bottom: 0,
                width: width / 2,
                height: height,
            });

            infinite_grid_drawer.draw(&mut target, &perspective, &view, &drawing_parameters);

            drawing_parameters.viewport = Some(Rect {
                left: width / 2,
                bottom: 0,
                width: width / 2,
                height: height,
            });

            infinite_grid_drawer.draw(&mut target, &perspective, &view, &drawing_parameters);

            egui_glium.paint(&display, &mut target);

            target.finish().unwrap();
        };

        match event {
            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::RedrawRequested => redraw(),
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        window_target.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        perspective = Matrix4::new_perspective(
                            new_size.width as f32 / new_size.height as f32,
                            std::f32::consts::PI / 2.0,
                            0.1,
                            100.0,
                        );
                        width = new_size.width;
                        height = new_size.height;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta = (position.x - mouse_position.0, position.y - mouse_position.1);
                        mouse_position = (position.x, position.y);
                        if camera_move_button_pressed {
                            camera_angle.x += delta.1 as f32 * 0.01;
                            camera_angle.y += delta.0 as f32
                                * 0.01
                                * if camera_angle.x.cos() < 0.0 {
                                    -1.0
                                } else {
                                    1.0
                                };
                            camera_direction =
                                (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0)
                                    * Vector4::new(0.0, 0.0, 1.0, 0.0))
                                .xyz();
                            camera_up =
                                (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0)
                                    * Vector4::new(0.0, 1.0, 0.0, 0.0))
                                .xyz();
                            view = Matrix4::look_at_rh(
                                &Point3::from_slice(
                                    (-camera_distant * camera_direction).as_slice(),
                                ),
                                &Point3::new(0.0, 0.0, 0.0),
                                &camera_up,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle {
                            camera_move_button_pressed = *state == ElementState::Pressed;
                        }
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => {
                        if event.logical_key == "c" && event.state.is_pressed() && !event.repeat {
                            camera_move_button_pressed = !camera_move_button_pressed;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        event::MouseScrollDelta::LineDelta(_x, y) => {
                            camera_distant += -y * 0.1;
                            view = Matrix4::look_at_rh(
                                &Point3::from_slice(
                                    (-camera_distant * camera_direction).as_slice(),
                                ),
                                &Point3::new(0.0, 0.0, 0.0),
                                &camera_up,
                            );
                        }
                        _ => {}
                    },
                    WindowEvent::PinchGesture { delta, .. } => {
                        camera_distant -= *delta as f32 * 3.0;
                        view = Matrix4::look_at_rh(
                            &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
                            &Point3::new(0.0, 0.0, 0.0),
                            &camera_up,
                        );
                    }
                    _ => {}
                }

                let event_response = egui_glium.on_event(&window, &event);

                if event_response.repaint {
                    window.request_redraw();
                }
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

fn build_ui(
    egui_glium: &mut egui_glium::EguiGlium,
    window: &winit::window::Window,
    animation_data: &mut AnimationData,
    fps: f64,
) {
    egui_glium.run(window, |egui_ctx| {
        egui::Window::new("panel")
            .auto_sized()
            .show(egui_ctx, |ui| {
                Flex::horizontal()
                    .grow_items(1.0)
                    .align_items(egui_flex::FlexAlign::Stretch)
                    .show(ui, |flex| {
                        flex.add_flex(item(), Flex::vertical(), |flex| {
                            flex.add_flex(item(), Flex::horizontal(), |flex| {
                                build_xyz_settings(
                                    flex,
                                    &mut animation_data.begin_position,
                                    RichText::new("Begin Position").size(15f32),
                                );
                                build_xyz_settings(
                                    flex,
                                    &mut animation_data.end_position,
                                    RichText::new("End Position").size(15f32),
                                );
                            });

                            flex.add(
                                item().align_self(egui_flex::FlexAlign::Start),
                                Checkbox::new(
                                    &mut animation_data.display_all_frames,
                                    "Display all frames",
                                ),
                            );
                            build_number_settings(
                                flex,
                                &mut animation_data.number_of_frames,
                                "Number of frames",
                            );
                            build_number_settings(
                                flex,
                                &mut animation_data.animation_time,
                                "Animation time",
                            );
                        });

                        flex.add_flex(item(), Flex::vertical(), |flex| {
                            flex.add_flex(item(), Flex::horizontal(), |flex| {
                                build_wxyz_settings(
                                    flex,
                                    &mut animation_data.begin_rotation_quaternion,
                                    RichText::new("Begin Quternion").size(15f32),
                                );
                                build_wxyz_settings(
                                    flex,
                                    &mut animation_data.end_rotation_quaternion,
                                    RichText::new("End Quternion").size(15f32),
                                );
                            });

                            if flex
                                .add(
                                    item().align_self(egui_flex::FlexAlign::Start),
                                    RadioButton::new(
                                        animation_data.quternion_interpolation_type
                                            == QuternionInterpolationType::Linear,
                                        "Linear",
                                    ),
                                )
                                .inner
                                .clicked()
                            {
                                animation_data.quternion_interpolation_type =
                                    QuternionInterpolationType::Linear;
                            }

                            if flex
                                .add(
                                    item().align_self(egui_flex::FlexAlign::Start),
                                    RadioButton::new(
                                        animation_data.quternion_interpolation_type
                                            == QuternionInterpolationType::Spherical,
                                        "Spherical",
                                    ),
                                )
                                .inner
                                .clicked()
                            {
                                animation_data.quternion_interpolation_type =
                                    QuternionInterpolationType::Spherical;
                            }

                            flex.add(item(), Button::new("run"));
                        });

                        flex.add_flex(item(), Flex::vertical(), |flex| {
                            flex.add_flex(item(), Flex::horizontal(), |flex| {
                                build_xyz_settings(
                                    flex,
                                    &mut animation_data.begin_rotation_xyz,
                                    RichText::new("Begin Euler Angle").size(15f32),
                                );
                                build_xyz_settings(
                                    flex,
                                    &mut animation_data.end_rotation_xyz,
                                    RichText::new("End Euler Angle").size(15f32),
                                );
                            });

                            flex.add(item(), Button::new("run"));
                        });
                    });
                ui.label(RichText::new(format!("FPS: {:.1}", fps)).size(15f32));
            });
    });
}

fn build_xyz_settings(
    flex: &mut egui_flex::FlexInstance<'_>,
    postion: &mut (f32, f32, f32),
    title: impl Into<WidgetText>,
) {
    flex.add_flex(item(), Flex::vertical(), |flex| {
        flex.add(item(), Label::new(title).extend());
        build_number_settings(flex, &mut postion.0, "X");
        build_number_settings(flex, &mut postion.1, "Y");
        build_number_settings(flex, &mut postion.2, "Z");
    });
}

fn build_wxyz_settings(
    flex: &mut egui_flex::FlexInstance<'_>,
    postion: &mut (f32, f32, f32, f32),
    title: impl Into<WidgetText>,
) {
    flex.add_flex(item(), Flex::vertical(), |flex| {
        flex.add(item(), Label::new(title).extend());
        build_number_settings(flex, &mut postion.0, "W");
        build_number_settings(flex, &mut postion.1, "X");
        build_number_settings(flex, &mut postion.2, "Y");
        build_number_settings(flex, &mut postion.3, "Z");
    });
}

fn build_number_settings<Num: emath::Numeric>(
    flex: &mut egui_flex::FlexInstance<'_>,
    num: &mut Num,
    name: impl Into<WidgetText>,
) {
    flex.add_flex(item(), Flex::horizontal(), |flex| {
        flex.add(item().grow(1.0), DragValue::new(num));
        flex.add(item(), Label::new(name).extend());
    });
}
