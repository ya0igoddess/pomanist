use bytemuck::{Pod, Zeroable};

#[derive(Pod, Clone, Copy, Zeroable, Default)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}