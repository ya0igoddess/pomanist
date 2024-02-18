use std::rc::Rc;


use crate::{gl_wrapper::{array_object::ArrayObject, buffer_object::{Buffer, BufferType}, texture_object::Texture}, util::{pod_pool::PodPool, vec3::Vec3}};
use super::renderable::Renderable;

struct StaticGroup {
    vao: ArrayObject,
    bao: Buffer,
    indicies: Buffer,
    texture: Rc<Texture>,
    enteties_buffer: PodPool<Vec3>
}

impl StaticGroup {
   pub fn new(texture: Rc<Texture>) -> Self {
        let vao = ArrayObject::new().unwrap();
        vao.bind();
        let bao = Buffer::new().unwrap();
        bao.bind(BufferType::Array);
        StaticGroup { vao: vao, bao: bao, indicies: Buffer::new().unwrap(), texture: texture, enteties_buffer: PodPool::default() }
   }
}

impl Renderable for StaticGroup {
    fn render_with_camera(&self, camera: super::camera::Camera) {
        
    }
}