#[derive(Debug, Clone, Default)]
pub struct AnimationData {
    pub begin_position: (f32, f32, f32),
    pub end_position: (f32, f32, f32),
    pub begin_rotation_quaternion: (f32, f32, f32, f32),
    pub end_rotation_quaternion: (f32, f32, f32, f32),
    pub begin_rotation_xyz: (f32, f32, f32),
    pub end_rotation_xyz: (f32, f32, f32),
}
