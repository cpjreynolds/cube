use glutin::{
    CursorState,
};
use glium::backend::glutin_backend::WinRef;

use errors::{
    Result,
    Error,
};

pub struct Cursor<'a> {
    deltax: i32,
    deltay: i32,
    center: (i32, i32),
    winref: WinRef<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(winref: WinRef, xcenter: i32, ycenter: i32) -> Result<Cursor> {
        try!(winref.set_cursor_position(xcenter, ycenter));

        Ok(Cursor {
            deltax: 0,
            deltay: 0,
            center: (xcenter, ycenter),
            winref: winref,
        })
    }

    pub fn update(&mut self, xpos: i32, ypos: i32) -> Result<()> {
        let (xc, yc) = self.center;

        self.deltax = xpos - xc;
        self.deltay = yc - ypos;

        try!(self.winref.set_cursor_position(xc, yc));
        Ok(())
    }

    pub fn update_center(&mut self, xcenter: i32, ycenter: i32) {
        self.center = (xcenter, ycenter);
    }

    pub fn get_delta(&self) -> (i32, i32) {
        (self.deltax, self.deltay)
    }
}
