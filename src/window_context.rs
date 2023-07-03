use std::vec;

use glium::glutin::event::{VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta};
use glium::glutin::dpi::PhysicalPosition;

pub struct Keyboard {
    keys_held: Vec<Option<VirtualKeyCode>>,
    keys_pressed: Vec<Option<VirtualKeyCode>>,
    keys_released: Vec<Option<VirtualKeyCode>>,
}

impl Keyboard {
    // Initializer
    pub fn new() -> Self {
        Self { keys_held: vec![], keys_pressed: vec![], keys_released: vec![] }
    }
    pub fn process_input(&mut self, input: glium::glutin::event::KeyboardInput) {
        if input.virtual_keycode.is_none() { // Eh? Dunno if this should ever happen but it must be an option for a reason..
            return;
        }
        match input.state {
            ElementState::Pressed => {
                if !self.keys_held.contains(&input.virtual_keycode) {
                    self.keys_held.push(input.virtual_keycode);
                    self.keys_pressed.push(input.virtual_keycode);
                }
            },
            ElementState::Released => {
                self.keys_held.retain(|&x| x != input.virtual_keycode);
                self.keys_released.push(input.virtual_keycode);
            },
        }
    }

    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        self.keys_held.contains(&Some(key))
    }
    pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&Some(key))
    }
    pub fn key_released(&self, key: VirtualKeyCode) -> bool {
        self.keys_released.contains(&Some(key))
    }

    // Clears keys_pressed and released to reset them - meant to be called at the end of every frame
    pub fn clear(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }
}

pub struct Mouse {
    pub buttons_held: Vec<MouseButton>,
    pub buttons_pressed: Vec<MouseButton>,
    pub buttons_released: Vec<MouseButton>,
    pub pos: PhysicalPosition<f64>,
    pub scroll_delta: MouseScrollDelta,
}

impl Mouse {
    pub fn new() -> Self {
        Self { buttons_held: vec![], buttons_pressed: vec![], buttons_released: vec![], pos: PhysicalPosition { x: 0., y: 0. }, scroll_delta: MouseScrollDelta::LineDelta(0., 0.) }
    }
    pub fn process_input(&mut self, state: ElementState, button: MouseButton) {
        // TODO: Process input and add functions to check state also clean things up
        if matches!(button, MouseButton::Other {..}) { // Don't wanna handle other mouse buttons
            return;
        }
        match state {
            ElementState::Pressed => {
                if !self.buttons_held.contains(&button) {
                    self.buttons_held.push(button);
                    self.buttons_pressed.push(button);
                }
            },
            ElementState::Released => {
                self.buttons_held.retain(|&x| x != button);
                self.buttons_released.push(button);
            },
        }
    }

    pub fn button_held(&self, button: MouseButton) -> bool {
        self.buttons_held.contains(&button)
    }
    pub fn button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }
    pub fn button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }

    pub fn get_pos(&self) -> &PhysicalPosition<f64> {
        &self.pos
    }
    pub fn get_scroll_delta(&self) -> &MouseScrollDelta {
        &self.scroll_delta
    }

    pub fn set_pos(&mut self, p: PhysicalPosition<f64>) {
        self.pos = p;
    }
    pub fn set_scroll_delta(&mut self, d: MouseScrollDelta) {
        self.scroll_delta = d;
    }
    // Clears buttons_pressed and released to reset them - meant to be called at the end of every frame
    pub fn clear(&mut self) {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.scroll_delta = MouseScrollDelta::LineDelta(0., 0.);
    }
}