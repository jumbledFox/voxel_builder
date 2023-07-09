use std::f32::consts::PI;

use glium::Surface;
use glium::glutin;
use glium::glutin::dpi::PhysicalPosition;
use crate::window_context;
use glam;

pub struct Camera {
    pub position: glam::Vec3,
    pub rotation: glam::Vec3,
    pub perspective_matrix: glam::Mat4,
    pub view_matrix: glam::Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self { position: glam::Vec3::new(0.0, 0.0, 0.0), rotation: glam::Vec3::new(0.0, 0.0, 0.0), perspective_matrix: glam::Mat4::IDENTITY, view_matrix: glam::Mat4::IDENTITY }
    }
    pub fn calculate_view_matrix(&mut self) -> glam::Mat4 {
        // Rotate and translate
        self.view_matrix = glam::Mat4::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, self.rotation.z);
        self.view_matrix *= glam::Mat4::from_translation(-self.position);
        self.view_matrix
    }
    pub fn calculate_perspective_matrix(&mut self, target: &glium::Frame) -> glam::Mat4 {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;

        self.perspective_matrix = glam::Mat4::perspective_lh(1.2, aspect_ratio, 0.01, 1024.0);
        self.perspective_matrix
    }
}

pub struct FlyCamera {
    pub speed: f32,
    pub default_speed: f32,
    pub fast_speed: f32,
    pub camera: Camera,
}
impl FlyCamera {
    pub fn new() -> Self {
        Self { speed: 5.0, default_speed: 5.0, fast_speed: 20.0, camera: Camera::new() }
    }
    pub fn handle_movement(&mut self, kb: &window_context::Keyboard, deltatime: &f32) {
        // Speed
        if kb.key_pressed(glutin::event::VirtualKeyCode::LControl) {
            self.speed = self.fast_speed;
        }
        if kb.key_released(glutin::event::VirtualKeyCode::LControl) {
            self.speed = self.default_speed;
        }
        // Simplified expressions to stop repetition
        let sin_rot = self.camera.rotation.y.sin() * deltatime * self.speed;
        let cos_rot = self.camera.rotation.y.cos() * deltatime * self.speed;
        // Forwards, backwards, and strafing
        if kb.key_held(glutin::event::VirtualKeyCode::W) {
            self.camera.position.x -= sin_rot;
            self.camera.position.z += cos_rot;
        }
        if kb.key_held(glutin::event::VirtualKeyCode::S) {
            self.camera.position.x += sin_rot;
            self.camera.position.z -= cos_rot;
        }
        if kb.key_held(glutin::event::VirtualKeyCode::A) {
            self.camera.position.x -= cos_rot;
            self.camera.position.z -= sin_rot;
        }
        if kb.key_held(glutin::event::VirtualKeyCode::D) {
            self.camera.position.x += cos_rot;
            self.camera.position.z += sin_rot;
        }
        // Up and down
        if kb.key_held(glutin::event::VirtualKeyCode::Space) {
            self.camera.position.y += deltatime * self.speed;
        }
        if kb.key_held(glutin::event::VirtualKeyCode::LShift) {
            self.camera.position.y -= deltatime * self.speed;
        }

    }

    pub fn handle_mouse_looking(&mut self, display: &glium::Display, pos: &PhysicalPosition<f64>) {
        let cy = pos.x - (display.gl_window().window().inner_size().width/2) as f64;
        let cx = pos.y - (display.gl_window().window().inner_size().height/2) as f64;
        self.camera.rotation.y -= cy as f32 / 100.0;
        self.camera.rotation.x -= cx as f32 / 100.0;
        self.camera.rotation.x = self.camera.rotation.x.min(PI/2.0).max(-PI/2.0);
        self.camera.rotation.y = (self.camera.rotation.y).rem_euclid(PI*2.0);
        FlyCamera::reset_mouse_pos(display);
    }

    pub fn reset_mouse_pos(display: &glium::Display) {
        let window_size = display.gl_window().window().inner_size();
        let _ = display.gl_window().window().set_cursor_position(glutin::dpi::LogicalPosition::new(window_size.width/2, window_size.height/2));
    }
}