# atoms two

A retry of atoms

The idea is that heavily nested data should not be modified. Try to keep your state as flat as possible. You will be a happier camper.


```rust
static UserId: Atom<Option<Uuid>> = || None;

static Items: FC<()> = |cx, props| {
    let id = use_read(cx, UserId);

    cx.render(rsx!(
        div { }
    ))
};

```
