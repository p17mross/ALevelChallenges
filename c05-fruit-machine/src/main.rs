
extern crate engine;

mod wheels;
mod numbers;
use std::{cell::RefCell, rc::Rc};

use engine::{Window, Resolution, Camera, Transform, shaders::Texture, Scene, Renderable};
use wheels::*;

const FRUIT_PATHS: [&str; 6] = [
    "bell.webp",
    "cherry.webp",
    "lemon.png",
    "orange.png",
    "skull.png",
    "star.webp"
];
const NUMBER_PATHS: [&str; 10] = [
    "0.png",
    "1.png",
    "2.png",
    "3.png",
    "4.png",
    "5.png",
    "6.png",
    "7.png",
    "8.png",
    "9.png",
];

fn main() {

    //Ititialise shared state
    let state = Rc::new( RefCell::new( GameState {
        balance: 100,
        screen: Screen::Scroll,
    }));

    //Create window and camera
    let mut window = Window::new(Some(Box::new(QuitWatcher{state: state.clone()})), Resolution::Physical(1920, 1080), "Fruit Machine".to_string()).unwrap();
    window.set_icon("src/resources/icon.webp".to_string()).unwrap();

    let mut main_camera = Camera::new(Transform::from_scale(3.5, 3.5, 3.5), 3.0);
    main_camera.set_clear_colour(Some([1.0, 1.0, 1.0, 0.0]));

    //load textures from files
    let fruit_textures: [Texture; 6] = FRUIT_PATHS.map(|p| {
        Texture::new("src/resources/fruits/".to_string() + p, &window).unwrap()
    });
    let number_textures: [Texture; 10] = NUMBER_PATHS.map(|p| {
        Texture::new("src/resources/numbers/".to_string() + p, &window).unwrap()
    });

    //Create scene
    window.set_scene(Scene::new(
        Box::new( MainScene {
            state: state,
            fruit_textures,
            number_textures,
        }), 
        Renderable::Camera(main_camera)
    ));

    window.main_loop().unwrap();
}
