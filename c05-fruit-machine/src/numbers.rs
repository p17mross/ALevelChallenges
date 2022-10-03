use std::{cell::RefCell, rc::Rc};

use engine::{shaders::{Texture, TextureOnly2D}, GameObjectCallback, GameObject, Scene, Transform, Mesh};

use crate::wheels::GameState;

///GameObject struct for a balance digit
#[derive(Debug)]
pub struct BalanceIndicatorDigit {
    ///Shared game state
    pub state: Rc<RefCell<GameState>>,
    ///Currently shown balance
    pub balance_current: u32,
    ///Which digit this instance is
    pub digit_no: u32,
    ///Number textures
    pub textures: [Texture; 10]
}

impl GameObjectCallback for BalanceIndicatorDigit {
    fn on_tick(&mut self, object: &mut GameObject, _frame: &engine::Frame) {
        let balance = self.state.borrow().balance;
        //only change if balance has changed
        if balance != self.balance_current {
            let digit = balance % ((10 as u32).pow(self.digit_no)) / (10 as u32).pow(self.digit_no - 1);
            let prev_digit = self.balance_current % ((10 as u32).pow(self.digit_no)) / (10 as u32).pow(self.digit_no - 1);
            //only change if this digit has changed
            if digit != prev_digit {
                object.meshes[digit as usize].0 = Transform::from_pos(0.0, 0.0, 0.0);
                object.meshes[prev_digit as usize].0 = Transform::from_pos(0.0, 10.0, 0.0);
            }
            self.balance_current = balance;
        }
    }
    fn on_load(&mut self, object: &mut GameObject, _scene: &mut Scene) {
        for texture in self.textures.iter() {
            object.meshes.push((Transform::from_pos(0.0, 10.0, 0.0), Mesh::plane(true, Box::new(TextureOnly2D::new(texture.clone())))));
        }
    }
}