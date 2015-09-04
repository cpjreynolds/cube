use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use errors::Result;

pub fn load<P>(path: P) -> Result<String>
    where P: AsRef<Path>
{
    let mut file = try!(File::open(path));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf));
    Ok(buf)
}
