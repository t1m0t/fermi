use std::{any::Any, collections::HashMap, rc::Rc};

use dioxus::core::ScopeId;
use im_rc::HashSet;

use crate::Readable;

pub type AtomId = *const ();

pub struct AtomRoot {
    pub atoms: HashMap<AtomId, Slot>,
    pub update_any: Rc<dyn Fn(ScopeId)>,
}

pub struct Slot {
    pub value: Rc<dyn Any>,
    pub subscribers: HashSet<ScopeId>,
}

impl AtomRoot {
    pub fn new(update_any: Rc<dyn Fn(ScopeId)>) -> Self {
        Self {
            update_any,
            atoms: HashMap::new(),
        }
    }

    pub fn register<V: 'static>(&mut self, f: impl Readable<V>, scope: ScopeId) -> Rc<V> {
        if let Some(slot) = self.atoms.get_mut(&f.unique_id()) {
            slot.subscribers.insert(scope);
            slot.value.clone().downcast().unwrap()
        } else {
            let value = Rc::new(f.init());
            self.atoms.insert(
                f.unique_id(),
                Slot {
                    value: value.clone(),
                    subscribers: HashSet::new(),
                },
            );
            value
        }
    }

    pub fn set<V: 'static>(&mut self, ptr: AtomId, value: V) {
        if let Some(slot) = self.atoms.get_mut(&ptr) {
            slot.value = Rc::new(value);
            for sub in &slot.subscribers {
                (self.update_any)(*sub);
            }
        }
    }

    pub fn unsubscribe(&mut self, ptr: AtomId, scope: ScopeId) {
        if let Some(slot) = self.atoms.get_mut(&ptr) {
            slot.subscribers.remove(&scope);
        }
    }

    pub fn read<V>(&self, f: impl Readable<V>) -> &V {
        todo!()
    }
}
