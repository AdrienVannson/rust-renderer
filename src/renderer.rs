use crate::Scene;

pub trait Renderer {
    /// Renders the scene
    fn render(&self, scene: Scene) -> Vec<Vec<(u8, u8, u8)>>;
}
