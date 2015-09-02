use std::error;
use std::result;
use std::fmt::{
    self,
    Display,
    Formatter,
};
use std::io;
use std::env;

use image;
use glium;
use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String,
    detail: Option<String>,
    inner: Option<Box<error::Error>>,
}

impl Error {
    pub fn new<M>(msg: M) -> Error
        where M: fmt::Display
    {
        Error {
            msg: msg.to_string(),
            detail: None,
            inner: None,
        }
    }

    pub fn with_detail<M, D>(msg: M, detail: D) -> Error
        where M: Display,
              D: Display,
    {
        Error {
            msg: msg.to_string(),
            detail: Some(detail.to_string()),
            inner: None,
        }
    }

    pub fn from_error<E>(err: E) -> Error
        where E: Into<Box<error::Error>>
    {
        let err = err.into();
        Error {
            msg: String::from(err.description()),
            detail: Some(format!("{}", err)),
            inner: Some(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(writeln!(fmt, "{}", self.msg));
        if let Some(ref detail) = self.detail {
            try!(writeln!(fmt, "\t{}", detail));
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&error::Error> {
        if let Some(ref err) = self.inner {
            Some(&**err)
        } else {
            None
        }
    }
}

macro_rules! from_error_impl {
    ($t:ty) => {
        impl From<$t> for Error {
            fn from(e: $t) -> Error {
                Error::from_error(e)
            }
        }
    };
}

from_error_impl!(io::Error);
from_error_impl!(toml::DecodeError);
from_error_impl!(env::VarError);
from_error_impl!(image::ImageError);
from_error_impl!(glium::ProgramCreationError);

impl<T> From<glium::GliumCreationError<T>> for Error
    where T: error::Error + 'static
{
    fn from(e: glium::GliumCreationError<T>) -> Error {
        Error::from_error(e)
    }
}

impl From<glium::texture::TextureCreationError> for Error {
    fn from(e: glium::texture::TextureCreationError) -> Error {
        use glium::texture::TextureCreationError;
        Error::with_detail(
            "texture creation error",
            match e {
                TextureCreationError::FormatNotSupported => "format not supported",
                TextureCreationError::DimensionsNotSupported => "dimensions not supported",
                TextureCreationError::TypeNotSupported => "type not supported",
            })
    }
}

impl From<glium::SwapBuffersError> for Error {
    fn from(e: glium::SwapBuffersError) -> Error {
        use glium::SwapBuffersError;
        Error::with_detail(
            "swap buffers error",
            match e {
                SwapBuffersError::ContextLost => "context lost",
                SwapBuffersError::AlreadySwapped => "already swapped",
            })
    }
}

impl From<glium::DrawError> for Error {
    fn from(e: glium::DrawError) -> Error {
        Error::with_detail("draw error", e)
    }
}

impl From<glium::vertex::BufferCreationError> for Error {
    fn from(e: glium::vertex::BufferCreationError) -> Error {
        Error::with_detail("buffer creation error", format!("{:#?}", e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::with_detail("error", e)
    }
}

impl From<()> for Error {
    fn from(e: ()) -> Error {
        Error::new("unknown error")
    }
}
