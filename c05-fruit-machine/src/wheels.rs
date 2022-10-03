use std::{rc::Rc, cell::RefCell, f64::consts::PI};

use engine::{Window, GameObjectCallback, SceneCallback, Scene, Transform, GameObject, Mesh, shaders::{Texture, TextureOnly2D, Unshaded2D, Unshaded3D}, KeyCode, WindowCallback};

extern crate rand;
use rand::Rng;

use super::numbers::*;

///Sets how long the wheels spin at full speed before decelerating
const SPIN_TIME: f64 = 4.0;
///Sets how fast the wheels spin during scroll
const WHEEL_SPEEDS_SCROLL: [f64; 3] = [1.0, 1.2, 1.4];
///Sets how far the wheels are apart horizontally
const WHEEL_SPACING_X: f64 = 2.25;
///Sets how far the wheels are apart vertically
const WHEEL_SPACING_Y: f64 = 2.1;
///Sets how fast the wheels spin at the start of a spin
const WHEEL_SPEEDS_SPIN: [f64; 3] = [35.0, 35.0, 35.0];
///Sets how fast the wheels decelerate
const WHEEL_DECEL_SPIN: [f64; 3] = [3.0, 2.5, 2.0];
///The range of possible speeds wheels can stop spinning at
const LOWER_SPEED_RANGE: [f64; 2] = [2.5, 4.0];

///Matches textures to fruits
const FRUITS: [Fruit; 6] = [
    Fruit::Bell,
    Fruit::Cherry,
    Fruit::Lemon,
    Fruit::Orange,
    Fruit::Skull,
    Fruit::Star
];

///Enum for all the possible fruits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fruit {
    Lemon,
    Orange,
    Cherry,
    Bell,
    Star,
    Skull
}

///Enum for states that the game can be in
#[derive(Debug, Clone, Copy)]
pub enum Screen {
    ///An idle state when the wheels are slowly spinning
    Scroll,
    ///A state active for one frame to instruct wheels to set their spin velocity
    SpinStart,
    ///A state 
    ///f64 is how long to keep spinning in seconds
    Spin(f64),
    ///A state active while wheels are decelerating
    ///stores the fruits that have been chosen
    Decel([Option<Fruit>; 3]),
    ///A state active when wheels are stopped before returning to scroll
    //stores how long to wait for before scrolling again
    Wait(f64),
    ///A state to signal to the QuitWatcher that the window should be closed
    Loss
}

///Struct for the game's shared state
#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub balance: u32,
    pub screen: Screen,
}

///Enum for the states any one wheel can be in
#[derive(Debug)]
pub enum WheelState {
    ///State active most of the time
    ///f64 is velocity to stop at when decelerating
    Going(f64),
    ///State active when wheel is stopping at the next fruit
    ///usize is index of fruit to stop on
    Stopping(usize),
    ///State active when the wheel is stopped after a spin
    Stopped,
}

///GameObject struct for a wheel
#[derive(Debug)]
pub struct WheelObject {
    ///The wheel's current y velocity
    pub velocity: f64,
    ///The wheel's current state
    pub wheel_state: WheelState,
    ///Which wheel this instance is (0 = left, 1 = middle, 2 = right)
    pub wheel_number: usize,
    ///Shared game state
    pub game_state: Rc<RefCell<GameState>>,
    ///Textures (only used during initialisation)
    pub fruit_textures: [Texture; 6],
}

impl GameObjectCallback for WheelObject {
    fn on_tick(&mut self, object: &mut engine::GameObject, frame: &engine::Frame) {

        match self.wheel_state{
            WheelState::Going(s) => {
                match self.game_state.borrow().screen {
                    Screen::SpinStart => {
                        self.velocity = WHEEL_SPEEDS_SPIN[self.wheel_number];
                    },
                    Screen::Decel(_) => {
                        self.velocity -= WHEEL_DECEL_SPIN[self.wheel_number] * frame.time.delta_time.as_secs_f64();
                        //if moving slower than target speed, find next fruit and set state to stop there
                        if self.velocity < s {
                            let mut next_fruit_idx = usize::MAX;
                            let mut next_fruit_value = 100.0;
                            //find lowest y value greater than 0
                            for (i, mesh) in object.meshes.iter().enumerate() {
                                let y = mesh.0.get_pos().1;
                                if y > 0.0 && y < next_fruit_value {
                                    next_fruit_idx = i;
                                    next_fruit_value = y;
                                }
                            }

                            self.wheel_state = WheelState::Stopping(next_fruit_idx);
                        }
                    },
                    
                    _ => ()
                }
                //move all meshes down by self.velocity and reset to top if far enough down
                for mesh in &mut object.meshes {
                    mesh.0 = mesh.0.clone() * Transform::from_pos(0.0,-self.velocity * frame.time.delta_time.as_secs_f64(), 0.0);
                    if mesh.0.get_pos().1 < -WHEEL_SPACING_Y * 3.0 {
                        mesh.0 = mesh.0.clone() * Transform::from_pos(0.0, WHEEL_SPACING_Y * 6.0, 0.0)
                    }
                }
            },
            WheelState::Stopping(m) => {
                //move all meshes down and wrap
                for (i, mesh) in &mut object.meshes.iter_mut().enumerate() {
                    mesh.0 = mesh.0.clone() * Transform::from_pos(0.0,-self.velocity * frame.time.delta_time.as_secs_f64(), 0.0);
                    let y = mesh.0.get_pos().1;
                    if y < -WHEEL_SPACING_Y * 3.0 {
                        mesh.0 = mesh.0.clone() * Transform::from_pos(0.0, WHEEL_SPACING_Y * 6.0, 0.0)
                    }
                    //check if target fruit has reached centre, and if so save it in shared state
                    if i == m && y < 0.0 {
                        self.wheel_state = WheelState::Stopped;
                        mesh.0 = mesh.0.clone() * Transform::from_pos(0.0, -y, 0.0);
                        let gs = &mut *self.game_state.borrow_mut();
                        if let Screen::Decel(mut t) = gs.screen {
                            t[self.wheel_number] = Some(FRUITS[i]);
                            gs.screen = Screen::Decel(t);
                        }
                    }
                }
            },
            WheelState::Stopped => {
                //restart wheel if game_state is Scroll
                if let Screen::Scroll = self.game_state.borrow().screen {
                    let mut rng = rand::thread_rng();
                    self.wheel_state = WheelState::Going(rng.gen_range(LOWER_SPEED_RANGE[0]..LOWER_SPEED_RANGE[1]));
                }
            }
        }
    }
    fn on_load(&mut self, object: &mut GameObject, _scene: &mut Scene) {
        //start with a random fruit
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(0..5);
        //add each fruit as a plane with a texture
        for (i, texture) in self.fruit_textures.iter().enumerate() {
            object.meshes.push((Transform::from_pos(0.0, ((i + offset) % 6) as f64 * WHEEL_SPACING_Y - 8.0, 0.0), Mesh::plane(true, Box::new(TextureOnly2D::new(texture.clone())))));
        }
    }
}

///GameObject struct for the lever at the side of the screen
#[derive(Debug)]
pub struct LeverObject {
    ///Shared game state
    pub state: Rc<RefCell<GameState>>,
}

///Sets how long it takes for the lever to go down
const LEVER_DOWN_TIME: f64 = 0.5;
///Sets how long it takes for the lever to go up
const LEVER_UP_TIME: f64 = 0.5;


impl GameObjectCallback for LeverObject {
    fn on_load(&mut self, object: &mut GameObject, _scene: &mut Scene) {
        //add mesh for the grey part (back of the lever)
        object.meshes.push((
            Transform::origin(),
            Mesh::from_obj(
                "src/resources/objects/back.obj", 
                Box::new(
                    Unshaded3D::new([0.3, 0.3, 0.3, 0.0])
                )
            ).unwrap()
        ));

        //add mesh for the tan part (handle of the lever)
        object.meshes.push((
            Transform::origin(),
            Mesh::from_obj(
                "src/resources/objects/cylinder.obj", 
                Box::new(
                    Unshaded3D::new([0.8, 0.4, 0.2, 0.0])
                )
            ).unwrap()
        ));

        //add mesh for the brown part (sphere on the end of lever)
        object.meshes.push((
            Transform::origin(),
            Mesh::from_obj(
                "src/resources/objects/sphere.obj", 
                Box::new(
                    Unshaded3D::new([0.5, 0.2, 0.0, 0.0])
                )
            ).unwrap()
        ));
    }

    fn on_tick(&mut self, object: &mut GameObject, _frame: &engine::Frame) {
        if let Screen::Spin(d) = self.state.borrow().screen {
            //the final rotation of the lever
            let r;
            //how long the wheels have been spinning for in seconds
            let d = SPIN_TIME - d;

            //lever is on the way down
            if d < LEVER_DOWN_TIME {
                r = (d * PI) / LEVER_DOWN_TIME;
            }
            //lever is on the way up
            else if d < LEVER_DOWN_TIME + LEVER_UP_TIME {
                r = ((LEVER_DOWN_TIME + LEVER_UP_TIME - d) * PI) / (LEVER_UP_TIME);
            }
            //lever has gone down and up - keep still at top
            else {
                r = 0.0;
            }

            //set meshes' rotation based on calculated rotation
            let t = Transform::from_euler(r, 0.0, 0.0);
            object.meshes[1].0 = t.clone();
            object.meshes[2].0 = t.clone();
        }
    }
}


///Main Scene struct
#[derive(Debug)]
pub struct MainScene {
    ///Shared game state
    pub state: Rc<RefCell<GameState>>,
    ///Fruit textures for passing to wheel objects
    pub fruit_textures: [Texture; 6],
    ///Number textures for passing to digit objects
    pub number_textures: [Texture; 10],
}

///Helper function to try to subtract y from x
///
///returns Some(x - y) if x > y
///
///returns None otherwise 
#[inline]
fn try_subtract(x: u32, y: u32) -> Option<u32> {
    if x > y {
        Some(x - y)
    }
    else {
        None
    }
}

///Helper function to calculate the resulting balance after a spin
fn score_spin(fs: [Fruit; 3], balance: u32) -> Option<u32> {
    use Fruit::*;
    match fs.iter().filter(|f|f == &&Skull).count() {
        3 => return None,
        2 => return try_subtract(balance, 100),
        _ => ()
    };

    if fs == [Bell, Bell, Bell] {
        return Some(balance + 500);
    }

    let mut i = fs.to_vec();
    i.sort();
    i.dedup();
    match i.len() {
        1 => return Some(balance + 100),
        2 => return Some(balance + 50),
        _ => return Some(balance)
    }
}

impl SceneCallback for MainScene {
    fn on_tick(&mut self, _scene: &mut Scene, frame: &engine::Frame) {
        //stores what the shared screen state will be set to after the function
        let mut end_state = self.state.borrow().screen.clone();
        //stores what the shared balance state will be set to after the function
        let mut end_balance = self.state.borrow().balance;
        match end_state {
            Screen::Scroll => {
                //start spin on space press
                if frame.input.is_key_pressed_this_frame(KeyCode::Space) {
                    end_state = Screen::SpinStart;
                    end_balance -= 20;
                }
            },
            Screen::SpinStart => end_state = Screen::Spin(SPIN_TIME),
            Screen::Spin(d) => {
                end_state = Screen::Spin(d - frame.time.delta_time.as_secs_f64());
                if d < 0.0 {
                    end_state = Screen::Decel([None, None, None]);
                }
            },
            Screen::Decel(fs) => match fs {
                //if all three wheels are stopped, calculate new balance
                [Some(f1), Some(f2), Some(f3)] => {
                    end_state = Screen::Wait(1.5);
                    match score_spin([f1, f2, f3], end_balance) {
                        Some(b) if b >= 20 => end_balance = b,
                        _ => end_state = Screen::Loss
                    }
                },
                _ => {}
            },
            Screen::Wait(d) => {
                end_state = Screen::Wait(d - frame.time.delta_time.as_secs_f64());
                if d < 0.0 {
                    end_state = Screen::Scroll;
                }
            }
            Screen::Loss => {}
        }

        let state: &mut GameState = &mut self.state.borrow_mut();
        state.screen = end_state;
        state.balance = end_balance;
    }

    fn on_insert(&mut self, scene: &mut Scene, _window: &mut Window) {
        
        //add left wheel
        scene.add_object( 
            GameObject::new( Some( Box::new( WheelObject {
                velocity: WHEEL_SPEEDS_SCROLL[0], 
                wheel_number: 0, 
                game_state: self.state.clone(), 
                fruit_textures: self.fruit_textures.clone(), 
                wheel_state: WheelState::Stopped
            })), 
            "Wheel1".to_string(), 
            Transform::from_pos(-WHEEL_SPACING_X, 0.0, 0.0))
        );
        //add middle wheel
        scene.add_object( 
            GameObject::new( Some( Box::new( WheelObject {
                velocity: WHEEL_SPEEDS_SCROLL[1], 
                wheel_number: 1, 
                game_state: self.state.clone(), 
                fruit_textures: self.fruit_textures.clone(), 
                wheel_state: WheelState::Stopped
            })), 
            "Wheel2".to_string(), 
            Transform::from_pos(0.0, 0.0, 0.0))
        );
        //add right wheel
        scene.add_object( 
            GameObject::new( Some( Box::new( WheelObject {
                velocity: WHEEL_SPEEDS_SCROLL[2], 
                wheel_number: 2, 
                game_state: self.state.clone(), 
                fruit_textures: self.fruit_textures.clone(), 
                wheel_state: WheelState::Stopped
            })), 
            "Wheel3".to_string(), 
            Transform::from_pos(WHEEL_SPACING_X, 0.0, 0.0))
        );
        
        //add black box at top of screen
        let mut top_box = GameObject::new(
            None, 
            "TopBox".to_string(),
            Transform::from_scale(10.0, 4.0, 1.0) * Transform::from_pos(0.0, 6.0, -1.0)
        );
        top_box.meshes.push((Transform::origin(), Mesh::plane(true, Box::new(Unshaded2D::new([0.0, 0.0, 0.0, 1.0])))));
        scene.add_object(top_box);

        //add £10s digit
        scene.add_object(GameObject::new( Some( Box::new( 
            BalanceIndicatorDigit {
                state: self.state.clone(),
                balance_current: 999999999,
                digit_no: 4,
                textures: self.number_textures.clone()
            })),
        "Digit4".to_string(),
        Transform::from_scale(0.5, 0.5, 1.0) * Transform::from_pos(-2.0, 2.5, -2.0)
        ));
        //add £1s digit
        scene.add_object(GameObject::new( Some( Box::new( 
            BalanceIndicatorDigit {
                state: self.state.clone(),
                balance_current: 999999999,
                digit_no: 3,
                textures: self.number_textures.clone()
            })),
        "Digit3".to_string(),
        Transform::from_scale(0.5, 0.5, 1.0) * Transform::from_pos(-1.0, 2.5, -2.0)
        ));
        //add 10ps digit
        scene.add_object(GameObject::new( Some( Box::new( 
            BalanceIndicatorDigit {
                state: self.state.clone(),
                balance_current: 999999999,
                digit_no: 2,
                textures: self.number_textures.clone()
            })),
        "Digit2".to_string(),
        Transform::from_scale(0.5, 0.5, 1.0) * Transform::from_pos(1.0, 2.5, -2.0)
        ));
        //add 1ps digit
        scene.add_object(GameObject::new( Some( Box::new( 
            BalanceIndicatorDigit {
                state: self.state.clone(),
                balance_current: 999999999,
                digit_no: 1,
                textures: self.number_textures.clone()
            })),
            "Digit1".to_string(),
            Transform::from_scale(0.5, 0.5, 1.0) * Transform::from_pos(2.0, 2.5, -2.0)
        ));

        //add lever
        scene.add_object(GameObject::new( Some( Box::new(
            LeverObject {
                state: self.state.clone()
            })),
            "Lever".to_string(), 
            Transform::from_scale(1.0, 1.0, 1.0) * Transform::from_euler(0.0, -0.5, 0.0) * Transform::from_pos(15.0, -3.0, 20.0)
        ));
    }
}

///Struct to close window on Loss
#[derive(Debug)]
pub struct QuitWatcher {
    pub state: Rc<RefCell<GameState>>,
}
impl WindowCallback for QuitWatcher {
    fn on_tick(&mut self, window: &mut Window, _frame: &engine::Frame) {
        if let Screen::Loss = self.state.borrow().screen{
            window.close();
        }
    }
}