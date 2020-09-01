use iced_graphics::{
    triangle::Vertex2D, Backend, Defaults, Primitive, Renderer,
};

use iced_native::{
    layout, mouse, Element, Hasher, Layout, Length, Point, Size, Vector, Widget,
};

/// A backend-agnostic renderer that supports all built in widgets necessary for point cloud viewing
pub struct CloudRenderer<B: Backend> {
    backend: B,
}

impl <B: Backend> CloudRenderer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }


}
