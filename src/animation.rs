use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use nalgebra::{Matrix4, Vector3};

pub trait Animation {
    fn get_frames(&self) -> Vec<Matrix4<f32>>;
    fn make_step(&mut self);
}

#[derive(Debug, Clone, Getters, new, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct DiscreteFrameAnimation {
    begin_position: Vector3<f32>,
    end_position: Vector3<f32>,
    frames_count: u8,

    #[builder(setter(skip))]
    frames: Option<Vec<Matrix4<f32>>>,
}

pub struct ContinuousAnimation {}

impl Animation for DiscreteFrameAnimation {
    fn get_frames(&self) -> Vec<Matrix4<f32>> {
        self.frames.clone().unwrap()
    }

    fn make_step(&mut self) {
        if self.frames.is_some() {
            return;
        }

        self.frames = Some(
            (0..self.frames_count)
                .map(|f| {
                    let x = f as f32 / (self.frames_count - 1) as f32;
                    let t = (1f32 - x) * self.begin_position + x * self.end_position;
                    Matrix4::new_translation(&t)
                })
                .collect(),
        );
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
