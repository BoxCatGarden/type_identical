This crate contains traits for type identity.

# Examples of the use of trait `Identical<T>`

Move the binding of a type parameter from the type to
an associated function, as the `Foo::new` shows.

```rust, no_run
use std::marker::PhantomData;
use type_identical::Identical;

trait Trait<T: ?Sized> {}

struct Foo<T, U: ?Sized>(T, PhantomData<fn() -> *const U>);

impl<T: Trait<U>, U: ?Sized> Foo<T, U> {
    pub fn new<W>(t: T) -> Self
    where
        W: ?Sized + Identical<U>,
        U: Identical<W>,
    {
        Foo(t, PhantomData)
    }
}

struct SingleImpl;

impl Trait<i32> for SingleImpl {}

struct MultipleImpl;

impl<T: ?Sized> Trait<T> for MultipleImpl {}

fn foo() {
    let _ = Foo::new(SingleImpl);
    let _ = Foo::new::<i32>(MultipleImpl);
    let _ = Foo::new::<u64>(MultipleImpl);
}
```
