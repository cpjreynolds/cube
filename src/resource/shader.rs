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
use std::fmt::{
    self,
    Formatter,
    Debug,
};

use glium::program::Program;
use glium::backend::Facade;

use errors::{
    Result,
    Error,
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

    pub fn load<F, Q: ?Sized>(&mut self, facade: &F, vert: &Q, frag: &Q) -> Result<&Program>
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
                let vsrc = try!(self.sources.get(vert)
                                .ok_or(Error::new("missing vertex shader")));
                let fsrc = try!(self.sources.get(frag)
                                .ok_or(Error::new("missing fragment shader")));

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

