use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use dioxus::core::ScopeId;
use im_rc::HashSet;

use crate::Readable;

pub type AtomId = *const ();

pub struct AtomRoot {
    pub atoms: RefCell<HashMap<AtomId, Slot>>,
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
            atoms: RefCell::new(HashMap::new()),
        }
    }

    pub fn register<V: 'static>(&self, f: impl Readable<V>, scope: ScopeId) -> Rc<V> {
        log::trace!("registering atom {:?}", f.unique_id());

        let mut atoms = self.atoms.borrow_mut();

        if let Some(slot) = atoms.get_mut(&f.unique_id()) {
            slot.subscribers.insert(scope);
            slot.value.clone().downcast().unwrap()
        } else {
            let value = Rc::new(f.init());
            let mut subscribers = HashSet::new();
            subscribers.insert(scope);

            atoms.insert(
                f.unique_id(),
                Slot {
                    value: value.clone(),
                    subscribers,
                },
            );
            value
        }
    }

    pub fn set<V: 'static>(&self, ptr: AtomId, value: V) {
        let mut atoms = self.atoms.borrow_mut();

        if let Some(slot) = atoms.get_mut(&ptr) {
            slot.value = Rc::new(value);
            log::trace!("found item with subscribers {:?}", slot.subscribers);

            for sub in &slot.subscribers {
                log::trace!("updating subcsriber");
                (self.update_any)(*sub);
            }
        } else {
            log::trace!("no atoms found for {:?}", ptr);
        }
    }

    pub fn unsubscribe(&self, ptr: AtomId, scope: ScopeId) {
        let mut atoms = self.atoms.borrow_mut();

        if let Some(slot) = atoms.get_mut(&ptr) {
            slot.subscribers.remove(&scope);
        }
    }

    pub fn read<V>(&self, _f: impl Readable<V>) -> &V {
        todo!()
    }
}
