use gel::{
    self,
    Mat4,
    Vec3,
    LookAt,
    Cross,
    Normalize,
};
use num::Zero;

use player::Player;
use config::CameraParams;

pub struct Camera {
    up: Vec3,
    radius: f32,
    elev: f32,
    azimuth: f32,
    sensitivity: f32,
}

impl Camera {
    // Target is relative to position
    pub fn new(params: &CameraParams) -> Camera {
        Camera {
            up: Vec3::new(0.0, 1.0, 0.0),
            radius: params.radius(),
            elev: 0.0,
            azimuth: 90.0,
            sensitivity: params.sensitivity(),
        }
    }

    pub fn set_sensitivity(&mut self, s: f32) {
        self.sensitivity = s;
    }

    pub fn update(&mut self, deltax: i32, deltay: i32) {
        let (mut deltax, mut deltay) = (deltax as f32, deltay as f32);

        deltax *= self.sensitivity;
        deltay *= self.sensitivity;

        self.azimuth += deltax;
        self.elev += -deltay;
    }

    pub fn look_at(&self, pos: Vec3) -> Mat4 {
        let elevation = gel::radians(self.elev);
        let azimuth = gel::radians(self.azimuth);
        let mut position = pos;
        position.x += self.radius * elevation.cos() * azimuth.cos();
        position.y += self.radius * elevation.sin();
        position.z += self.radius * elevation.cos() * azimuth.sin();
        Mat4::look_at(position, pos, self.up)
    }
}

