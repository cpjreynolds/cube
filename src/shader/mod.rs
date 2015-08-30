use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::error::Error;
use std::path::Path;
use std::borrow::Borrow;
use std::collections::hash_map::{
    HashMap,
    Entry,
};
use std::hash::{
    Hash,
    Hasher,
    SipHasher,
};

use glium::program::{
    ProgramCreationError,
    Program,
};
use glium::backend::Facade;
use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};

pub struct Manager<K> {
    sources: HashMap<K, String>,
    programs: HashMap<u64, Program>,
}

impl<K> Manager<K>
    where K: Hash + Eq
{
    pub fn new() -> Manager<K> {
        Manager {
            sources: HashMap::new(),
            programs: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: K, source: String) -> Option<String> {
        self.sources.insert(key, source)
    }

    pub fn load<F, Q: ?Sized>(&mut self, facade: &F, vert: &Q, frag: &Q) -> Result<&Program, LoadError>
        where F: Facade,
              K: Borrow<Q>,
              Q: Hash + Eq
    {
        let prog_key = {
            let mut hasher = SipHasher::new();
            vert.hash(&mut hasher);
            frag.hash(&mut hasher);
            hasher.finish()
        };

        match self.programs.entry(prog_key) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            },
            Entry::Vacant(entry) => {
                let vsrc = try!(self.sources.get(vert).ok_or(LoadError::MissingShader));
                let fsrc = try!(self.sources.get(frag).ok_or(LoadError::MissingShader));

                let program = try!(Program::from_source(facade, vsrc, fsrc, None));
                Ok(entry.insert(program))
            },
        }
    }
}

impl<K> Debug for Manager<K>
    where K: Eq + Hash + Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Manager")
            .field("sources", &self.sources)
            .field("programs", &self.programs)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum LoadError {
    MissingShader,
    Creation(ProgramCreationError),
}

impl Error for LoadError {
    fn description(&self) -> &str {
        match *self {
            LoadError::MissingShader => "missing shader",
            LoadError::Creation(..) => "program creation error",
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadError::MissingShader => {
                write!(f, "{}", self.description())
            },
            LoadError::Creation(ref err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl From<ProgramCreationError> for LoadError {
    fn from(e: ProgramCreationError) -> LoadError {
        LoadError::Creation(e)
    }
}

