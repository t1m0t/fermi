# Fermi: A global state management solution for Dioxus, inspired by Recoil.Js


Fermi provides primitives for managing global state in Dioxus applications. Heavily inspired by Recoil.Js, Fermi uses the concepts of `atoms` and `selectors` to easily refactor local state into global state.

```rust

// First, we define an atom of our global state
static Name: Atom<String> = |_| "Dioxus".to_string();

fn app(cx: Scope) -> Element {      
    // then we read it
    let name = use_read(&cx, Name);

    cx.render(rsx!{
        h1 { "Hello, {name}"}
    })
}

fn Child(cx: Scope) -> Element {
    // and then we can read or write it from anywhere in our app
    let set_name = use_set(&cx, Name);

    cx.render(rsx!{
        button { 
            onclick: move |_| set_name(random_name()),
            "Click to set a random name"
        }
    })
}
```

Fermi is currently under construction, so you have to use the `master` branch to get started.

```rust
[depdencies]
fermi = { git = "https://github.com/dioxuslabs/fermi" }
```


Broadly our feature set to required to be released includes:
- [x] Support for Atoms
- [ ] Support for Atom Families
- [ ] Support for memoized Selectors
- [ ] Support for memoized SelectorFamilies
- [ ] Support for UseFermiCallback for access to fermi from async 
