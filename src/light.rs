use glium::backend::Facade;
use glium::vertex::{
    VertexBuffer,
};
use glium::{
    Surface,
    Program,
    DrawParameters,
    DepthTest,
};
use glium::uniforms::{
    Uniforms,
    Sampler,
    SamplerWrapFunction,
    MinifySamplerFilter,
    MagnifySamplerFilter,
};
use glium::index::{
    NoIndices,
    PrimitiveType,
};
use glium::texture::SrgbTexture2d;
use num::{
    One,
    Zero,
};
use gel::{
    Vec3,
    Mat4,
    Repeat,
    Rotate,
    Scale,
    Translate,
};

use config::LightParams;
use errors::{
    Result,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: [x, y ,z],
        }
    }
}

pub struct Light {
    vtxbuf: VertexBuffer<Vertex>,
    idxbuf: NoIndices,
    position: Vec3,
    scale: f32,
    color: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl Light {
    pub fn new<F>(facade: &F, params: &LightParams) -> Result<Light>
        where F: Facade
    {
        let ref vertices = [
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new( 1.0, -1.0, -1.0),
            Vertex::new( 1.0,  1.0, -1.0),
            Vertex::new( 1.0,  1.0, -1.0),
            Vertex::new(-1.0,  1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),

            Vertex::new(-1.0, -1.0,  1.0),
            Vertex::new( 1.0, -1.0,  1.0),
            Vertex::new( 1.0,  1.0,  1.0),
            Vertex::new( 1.0,  1.0,  1.0),
            Vertex::new(-1.0,  1.0,  1.0),
            Vertex::new(-1.0, -1.0,  1.0),

            Vertex::new(-1.0,  1.0,  1.0),
            Vertex::new(-1.0,  1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0,  1.0),
            Vertex::new(-1.0,  1.0,  1.0),

            Vertex::new( 1.0,  1.0,  1.0),
            Vertex::new( 1.0,  1.0, -1.0),
            Vertex::new( 1.0, -1.0, -1.0),
            Vertex::new( 1.0, -1.0, -1.0),
            Vertex::new( 1.0, -1.0,  1.0),
            Vertex::new( 1.0,  1.0,  1.0),

            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new( 1.0, -1.0, -1.0),
            Vertex::new( 1.0, -1.0,  1.0),
            Vertex::new( 1.0, -1.0,  1.0),
            Vertex::new(-1.0, -1.0,  1.0),
            Vertex::new(-1.0, -1.0, -1.0),

            Vertex::new(-1.0,  1.0, -1.0),
            Vertex::new( 1.0,  1.0, -1.0),
            Vertex::new( 1.0,  1.0,  1.0),
            Vertex::new( 1.0,  1.0,  1.0),
            Vertex::new(-1.0,  1.0,  1.0),
            Vertex::new(-1.0,  1.0, -1.0),
        ];

        let vtxbuf = try!(VertexBuffer::new(facade, vertices));
        let idxbuf = NoIndices(PrimitiveType::TrianglesList);

        Ok(Light {
            vtxbuf: vtxbuf,
            idxbuf: idxbuf,
            position: params.position(),
            scale: params.scale(),
            color: params.color(),
            ambient: params.ambient(),
            diffuse: params.diffuse(),
            specular: params.specular(),
        })
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_position(&mut self, v: Vec3) {
        self.position = v;
    }

    pub fn color(&self) -> Vec3 {
        self.color
    }

    pub fn set_color(&mut self, v: Vec3) {
        self.color = v;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn ambient(&self) -> Vec3 {
        self.ambient
    }

    pub fn diffuse(&self) -> Vec3 {
        self.diffuse
    }

    pub fn specular(&self) -> Vec3 {
        self.specular
    }

    pub fn model(&self) -> Mat4 {
        let mut model = Mat4::translation(self.position);
        model.scale_mut(Vec3::repeat(self.scale));
        model
    }

    pub fn draw<S, U>(&self,
                      surface: &mut S,
                      program: &Program,
                      uniforms: &U) -> Result<()>
        where S: Surface,
              U: Uniforms,
    {
        let ref draw_params = DrawParameters {
            depth_test: DepthTest::IfLessOrEqual,
            depth_write: true,
            ..Default::default()
        };
        try!(surface.draw(&self.vtxbuf, self.idxbuf, program, uniforms, draw_params));
        Ok(())
    }
}

