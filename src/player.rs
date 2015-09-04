use glium::backend::Facade;
use glium::{
    Surface,
    Program,
};
use glium::texture::SrgbTexture2d;
use glium::uniforms::{
    Uniforms,
    Sampler,
    SamplerWrapFunction,
    MinifySamplerFilter,
    MagnifySamplerFilter,
};
use gel::{
    Mat4,
    Vec3,
    Normalize,
    Cross,
};

use config::PlayerParams;
use cube::Cube;
use errors::Result;

pub struct Player {
    object: Cube,
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    speed: f32,
    diffuse_map: SrgbTexture2d,
    specular_map: SrgbTexture2d,
    shine: f32,
}

impl Player {
    pub fn new<F>(facade: &F,
                  diffuse: SrgbTexture2d,
                  specular: SrgbTexture2d,
                  shine: f32,
                  params: &PlayerParams) -> Result<Player>
        where F: Facade
    {
        let mut cube: Cube = try!(Cube::new(facade));
        cube.set_scale(params.scale());
        Ok(Player {
            object: cube,
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            speed: params.speed(),
            diffuse_map: diffuse,
            specular_map: specular,
            shine: shine,
        })
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_speed(&mut self, s: f32) {
        self.speed = s;
    }

    pub fn forward(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position + (self.direction * velocity);
        self.object.set_position(self.position);
    }

    pub fn backward(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position - (self.direction * velocity);
        self.object.set_position(self.position);
    }

    pub fn right(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position + (self.direction.cross(&self.up).normalize() * velocity);
        self.object.set_position(self.position);
    }

    pub fn left(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position - (self.direction.cross(&self.up).normalize() * velocity);
        self.object.set_position(self.position);
    }

    pub fn up(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position + (self.up * velocity);
        self.object.set_position(self.position);
    }

    pub fn down(&mut self, dt: f32) {
        let velocity = self.speed * dt;
        self.position = self.position - (self.up * velocity);
        self.object.set_position(self.position);
    }

    pub fn model(&self) -> Mat4 {
        self.object.model()
    }

    pub fn diffuse_map(&self) -> Sampler<SrgbTexture2d> {
        self.diffuse_map.sampled()
            .wrap_function(SamplerWrapFunction::Mirror)
            .minify_filter(MinifySamplerFilter::LinearMipmapLinear)
            .magnify_filter(MagnifySamplerFilter::Linear)
    }

    pub fn specular_map(&self) -> Sampler<SrgbTexture2d> {
        self.specular_map.sampled()
            .wrap_function(SamplerWrapFunction::Mirror)
            .minify_filter(MinifySamplerFilter::LinearMipmapLinear)
            .magnify_filter(MagnifySamplerFilter::Linear)
    }

    pub fn shine(&self) -> f32 {
        self.shine
    }

    pub fn draw<S, U>(&self,
                      surface: &mut S,
                      program: &Program,
                      uniforms: &U) -> Result<()>
        where S: Surface,
              U: Uniforms,
    {
        self.object.draw(surface, program, uniforms)
    }
}
