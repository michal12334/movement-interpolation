#[derive(Debug, Clone, Default)]
pub struct AnimationData {
    pub begin_position: (f32, f32, f32),
    pub end_position: (f32, f32, f32),
    pub begin_rotation_quaternion: (f32, f32, f32, f32),
    pub end_rotation_quaternion: (f32, f32, f32, f32),
    pub begin_rotation_xyz: (f32, f32, f32),
    pub end_rotation_xyz: (f32, f32, f32),
    pub quaternion_interpolation_type: QuaternionInterpolationType,
    pub display_all_frames: bool,
    pub animation_time: f64,
    pub frames_count: u8,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum QuaternionInterpolationType {
    #[default]
    Linear,
    Spherical,
}

impl AnimationData {
    pub fn new() -> Self {
        Self {
            begin_rotation_quaternion: (1f32, 0f32, 0f32, 0f32),
            end_rotation_quaternion: (1f32, 0f32, 0f32, 0f32),
            frames_count: 10,
            animation_time: 10.0,
            ..Default::default()
        }
    }
}
