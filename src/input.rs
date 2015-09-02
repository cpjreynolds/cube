use std::collections::hash_map::{
    HashMap,
};

use glutin::{
    Event,
};
use glium::backend::glutin_backend::{
    GlutinFacade,
    PollEventsIter,
    WinRef,
};

pub use glutin::{
    ElementState,
    VirtualKeyCode as Key,
};

use errors::{
    Result,
    Error,
};
use cursor::Cursor;

pub struct Input<'a> {
    keys: HashMap<Key, ElementState>,
    cursor: Cursor<'a>,
    winsize: (i32, i32),
    should_close: bool,
}

impl<'a> Input<'a> {
    pub fn new(winref: WinRef) -> Result<Input> {
        let (x, y) = try!(winref.get_inner_size_pixels()
                          .ok_or(Error::with_detail("window error",
                                                    "could not get window size")));
        let (x, y) = (x as i32, y as i32);
        Ok(Input {
            keys: HashMap::new(),
            cursor: try!(Cursor::new(winref, x / 2, y / 2)),
            winsize: (x as i32, y as i32),
            should_close: false,
        })
    }

    pub fn update(&mut self, events: PollEventsIter) {
        for event in events {
            if is_event_should_close(&event) {
                self.should_close = true;
            }

            match event {
                Event::KeyboardInput(state, _, Some(key)) => {
                    self.keys.insert(key, state);
                },
                Event::MouseMoved((xpos, ypos)) => {
                    self.cursor.update(xpos, ypos);
                },
                Event::Resized(x, y) => {
                    let (x, y) = (x as i32, y as i32);
                    self.cursor.update_center(x / 2, y / 2);
                    self.winsize = (x as i32, y as i32);
                },
                _ => {},
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        if let Some(state) = self.keys.get(&key) {
            *state == ElementState::Pressed
        } else {
            false
        }
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
}

fn is_event_should_close(event: &Event) -> bool {
    match *event {
        Event::Closed => true,
        Event::KeyboardInput(ElementState::Pressed, _, Some(Key::Q)) => true,
        _ => false,
    }
}

