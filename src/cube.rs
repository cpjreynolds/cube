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
};
use glium::index::{
    NoIndices,
    PrimitiveType,
};
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

use errors::{
    Result,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
    fn new(x: f32, y: f32, z: f32, nx: f32, ny: f32, nz: f32, tx: f32, ty: f32) -> Vertex {
        Vertex {
            position: [x, y ,z],
            normal: [nx, ny, nz],
            tex_coords: [tx, ty],
        } }
}

pub struct Cube {
    vtxbuf: VertexBuffer<Vertex>,
    idxbuf: NoIndices,
    scale: f32,
    position: Vec3,
}

impl Cube {
    pub fn new<F>(facade: &F) -> Result<Cube>
        where F: Facade
    {
        let ref vertices = [
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new( 1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0),
            Vertex::new(-1.0,  1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0),

            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0),
            Vertex::new( 1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 1.0, 1.0),
            Vertex::new(-1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0),

            Vertex::new(-1.0,  1.0,  1.0, -1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new(-1.0,  1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(-1.0, -1.0,  1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(-1.0,  1.0,  1.0, -1.0, 0.0, 0.0, 1.0, 0.0),

            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0,  1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 1.0, 0.0, 0.0, 1.0, 0.0),

            Vertex::new(-1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0),
            Vertex::new( 1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0),
            Vertex::new( 1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 1.0, 0.0),
            Vertex::new( 1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 1.0, 0.0),
            Vertex::new(-1.0, -1.0,  1.0, 0.0, -1.0, 0.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0),

            Vertex::new(-1.0,  1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0),
            Vertex::new( 1.0,  1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0),
            Vertex::new( 1.0,  1.0,  1.0, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::new( 1.0,  1.0,  1.0, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::new(-1.0,  1.0,  1.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Vertex::new(-1.0,  1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0),
        ];

        let vtxbuf = try!(VertexBuffer::new(facade, vertices));
        let idxbuf = NoIndices(PrimitiveType::TrianglesList);

        Ok(Cube {
            vtxbuf: vtxbuf,
            idxbuf: idxbuf,
            scale: f32::one(),
            position: Vec3::zero(),
        })
    }


    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_position(&mut self, v: Vec3) {
        self.position = v;
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn position(&self) -> Vec3 {
        self.position
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

