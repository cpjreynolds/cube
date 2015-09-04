use std::path::{
    PathBuf,
    Path,
};
use std::io::prelude::*;
use std::fs::File;

use toml::{
    self,
    Parser,
    Decoder,
    Value,
};
use rustc_serialize::Decodable;
use glutin::WindowBuilder;
use gel::{
    self,
    Vec3,
    Repeat,
};
use num::{
    One,
    Zero,
};

use errors::{
    Result,
    Error,
};

#[derive(Debug, Clone)]
pub struct Config {
    paths: Paths,
    window: WindowParams,
    projection: ProjectionParams,
    camera: CameraParams,
    player: PlayerParams,
    light: LightParams,
}

impl Config {
    pub fn new<P>(path: P) -> Result<Config>
        where P: AsRef<Path>
    {
        let mut cfile = try!(File::open(&path));
        let mut buf = String::new();
        try!(cfile.read_to_string(&mut buf));

        let table = try!(parse(&buf, path.as_ref()));
        let mut decoder = Decoder::new(Value::Table(table));

        let toml_config = try!(TomlConfig::decode(&mut decoder));
        let config: Config = toml_config.into();
        Ok(config)
    }

    pub fn paths(&self) -> &Paths {
        &self.paths
    }

    pub fn window(&self) -> &WindowParams {
        &self.window
    }

    pub fn projection(&self) -> &ProjectionParams {
        &self.projection
    }

    pub fn camera(&self) -> &CameraParams {
        &self.camera
    }

    pub fn player(&self) -> &PlayerParams {
        &self.player
    }

    pub fn light(&self) -> &LightParams {
        &self.light
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    root: PathBuf,
    assets: PathBuf,
    shaders: PathBuf,
}

impl Paths {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn assets(&self) -> &Path {
        &self.assets
    }

    pub fn shaders(&self) -> &Path {
        &self.shaders
    }
}

#[derive(Debug, Clone)]
pub struct WindowParams {
    multisampling: u16,
    vsync: bool,
    depth_bits: Option<u8>,
    stencil_bits: Option<u8>,
    color_bits: Option<u8>,
    alpha_bits: Option<u8>,
    srgb: Option<bool>,
    transparent: bool,
}

impl WindowParams {
    pub fn to_window_builder(&self) -> WindowBuilder<'static> {
        let mut builder = WindowBuilder::new();

        builder = builder.with_multisampling(self.multisampling);
        if self.vsync {
            builder = builder.with_vsync();
        }
        if let Some(db) = self.depth_bits {
            builder = builder.with_depth_buffer(db);
        }
        if let Some(sb) = self.stencil_bits {
            builder = builder.with_stencil_buffer(sb);
        }
        if let (Some(cb), Some(ab)) = (self.color_bits, self.alpha_bits) {
            builder = builder.with_pixel_format(cb, ab);
        }
        builder = builder.with_srgb(self.srgb);
        builder = builder.with_transparency(self.transparent);

        builder
    }
}

#[derive(Debug, Clone)]
pub struct PlayerParams {
    diffuse_map: PathBuf,
    specular_map: PathBuf,
    shine: f32,
    scale: f32,
    speed: f32,
}

impl PlayerParams {
    pub fn diffuse_map(&self) -> &Path {
        &self.diffuse_map
    }

    pub fn specular_map(&self) -> &Path {
        &self.specular_map
    }

    pub fn shine(&self) -> f32 {
        self.shine
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

#[derive(Debug, Clone)]
pub struct LightParams {
    scale: f32,
    position: Vec3,
    color: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl LightParams {
    pub fn color(&self) -> Vec3 {
        self.color
    }

    pub fn scale(&self) -> f32 {
        self.scale
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
}

#[derive(Debug, Clone)]
pub struct CameraParams {
    sensitivity: f32,
    radius: f32,
}

impl CameraParams {
    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

#[derive(Debug, Clone)]
pub struct ProjectionParams {
    fov: f32,
    znear: f32,
    zfar: f32,
}

impl ProjectionParams {
    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn znear(&self) -> f32 {
        self.znear
    }

    pub fn zfar(&self) -> f32 {
        self.zfar
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlConfig {
    paths: TomlPaths,
    window: TomlWindowParams,
    projection: TomlProjectionParams,
    camera: TomlCameraParams,
    player: TomlPlayerParams,
    light: TomlLightParams,
}

impl Into<Config> for TomlConfig {
    fn into(self) -> Config {
        Config {
            paths: self.paths.into(),
            window: self.window.into(),
            projection: self.projection.into(),
            camera: self.camera.into(),
            player: self.player.into(),
            light: self.light.into(),
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlPaths {
    root: String,
    assets: String,
    shaders: String,
}

impl Into<Paths> for TomlPaths {
    fn into(self) -> Paths {
        Paths {
            root: self.root.into(),
            assets: self.assets.into(),
            shaders: self.shaders.into(),
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlWindowParams {
    multisampling: Option<u16>,
    vsync: Option<bool>,
    depth_bits: Option<u8>,
    stencil_bits: Option<u8>,
    color_bits: Option<u8>,
    alpha_bits: Option<u8>,
    srgb: Option<bool>,
    transparent: Option<bool>,
}

impl Into<WindowParams> for TomlWindowParams {
    fn into(self) -> WindowParams {
        WindowParams {
            multisampling: self.multisampling.unwrap_or(0),
            vsync: self.vsync.unwrap_or(false),
            depth_bits: self.depth_bits,
            stencil_bits: self.stencil_bits,
            color_bits: self.color_bits,
            alpha_bits: self.alpha_bits,
            srgb: self.srgb,
            transparent: self.transparent.unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlPlayerParams {
    diffuse_map: String,
    specular_map: String,
    shine: f32,
    scale: f32,
    speed: f32,
}

impl Into<PlayerParams> for TomlPlayerParams {
    fn into(self) -> PlayerParams {
        PlayerParams {
            diffuse_map: self.diffuse_map.into(),
            specular_map: self.specular_map.into(),
            shine: self.shine,
            scale: self.scale,
            speed: self.speed,
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlLightParams {
    scale: f32,
    position: Vec3,
    color: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl Into<LightParams> for TomlLightParams {
    fn into(self) -> LightParams {
        LightParams {
            scale: self.scale,
            position: self.position,
            color: self.color,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlCameraParams {
    sensitivity: f32,
    radius: f32,
}

impl Into<CameraParams> for TomlCameraParams {
    fn into(self) -> CameraParams {
        CameraParams {
            sensitivity: self.sensitivity,
            radius: self.radius,
        }
    }
}

#[derive(Debug, Clone, RustcDecodable)]
struct TomlProjectionParams {
    fov: Option<f32>,
    znear: Option<f32>,
    zfar: Option<f32>,
}

impl Into<ProjectionParams> for TomlProjectionParams {
    fn into(self) -> ProjectionParams {
        ProjectionParams {
            fov: self.fov.unwrap_or(70.0),
            znear: self.znear.unwrap_or(0.1),
            zfar: self.zfar.unwrap_or(100.0),
        }
    }
}

fn parse(toml: &str, path: &Path) -> Result<toml::Table> {
    let mut parser = Parser::new(&toml);
    match parser.parse() {
        Some(toml) => return Ok(toml),
        None => {},
    }

    let mut error_str = String::new();
    for error in parser.errors.iter() {
        let (loline, locol) = parser.to_linecol(error.lo);
        let (hiline, hicol) = parser.to_linecol(error.hi);
        error_str.push_str(&format!("{}:{}:{}{} {}\n",
                                    path.display(),
                                    loline + 1, locol + 1,
                                    if loline != hiline || locol != hicol {
                                        format!("-{}:{}", hiline + 1, hicol + 1)
                                    } else {
                                        String::from("")
                                    },
                                    error.desc));
    }
    Err(Error::with_detail("parse error", error_str))
}
