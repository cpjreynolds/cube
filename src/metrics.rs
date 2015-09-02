use std::io::prelude::*;

use time;
use term::{
    self,
    StdoutTerminal,
};

use errors::{
    Result,
    Error,
};

#[derive(Debug, Clone)]
pub struct Metrics {
    spf: f64, // seconds per frame.
    frames: usize,
    last: f64,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            spf: 0.0,
            frames: 0,
            last: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frames += 1;
        let now = time::precise_time_s();
        let elapsed = now - self.last;
        if elapsed >= 1.0 {
            self.spf = elapsed / self.frames as f64;
            self.frames = 0;
            self.last = now;
        }
    }

    pub fn spf(&self) -> f64 {
        self.spf
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.spf
    }
}

pub struct MetricsDisplay {
    inner: Box<StdoutTerminal>,
}

impl MetricsDisplay {
    pub fn new() -> Result<MetricsDisplay> {
        term::stdout().and_then(|term| {
            Some(MetricsDisplay {
                inner: term,
            })
        }).ok_or(Error::new("could not open terminal for writing"))
    }

    pub fn display(&mut self, metrics: &Metrics) -> Result<()> {
        try!(self.inner.carriage_return());
        let ms_per_frame = format!("MS/F:\t{:2.2}        \n", metrics.spf() * 1000.0);
        try!(self.inner.write_all(ms_per_frame.as_ref()));

        try!(self.inner.carriage_return());
        let frames_per_s = format!("FPS:\t{:2.2}        ", metrics.fps());
        try!(self.inner.write_all(frames_per_s.as_ref()));
        try!(self.inner.cursor_up());
        Ok(())
    }
}
