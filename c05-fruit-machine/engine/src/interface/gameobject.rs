use std::fmt::Debug;

use crate::Transform;
use crate::Scene;
use crate::Mesh;
use crate::Frame;

pub trait GameObjectCallback : Debug {
    /// Called after the GameObject is added to a Scene
    fn on_load(&mut self, _object: &mut GameObject, _scene: &mut Scene) {}
    /// Called before the GameObject is removed from a Scene
    fn on_unload(&mut self, _object: &mut GameObject, _scene: &mut Scene) {}
    /// Called every frame
    fn on_tick(&mut self, _object: &mut GameObject, _frame: &Frame) {}
    /// Called before the GameObject is destroyed
    /// Allows the GameObject to clean up state e.g. file handles
    fn on_destroy(&mut self, _object: &mut GameObject) {}
}

#[derive(Debug)]
pub struct GameObjectCallbackDefault {}
impl GameObjectCallback for GameObjectCallbackDefault {}

/// Struct representing an object in a scene
#[derive(Debug)]
pub struct GameObject {
    pub(crate) callbacks: Option<Box<dyn GameObjectCallback>>,
    pub name: String,
    pub transform: Transform,
    pub meshes: Vec<(Transform, Mesh)>,
}

impl GameObject {
    pub fn new (callbacks: Option<Box<dyn GameObjectCallback>>, name: String, transform: Transform) -> Self {
        let object = GameObject {
            callbacks: callbacks,
            name: name,
            transform: transform,

            meshes: Vec::new()
        };
        object
    }
}
