use gel::{
    self,
    Mat4,
    Vec3,
    LookAt,
    Cross,
    Normalize,
};
use num::Zero;

#[derive(Debug, Clone)]
pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}

impl Camera {
    // Target is relative to position
    pub fn new(pos: Vec3, target: Vec3) -> Camera {
        Camera {
            position: pos,
            target: target,
            up: Vec3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 1.0,
            sensitivity: 1.0,
        }
    }

    pub fn set_sensitivity(&mut self, s: f32) -> &mut Camera {
        self.sensitivity = s;
        self
    }

    pub fn set_speed(&mut self, s: f32) -> &mut Camera {
        self.speed = s;
        self
    }

    pub fn forward(&mut self, dt: f32) {
        self.position = self.position + (self.target * (self.speed * dt));
    }

    pub fn backward(&mut self, dt: f32) {
        self.position = self.position - (self.target * (self.speed * dt));
    }

    pub fn left(&mut self, dt: f32) {
        self.position = self.position - (self.target.cross(&self.up).normalize() * (self.speed * dt));
    }

    pub fn right(&mut self, dt: f32) {
        self.position = self.position + (self.target.cross(&self.up).normalize() * (self.speed * dt));
    }

    pub fn up(&mut self, dt: f32) {
        self.position = self.position + (self.up * (self.speed * dt));
    }

    pub fn down(&mut self, dt: f32) {
        self.position = self.position + (-self.up * (self.speed * dt));
    }

    pub fn update_target(&mut self, deltax: i32, deltay: i32) {
        let (mut deltax, mut deltay) = (deltax as f32, deltay as f32);

        deltax *= self.sensitivity;
        deltay *= self.sensitivity;

        self.yaw += deltax;
        self.pitch += deltay;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let mut target = Vec3::zero();
        target.x = gel::radians(self.yaw).cos() * gel::radians(self.pitch).cos();
        target.y = gel::radians(self.pitch);
        target.z = gel::radians(self.yaw).sin() * gel::radians(self.pitch).cos();
        self.target = target.normalize();
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.target, self.up)
    }
}


