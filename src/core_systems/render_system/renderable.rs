use super::camera::Camera;

pub trait Renderable {
    fn render_with_camera(&self, camera: Camera);
}