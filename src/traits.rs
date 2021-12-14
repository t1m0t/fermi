use crate::{Atom, AtomBuilder, AtomFamily, AtomFamilyBuilder, AtomId, AtomRoot};
use im_rc::HashMap as ImMap;

pub trait Readable<V> {
    fn read(&self, root: AtomRoot) -> Option<V>;
    fn init(&self) -> V;
    fn unique_id(&self) -> AtomId;
}

impl<V> Readable<V> for Atom<V> {
    fn read(&self, _root: AtomRoot) -> Option<V> {
        todo!()
    }
    fn init(&self) -> V {
        (*self)(AtomBuilder)
    }
    fn unique_id(&self) -> AtomId {
        *self as *const ()
    }
}

impl<K, V> Readable<ImMap<K, V>> for AtomFamily<K, V> {
    fn read(&self, _root: AtomRoot) -> Option<ImMap<K, V>> {
        todo!()
    }

    fn init(&self) -> ImMap<K, V> {
        (*self)(AtomFamilyBuilder)
    }
    fn unique_id(&self) -> AtomId {
        *self as *const ()
    }
}

pub trait Writable<V>: Readable<V> {
    fn write(&self, root: AtomRoot, value: V);
}

impl<V> Writable<V> for Atom<V> {
    fn write(&self, _root: AtomRoot, _value: V) {
        todo!()
    }
}

impl<K, V> Writable<ImMap<K, V>> for AtomFamily<K, V> {
    fn write(&self, _root: AtomRoot, _value: ImMap<K, V>) {
        todo!()
    }
}
