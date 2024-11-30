use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use nalgebra::{Matrix4, Quaternion, Rotation3, UnitQuaternion, Vector3};

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

        let (begin_quaternion, begin_euler) = self.begin_angle.deconstruct();
        let (end_quaternion, end_euler) = self.end_angle.deconstruct();

        self.quaternion_frames = Some(
            (0..self.frames_count)
                .map(|f| {
                    let x = f as f32 / (self.frames_count - 1) as f32;
                    let t = (1f32 - x) * self.begin_position + x * self.end_position;
                    let r = (1f32 - x) * begin_quaternion.quaternion()
                        + x * end_quaternion.quaternion();
                    Matrix4::new_translation(&t)
                        * UnitQuaternion::from_quaternion(r)
                            .to_rotation_matrix()
                            .to_homogeneous()
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
        let (begin_quaternion, _) = self.begin_angle.deconstruct();
        let (end_quaternion, _) = self.end_angle.deconstruct();

        let x = (self.time_elapsed / self.animation_time) as f32;
        let t = (1f32 - x) * self.begin_position + x * self.end_position;
        let r = (1f32 - x) * begin_quaternion.quaternion() + x * end_quaternion.quaternion();
        vec![
            Matrix4::new_translation(&t)
                * UnitQuaternion::from_quaternion(r)
                    .to_rotation_matrix()
                    .to_homogeneous(),
        ]
    }

    fn get_euler_frames(&self) -> Vec<Matrix4<f32>> {
        let (_, begin_euler) = self.begin_angle.deconstruct();
        let (_, end_euler) = self.end_angle.deconstruct();

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
        match self {
            AnimationAngle::Quternion(quaternion) => {
                let q = UnitQuaternion::from_quaternion(*quaternion);
                let e = q.euler_angles();
                (q, Vector3::new(e.0, e.1, e.2))
            }
            AnimationAngle::Euler(euler) => (
                UnitQuaternion::from_euler_angles(euler.x, euler.y, euler.z),
                *euler,
            ),
        }
    }
}
