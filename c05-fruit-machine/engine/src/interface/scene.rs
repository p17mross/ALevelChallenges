use std::fmt::Debug;
use std::mem::take;
use std::mem::replace;

use glium::Frame;

use crate::Renderable;
use crate::Camera;
use crate::Transform;
use crate::Window;
use crate::GameObject;

pub trait SceneCallback: Debug {
    fn on_load(&mut self, _scene: &mut Scene){}
    fn on_unload(&mut self, _scene: &mut Scene){}

    fn on_insert(&mut self, _scene: &mut Scene, _window: &mut Window){}
    fn on_remove(&mut self, _scene: &mut Scene, _window: &mut Window){}

    fn on_tick(&mut self, _scene: &mut Scene, _frame: &crate::Frame) {}
}

#[derive(Debug)]
pub struct SceneCallbackDefault {}
impl SceneCallback for SceneCallbackDefault {}

#[derive(Debug)]
pub struct Scene {
    callbacks: Option<Box<dyn SceneCallback>>,
    pub main_camera: Renderable,

    pub(crate) objects: Vec<GameObject>,
}

impl Scene {
    pub(crate) fn remove (mut self, window: &mut Window) {
        if let Some(mut callbacks) = take(&mut self.callbacks) {
            callbacks.on_remove(&mut self, window);
            self.callbacks = Some(callbacks);
        }

        //TODO: free resources, call unload callback on gameobjects
        for object in &mut self.objects {
            if let Some(mut callbacks) = take(&mut object.callbacks) {
                callbacks.on_destroy(object);
                object.callbacks = Some(callbacks);
            }
        }
    }

    ///Insert the scene into a window
    pub(crate) fn insert (&mut self, window: &mut Window) {
        if let Some(mut callbacks) = take(&mut self.callbacks) {
            callbacks.on_insert(self, window);
            self.callbacks = Some(callbacks);
        }
    }

    ///Create a new scene
    pub fn new(mut callbacks: Box<dyn SceneCallback>, camera: Renderable) -> Self {
        let mut scene = Scene {
            callbacks: None,
            objects: Vec::new(),
            main_camera: camera,
        };
        callbacks.on_load(&mut scene);
        scene.callbacks = Some(callbacks);
        scene
    }

    pub(crate) fn render(&mut self, frame: &mut Frame, window: &Window) {
        let mut main_camera = replace(&mut self.main_camera, Renderable::Camera(Camera::new(Transform::origin(), 3.0)));
        main_camera.render(frame, self, window, -1.0, 1.0, -1.0, 1.0);
        self.main_camera = main_camera;
    }

    pub(crate) fn tick(&mut self, window: &Window) {

        if let Some(mut callbacks) = take(&mut self.callbacks) {
            callbacks.on_tick(self, &window.frame);
            self.callbacks = Some(callbacks);
        }

        for mut object in &mut self.objects {
            if let Some(mut callbacks) = take(&mut object.callbacks) {
                callbacks.on_tick(object, &window.frame);
                object.callbacks = Some(callbacks);
            }
        }
    }

    pub fn add_object(&mut self, mut object: GameObject) {
        if let Some(mut callbacks) = take(&mut object.callbacks) {
            callbacks.on_load(&mut object, self);
            object.callbacks = Some(callbacks);
        }
        self.objects.push(object);
    }

    pub fn alter_object_by_name(&mut self, name: String, closure: Box<dyn FnOnce(&mut GameObject)>) -> Option<()> {
        for mut object in &mut self.objects {
            if object.name == name {
                closure(&mut object);
                return Some(());
            }
        }
        None
    }

}

impl Drop for Scene {
    fn drop(&mut self) {
        if let Some(mut callbacks) = take(&mut self.callbacks) {
            callbacks.on_unload(self);
            self.callbacks = Some(callbacks);
        }

        for object in &mut self.objects {
            if let Some(mut callbacks) = take(&mut object.callbacks) {
                callbacks.on_destroy(object);
                object.callbacks = Some(callbacks);
            }
        }
    }
}