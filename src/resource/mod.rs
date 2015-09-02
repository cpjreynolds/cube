pub mod shader;

use std::hash::Hash;

pub trait Manager<K>
    where K: Hash + Eq
{
    type Managing: Resource;

    fn store(&mut self, Self::Managing);
    fn load<'a>(&'a mut self, K) -> &'a Self::Managing;
}

pub trait Resource {
}
