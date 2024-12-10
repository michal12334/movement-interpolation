use std::f32::consts::PI;

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use egui::emath::normalized_angle;
use nalgebra::{Matrix4, Quaternion, Rotation3, UnitQuaternion, Vector3};

use crate::animation_data::QuaternionInterpolationType;

pub trait Animation {
    fn get_quaternion_frames(&self) -> Vec<Matrix4<f32>>;
    fn get_euler_frames(&self) -> Vec<Matrix4<f32>>;
    fn make_step(&mut self, time_elapsed: f64);
}

#[derive(Debug, Clone, new)]
pub enum AnimationAngle {
    Quternion(Quaternion<f32>),
    Euler(Vector3<f32>),
}

#[derive(Debug, Clone, Getters, new, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct DiscreteFrameAnimation {
    begin_position: Vector3<f32>,
    end_position: Vector3<f32>,
    frames_count: u8,
    begin_angle: AnimationAngle,
    end_angle: AnimationAngle,
    quaternion_interpolation_type: QuaternionInterpolationType,

    #[builder(setter(skip))]
    quaternion_frames: Option<Vec<Matrix4<f32>>>,
    #[builder(setter(skip))]
    euler_frames: Option<Vec<Matrix4<f32>>>,
}

#[derive(Debug, Clone, Getters, new, Builder)]
pub struct ContinuousAnimation {
    begin_position: Vector3<f32>,
    end_position: Vector3<f32>,
    animation_time: f64,
    begin_angle: AnimationAngle,
    end_angle: AnimationAngle,
    quaternion_interpolation_type: QuaternionInterpolationType,

    #[builder(setter(skip))]
    time_elapsed: f64,
}

impl Animation for DiscreteFrameAnimation {
    fn get_quaternion_frames(&self) -> Vec<Matrix4<f32>> {
        self.quaternion_frames.clone().unwrap()
    }

    fn get_euler_frames(&self) -> Vec<Matrix4<f32>> {
        self.euler_frames.clone().unwrap()
    }

    fn make_step(&mut self, _time_elapsed: f64) {
        if self.euler_frames.is_some() {
            return;
        }

        let (begin_quaternion, begin_euler, end_quaternion, end_euler) =
            AnimationAngle::get_normalized_angles(&self.begin_angle, &self.end_angle);

        self.quaternion_frames = Some(
            (0..self.frames_count)
                .map(|f| {
                    let x = f as f32 / (self.frames_count - 1) as f32;
                    let t = (1f32 - x) * self.begin_position + x * self.end_position;
                    let r = get_quaternions_interpolation(
                        &begin_quaternion,
                        &end_quaternion,
                        x,
                        &self.quaternion_interpolation_type,
                    );
                    Matrix4::new_translation(&t) * r.to_rotation_matrix().to_homogeneous()
                })
                .collect(),
        );

        self.euler_frames = Some(
            (0..self.frames_count)
                .map(|f| {
                    let x = f as f32 / (self.frames_count - 1) as f32;
                    let t = (1f32 - x) * self.begin_position + x * self.end_position;
                    let r = (1f32 - x) * begin_euler + x * end_euler;
                    Matrix4::new_translation(&t)
                        * Rotation3::from_euler_angles(r.x, r.y, r.z).to_homogeneous()
                })
                .collect(),
        );
    }
}

impl Animation for ContinuousAnimation {
    fn get_quaternion_frames(&self) -> Vec<Matrix4<f32>> {
        let (begin_quaternion, _, end_quaternion, _) =
            AnimationAngle::get_normalized_angles(&self.begin_angle, &self.end_angle);

        let x = (self.time_elapsed / self.animation_time) as f32;
        let t = (1f32 - x) * self.begin_position + x * self.end_position;
        let r = get_quaternions_interpolation(
            &begin_quaternion,
            &end_quaternion,
            x,
            &self.quaternion_interpolation_type,
        );
        vec![Matrix4::new_translation(&t) * r.to_rotation_matrix().to_homogeneous()]
    }

    fn get_euler_frames(&self) -> Vec<Matrix4<f32>> {
        let (_, begin_euler, _, end_euler) =
            AnimationAngle::get_normalized_angles(&self.begin_angle, &self.end_angle);

        let x = (self.time_elapsed / self.animation_time) as f32;
        let t = (1f32 - x) * self.begin_position + x * self.end_position;
        let r = (1f32 - x) * begin_euler + x * end_euler;
        vec![
            Matrix4::new_translation(&t)
                * Rotation3::from_euler_angles(r.x, r.y, r.z).to_homogeneous(),
        ]
    }

    fn make_step(&mut self, time_elapsed: f64) {
        self.time_elapsed += time_elapsed;

        if self.time_elapsed >= self.animation_time {
            self.time_elapsed = self.animation_time;
        }
    }
}

impl DiscreteFrameAnimationBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(fc) = self.frames_count {
            if fc >= 2 {
                return Ok(());
            } else {
                return Result::Err("Frames count too low".to_string());
            }
        } else {
            return Ok(());
        }
    }
}

impl AnimationAngle {
    fn deconstruct(&self) -> (UnitQuaternion<f32>, Vector3<f32>) {
        let mut result = match self {
            AnimationAngle::Quternion(quaternion) => {
                let q = UnitQuaternion::from_quaternion(*quaternion);
                let e = q.euler_angles();
                (q, Vector3::new(e.0, e.1, e.2))
            }
            AnimationAngle::Euler(euler) => {
                let e = Vector3::new(
                    Self::normalize_angle(euler.x),
                    Self::normalize_angle(euler.y),
                    Self::normalize_angle(euler.z),
                );
                (UnitQuaternion::from_euler_angles(e.x, e.y, e.z), e)
            }
        };

        if result.0.norm_squared() < 1e-6 {
            result.0 = UnitQuaternion::from_quaternion(Quaternion::new(1f32, 0f32, 0f32, 0f32));
        }

        result.1 = Vector3::new(
            Self::normalize_angle(result.1.x),
            Self::normalize_angle(result.1.y),
            Self::normalize_angle(result.1.z),
        );

        result
    }

    fn get_normalized_angles(
        begin: &AnimationAngle,
        end: &AnimationAngle,
    ) -> (
        UnitQuaternion<f32>,
        Vector3<f32>,
        UnitQuaternion<f32>,
        Vector3<f32>,
    ) {
        let (begin_quaternion, begin_euler) = begin.deconstruct();
        let (end_quaternion, end_euler) = end.deconstruct();

        let (begin_euler, end_euler) = {
            let mut begin = begin_euler;
            let mut end = end_euler;
            (begin.x, end.x) = AnimationAngle::angles_shortest_path(begin_euler.x, end_euler.x);
            (begin.y, end.y) = AnimationAngle::angles_shortest_path(begin_euler.y, end_euler.y);
            (begin.z, end.z) = AnimationAngle::angles_shortest_path(begin_euler.z, end_euler.z);

            (begin, end)
        };

        let (begin_quaternion, end_quaternion) = if let AnimationAngle::Euler(_) = begin {
            (
                UnitQuaternion::from_euler_angles(begin_euler.x, begin_euler.y, begin_euler.z),
                UnitQuaternion::from_euler_angles(end_euler.x, end_euler.y, end_euler.z),
            )
        } else {
            let qs = [
                UnitQuaternion::from_euler_angles(0f32, 0f32, 0f32),
                UnitQuaternion::from_euler_angles(2f32 * PI, 0f32, 0f32),
                UnitQuaternion::from_euler_angles(-2f32 * PI, 0f32, 0f32),
                UnitQuaternion::from_euler_angles(0f32, 2f32 * PI, 0f32),
                UnitQuaternion::from_euler_angles(0f32, -2f32 * PI, 0f32),
                UnitQuaternion::from_euler_angles(0f32, 0f32, 2f32 * PI),
                UnitQuaternion::from_euler_angles(0f32, 0f32, -2f32 * PI),
            ];
            qs.iter()
                .flat_map(|f| qs.iter().map(move |g| f * g))
                .flat_map(|f| qs.iter().map(move |g| f * g))
                .flat_map(|a| {
                    [
                        (a * begin_quaternion, end_quaternion),
                        (begin_quaternion, a * end_quaternion),
                    ]
                })
                .reduce(|a, b| {
                    if (a.0.into_inner() - a.1.into_inner()).norm_squared()
                        < (b.0.into_inner() - b.1.into_inner()).norm_squared()
                    {
                        a
                    } else {
                        b
                    }
                })
                .unwrap()
        };

        (begin_quaternion, begin_euler, end_quaternion, end_euler)
    }

    fn normalize_angle(angle: f32) -> f32 {
        let angle = normalized_angle(angle);
        if angle >= 0f32 {
            angle
        } else {
            angle + 2f32 * PI
        }
    }

    fn angles_shortest_path(begin: f32, end: f32) -> (f32, f32) {
        let mut begin = begin;
        let mut end = end;
        if end - begin > PI {
            if begin < end {
                end -= 2.0 * PI;
            } else {
                end += 2.0 * PI;
            }
        } else if end - begin < -PI {
            if begin < end {
                begin += 2.0 * PI;
            } else {
                begin -= 2.0 * PI;
            }
        }
        (begin, end)
    }
}

fn get_quaternions_interpolation(
    begin: &UnitQuaternion<f32>,
    end: &UnitQuaternion<f32>,
    t: f32,
    interpolation_type: &QuaternionInterpolationType,
) -> UnitQuaternion<f32> {
    let r = match interpolation_type {
        QuaternionInterpolationType::Linear => {
            (1f32 - t) * begin.quaternion() + t * end.quaternion()
        }
        QuaternionInterpolationType::Spherical => {
            let cos = begin.dot(&end).clamp(-1f32, 1f32);
            let theta = cos.acos();
            let theta_sin = theta.sin();
            let (s1, s2) = if theta_sin == 0.0 {
                (1f32 - t, t)
            } else {
                (
                    ((1f32 - t) * theta).sin() / theta_sin,
                    (t * theta).sin() / theta_sin,
                )
            };
            s1 * begin.into_inner() + s2 * end.into_inner()
        }
    };
    UnitQuaternion::from_quaternion(r)
}
