use std::borrow::Borrow;
use std::cell::{
    RefCell,
    Ref,
};
use std::ops::Deref;
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

    pub fn compile<F, Q: ?Sized>(&mut self, facade: &F, vert: &Q, frag: &Q) -> Result<()>
        where F: Facade,
              K: Borrow<Q>,
              Q: Hash + Eq
    {
        let vsrc = try!(self.sources.get(vert).ok_or(Error::new("missing vertex shader")));
        let fsrc = try!(self.sources.get(frag).ok_or(Error::new("missing fragment shader")));

        let prog_key = {
            let mut hasher = SipHasher::new();
            vert.hash(&mut hasher);
            frag.hash(&mut hasher);
            hasher.finish()
        };

        let program = try!(Program::from_source(facade, vsrc, fsrc, None));
        self.programs.insert(prog_key, program);
        Ok(())
    }

    pub fn load<Q: ?Sized>(&self, vert: &Q, frag: &Q) -> Result<&Program>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        let prog_key = {
            let mut hasher = SipHasher::new();
            vert.hash(&mut hasher);
            frag.hash(&mut hasher);
            hasher.finish()
        };

        if let Some(program) = self.programs.get(&prog_key) {
            Ok(program)
        } else {
            Err(Error::new("requested program does not exist"))
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

