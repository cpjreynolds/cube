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
use resource::shader::{
    Manager,
};
use config::Config;
use process::Process;
use camera::Camera;
use input::{
    Input,
    Key,
};
use delta::Delta;
use player::Player;
use light::Light;
use errors::{
    Result,
    Error,
};

mod cube;
mod resource;
mod camera;
mod config;
mod errors;
mod process;
mod delta;
mod input;
mod player;
mod cursor;
mod light;
mod file;

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
    {
        // Clear sceen to something while loading.
        let mut target = display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);
        target.clear_depth(1.0);
        try!(target.finish());
    }

    let player_params = config.player();
    let diffuse_map_path = config.paths().assets().join(player_params.diffuse_map());
    let diffuse_map_img = try!(image::open(diffuse_map_path));
    let diffuse_map = try!(SrgbTexture2d::new(&display, diffuse_map_img));
    let specular_map_path = config.paths().assets().join(player_params.specular_map());
    let specular_map_img = try!(image::open(specular_map_path));
    let specular_map = try!(SrgbTexture2d::new(&display, specular_map_img));
    let mut player: Player = try!(Player::new(&display,
                                              diffuse_map,
                                              specular_map,
                                              player_params.shine(),
                                              player_params));

    let mut shaders = Manager::new();

    let vert_src = try!(file::load(config.paths().shaders().join("cube.vert")));
    shaders.store("cube.vert", vert_src);

    let frag_src = try!(file::load(config.paths().shaders().join("cube.frag")));
    shaders.store("cube.frag", frag_src);

    try!(shaders.compile(&display, "cube.vert", "cube.frag"));

    let light_vert_src = try!(file::load(config.paths().shaders().join("light.vert")));
    shaders.store("light.vert", light_vert_src);

    let light_frag_src = try!(file::load(config.paths().shaders().join("light.frag")));
    shaders.store("light.frag", light_frag_src);

    try!(shaders.compile(&display, "light.vert", "light.frag"));

    let winref = try!(display.get_window()
                      .ok_or(Error::with_detail("window error",
                                                "failed to get window reference")));

    let mut light: Light = try!(Light::new(&display, config.light()));

    let mut input = try!(Input::new(winref));

    let mut camera = Camera::new(config.camera());

    let mut delta = Delta::new();

    let player_program = try!(shaders.load("cube.vert", "cube.frag"));
    let light_program = try!(shaders.load("light.vert", "light.frag"));

    'main: loop {
        let dtime = delta.update();

        let events = display.poll_events();
        input.update(events);
        if input.should_close() {
            break 'main;
        }
        if input.is_pressed(Key::Comma) {
            player.forward(dtime);
        }
        if input.is_pressed(Key::O) {
            player.backward(dtime);
        }
        if input.is_pressed(Key::A) {
            player.left(dtime);
        }
        if input.is_pressed(Key::E) {
            player.right(dtime);
        }
        if input.is_pressed(Key::Space) {
            player.up(dtime);
        }
        if input.is_pressed(Key::LControl) {
            player.down(dtime);
        }

        let (dx, dy) = input.cursor().get_delta();
        camera.update(dx, dy);

        let mut target = display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect = width as f32 / height as f32;


        let projection = Mat4::perspective(config.projection().fov(),
                                           aspect,
                                           config.projection().znear(),
                                           config.projection().zfar());

        let view = camera.look_at(player.position());

        {
            let uniforms = uniform! {
                model: player.model(),
                view: view,
                projection: projection,

                diffuse_map: player.diffuse_map(),
                specular_map: player.specular_map(),

                light_pos: light.position(),
                light_color: light.color(),
                light_ambient: light.ambient(),
                light_diffuse: light.diffuse(),
                light_specular: light.specular(),

                shine: player.shine(),
            };
            try!(player.draw(&mut target, player_program, &uniforms));
        }

        {
            let uniforms = uniform! {
                model: light.model(),
                view: view,
                projection: projection,
            };
            try!(light.draw(&mut target, light_program, &uniforms));
        }

        try!(target.finish());
    }
    Ok(())
}

