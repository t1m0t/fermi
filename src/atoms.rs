use im_rc::HashMap as ImMap;

pub type Atom<T> = fn(AtomBuilder) -> T;
pub type AtomFamily<K, V> = fn(AtomFamilyBuilder) -> ImMap<K, V>;

pub struct AtomBuilder;
pub struct AtomFamilyBuilder;

mod tests {
    use super::*;

    const Name: Atom<&'static str> = |_| "name";
    const Users: AtomFamily<&'static str, String> = |_| Default::default();
}
