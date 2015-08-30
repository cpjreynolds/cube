use glium::backend::Facade;
use glium::vertex::{
    VertexBuffer,
    BufferCreationError,
};
use glium::{
    Surface,
    Program,
    DrawParameters,
    DrawError,
};
use glium::uniforms::Uniforms;
use glium::index::{
    NoIndices,
    PrimitiveType,
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
}

impl<'a> Cube<'a> {
    pub fn new<F>(facade: &F, size: f32) -> Result<Cube<'a>, BufferCreationError>
        where F: Facade
    {
        let ref vertices = [
            Vertex::new(-size, -size, -size, 0.0, 0.0),
            Vertex::new( size, -size, -size, 1.0, 0.0),
            Vertex::new( size,  size, -size, 1.0, 1.0),
            Vertex::new( size,  size, -size, 1.0, 1.0),
            Vertex::new(-size,  size, -size, 0.0, 1.0),
            Vertex::new(-size, -size, -size, 0.0, 0.0),
            Vertex::new(-size, -size,  size, 0.0, 0.0),
            Vertex::new( size, -size,  size, 1.0, 0.0),
            Vertex::new( size,  size,  size, 1.0, 1.0),
            Vertex::new( size,  size,  size, 1.0, 1.0),
            Vertex::new(-size,  size,  size, 0.0, 1.0),
            Vertex::new(-size, -size,  size, 0.0, 0.0),
            Vertex::new(-size,  size,  size, 1.0, 0.0),
            Vertex::new(-size,  size, -size, 1.0, 1.0),
            Vertex::new(-size, -size, -size, 0.0, 1.0),
            Vertex::new(-size, -size, -size, 0.0, 1.0),
            Vertex::new(-size, -size,  size, 0.0, 0.0),
            Vertex::new(-size,  size,  size, 1.0, 0.0),
            Vertex::new( size,  size,  size, 1.0, 0.0),
            Vertex::new( size,  size, -size, 1.0, 1.0),
            Vertex::new( size, -size, -size, 0.0, 1.0),
            Vertex::new( size, -size, -size, 0.0, 1.0),
            Vertex::new( size, -size,  size, 0.0, 0.0),
            Vertex::new( size,  size,  size, 1.0, 0.0),
            Vertex::new(-size, -size, -size, 0.0, 1.0),
            Vertex::new( size, -size, -size, 1.0, 1.0),
            Vertex::new( size, -size,  size, 1.0, 0.0),
            Vertex::new( size, -size,  size, 1.0, 0.0),
            Vertex::new(-size, -size,  size, 0.0, 0.0),
            Vertex::new(-size, -size, -size, 0.0, 1.0),
            Vertex::new(-size,  size, -size, 0.0, 1.0),
            Vertex::new( size,  size, -size, 1.0, 1.0),
            Vertex::new( size,  size,  size, 1.0, 0.0),
            Vertex::new( size,  size,  size, 1.0, 0.0),
            Vertex::new(-size,  size,  size, 0.0, 0.0),
            Vertex::new(-size,  size, -size, 0.0, 1.0),
        ];

        let vtxbuf = try!(VertexBuffer::new(facade, vertices));
        let idxbuf = NoIndices(PrimitiveType::TrianglesList);

        Ok(Cube {
            vtxbuf: vtxbuf,
            idxbuf: idxbuf,
            draw_params: Default::default(),
        })
    }

    pub fn draw_params(&mut self) -> &mut DrawParameters<'a> {
        &mut self.draw_params
    }

    pub fn draw<S, U>(&self,
                      surface: &mut S,
                      program: &Program,
                      uniforms: &U) -> Result<(), DrawError>
        where S: Surface,
              U: Uniforms,
    {
        surface.draw(&self.vtxbuf, self.idxbuf, program, uniforms, &self.draw_params)
    }
}

