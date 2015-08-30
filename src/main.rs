#![feature(box_syntax)]

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate time;
extern crate gel;
extern crate num;
extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

use gel::{
    Mat4,
    Vec3,
    Translate,
    Scale,
    Rotate,
    Projection,
    PI,
    PI_2,
};

use glium::Program;

use glium::{
    Display,
    DisplayBuild,
    Surface,
    DepthTest,
};
use glium::texture::SrgbTexture2d;
use glium::uniforms::{
    SamplerWrapFunction,
    MinifySamplerFilter,
    MagnifySamplerFilter,
};

use num::One;

use glutin::{
    WindowBuilder,
    Event,
    VirtualKeyCode,
    ElementState,
};

mod shader;
mod cube;
mod wave;

use cube::Cube;
use wave::Wave;
use shader::Manager;

fn main() {
    let display: Display = WindowBuilder::new()
                                         .with_depth_buffer(24)
                                         .build_glium()
                                         .unwrap();

    let container_img = image::open("container.jpg").unwrap();
    let texture = SrgbTexture2d::new(&display, container_img).unwrap();

    let mut cube = Cube::new(&display, 0.5).unwrap();
    cube.draw_params().depth_test = DepthTest::IfLessOrEqual;
    cube.draw_params().depth_write = true;

    let mut rng = rand::thread_rng();

    let sinwave = Wave::default();
    let coswave = wave::Builder::new();
    let angle_wave = wave::Builder::new().amplitude(PI).period(5.0).build();

    let mut shaders = Manager::new();

    let mut vert_src_file = File::open("cube.vert").unwrap();
    let mut vert_src = String::new();
    vert_src_file.read_to_string(&mut vert_src).unwrap();
    shaders.store("cube.vert", vert_src);

    let mut frag_src_file = File::open("cube.frag").unwrap();
    let mut frag_src = String::new();
    frag_src_file.read_to_string(&mut frag_src).unwrap();
    shaders.store("cube.frag", frag_src);

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.8, 0.8, 1.0, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect = width as f32 / height as f32;

        let model = Mat4::rotation(PI / 4.0, Vec3::new(0.0, 1.0, 0.0));
        let view = Mat4::translation(Vec3::new(0.0, 0.0, -5.0));
        let projection = Mat4::perspective(70.0, aspect, 0.1, 100.0);

        let uniforms = uniform! {
            model: model,
            view: view,
            projection: projection,
            tex: texture.sampled()
                        .wrap_function(SamplerWrapFunction::Mirror)
                        .minify_filter(MinifySamplerFilter::LinearMipmapLinear)
                        .magnify_filter(MagnifySamplerFilter::Linear),
        };
        let program = shaders.load(&display, "cube.vert", "cube.frag").unwrap();
        cube.draw(&mut target, program, &uniforms).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => break 'main,
                Event::KeyboardInput( ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => {
                    break 'main
                },
                _ => {},
            }
        }
    }
}

