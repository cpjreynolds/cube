#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(const_fn)]
#![feature(result_expect)]

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate time;
extern crate gel;
extern crate num;
extern crate image;
extern crate rand;
extern crate toml;
extern crate rustc_serialize;
extern crate term;

use std::fs::File;
use std::io::prelude::*;
use std::env;

use num::One;
use gel::{
    Mat4,
    Vec3,
    Translate,
    Scale,
    Rotate,
    Repeat,
    Projection,
};
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
use glutin::{
    WindowBuilder,
    GlProfile,
    Event,
    VirtualKeyCode,
    ElementState,
};

use cube::Cube;
use wave::WaveBuilder;
use resource::shader::{
    Manager,
};
use metrics::{
    Metrics,
    MetricsDisplay,
};
use config::Config;
use process::Process;
use camera::Camera;
use input::{
    Input,
    Key,
};
use delta::Delta;
use errors::{
    Result,
    Error,
};

mod cube;
mod wave;
mod resource;
mod camera;
mod config;
mod metrics;
mod errors;
mod process;
mod delta;
mod input;
mod cursor;

fn main() {
    let process = Process::new(execute);
    process.execute().handle();
}

fn execute() -> Result<()> {
    let config_path = try!(env::var("CUBE_CONF"));
    let config: Config = try!(Config::new(&config_path));
    println!("{:#?}", config);

    let window_builder = config.window().to_window_builder();
    let display = try!(window_builder.build_glium());

    let mut cubes: Vec<Cube> = Vec::new();
    for cparams in config.cubes().iter() {
        let tex_img_path = config.paths().assets().join(cparams.texture());
        let img = try!(image::open(tex_img_path));
        let texture = try!(SrgbTexture2d::new(&display, img));

        let mut cube: Cube = try!(Cube::new(&display, texture));
        cube.set_translation(cparams.translation())
            .set_axis(cparams.axis())
            .set_scale(cparams.scale());
        cube.wave_mut()
            .set_amplitude(cparams.amplitude())
            .set_period(cparams.period())
            .set_pshift(cparams.pshift())
            .set_vshift(cparams.vshift());

        cubes.push(cube);
    }

    let mut shaders = Manager::new();

    let mut vert_src_file = try!(File::open(config.paths().shaders().join("cube.vert")));
    let mut vert_src = String::new();
    try!(vert_src_file.read_to_string(&mut vert_src));
    shaders.store("cube.vert", vert_src);

    let mut frag_src_file = try!(File::open(config.paths().shaders().join("cube.frag")));
    let mut frag_src = String::new();
    try!(frag_src_file.read_to_string(&mut frag_src));
    shaders.store("cube.frag", frag_src);

    let program = try!(shaders.load(&display, "cube.vert", "cube.frag"));

    let mut metrics = Metrics::new();
    let mut mdisplay = try!(MetricsDisplay::new());

    let winref = try!(display.get_window()
                      .ok_or(Error::with_detail("window error",
                                                "failed to get window reference")));
    let mut input = try!(Input::new(winref));

    let mut camera = Camera::new(config.camera().position(),
                                 config.camera().target());
    camera.set_speed(config.camera().speed());
    camera.set_sensitivity(config.camera().sensitivity());

    let mut delta = Delta::new();

    'main: loop {
        metrics.update();
        try!(mdisplay.display(&metrics));

        let dtime = delta.update();

        let events = display.poll_events();
        input.update(events);
        if input.should_close() {
            break 'main;
        }
        if input.is_pressed(Key::Comma) {
            camera.forward(dtime);
        }
        if input.is_pressed(Key::O) {
            camera.backward(dtime);
        }
        if input.is_pressed(Key::A) {
            camera.left(dtime);
        }
        if input.is_pressed(Key::E) {
            camera.right(dtime);
        }
        if input.is_pressed(Key::Space) {
            camera.up(dtime);
        }
        if input.is_pressed(Key::LControl) {
            camera.down(dtime);
        }

        let (dx, dy) = input.cursor().get_delta();
        println!("{} {}", dx, dy);
        camera.update_target(dx, dy);

        let mut target = display.draw();
        target.clear_color(0.8, 0.8, 1.0, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect = width as f32 / height as f32;


        let projection = Mat4::perspective(config.projection().fov(),
                                           aspect,
                                           config.projection().znear(),
                                           config.projection().zfar());



        for cube in cubes.iter() {
            let model = cube.model();

            let uniforms = uniform! {
                model: model,
                view: camera.to_matrix(),
                projection: projection,
                tex: cube.sampler(),
            };

            try!(cube.draw(&mut target, program, &uniforms));
        }
        try!(target.finish());
    }
    Ok(())
}

