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

use wave::{
    Wave,
};
use errors::{
    Result,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    fn new(x: f32, y: f32, z: f32, tx: f32, ty: f32) -> Vertex {
        Vertex {
            position: [x, y ,z],
            tex_coords: [tx, ty],
        } }
}

pub struct Cube<'a> {
    vtxbuf: VertexBuffer<Vertex>,
    idxbuf: NoIndices,
    draw_params: DrawParameters<'a>,
    texture: SrgbTexture2d,
    scale: f32,
    translation: Vec3,
    axis: Vec3,
    wave: Wave,
}

impl<'a> Cube<'a> {
    pub fn new<F>(facade: &F, texture: SrgbTexture2d) -> Result<Cube<'a>>
        where F: Facade
    {
        let ref vertices = [
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 0.0),
            Vertex::new( 1.0, -1.0, -1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0, -1.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0, -1.0, 1.0, 1.0),
            Vertex::new(-1.0,  1.0, -1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0),
            Vertex::new( 1.0, -1.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 1.0),
            Vertex::new(-1.0,  1.0,  1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0),
            Vertex::new(-1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new(-1.0,  1.0, -1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0),
            Vertex::new(-1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0, -1.0, 1.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0,  1.0, 0.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 1.0, 1.0),
            Vertex::new( 1.0, -1.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0, -1.0,  1.0, 1.0, 0.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 1.0),
            Vertex::new(-1.0,  1.0, -1.0, 0.0, 1.0),
            Vertex::new( 1.0,  1.0, -1.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0),
            Vertex::new(-1.0,  1.0,  1.0, 0.0, 0.0),
            Vertex::new(-1.0,  1.0, -1.0, 0.0, 1.0),
        ];

        let vtxbuf = try!(VertexBuffer::new(facade, vertices));
        let idxbuf = NoIndices(PrimitiveType::TrianglesList);

        Ok(Cube {
            vtxbuf: vtxbuf,
            idxbuf: idxbuf,
            draw_params: DrawParameters {
                depth_test: DepthTest::IfLessOrEqual,
                depth_write: true,
                .. Default::default()
            },
            texture: texture,
            scale: f32::one(),
            translation: Vec3::zero(),
            axis: Vec3::zero(),
            wave: Wave::default(),
        })
    }

    pub fn sampler(&self) -> Sampler<SrgbTexture2d> {
        self.texture.sampled()
            .wrap_function(SamplerWrapFunction::Mirror)
            .minify_filter(MinifySamplerFilter::LinearMipmapLinear)
            .magnify_filter(MagnifySamplerFilter::Linear)
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn set_translation(&mut self, v: Vec3) -> &mut Self {
        self.translation = v;
        self
    }

    pub fn set_axis(&mut self, v: Vec3) -> &mut Self {
        self.axis = v;
        self
    }

    pub fn wave_mut(&mut self) -> &mut Wave {
        &mut self.wave
    }

    pub fn model(&self) -> Mat4 {
        let mut model = Mat4::translation(self.translation);
        model.rotate_mut(self.wave.eval(), self.axis);
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
        try!(surface.draw(&self.vtxbuf, self.idxbuf, program, uniforms, &self.draw_params));
        Ok(())
    }
}

