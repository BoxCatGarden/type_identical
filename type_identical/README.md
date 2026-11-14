This crate contains traits for asserting that:

* a type is **identical** to another type; or in other words,
* a type is **equal** to another type; or in other words,
* a type is the **same** as another type.

The main use of such assertions is to help to infer generic
types `T` from types identical to `T` or some type like `Bar<T>`.

# Examples

The trait [`Identical<T>`](crate::Identical) can be used to
move the binding of a type parameter from the type to
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

fn foo<T: ?Sized>() {
    let _ = Foo::new(SingleImpl);
    let _ = Foo::new::<i32>(MultipleImpl);
    let _ = Foo::new::<dyn Send>(MultipleImpl);
    let _ = Foo::new::<T>(MultipleImpl);
}
```

`Foo::new` can also be implemented with the trait
[`Assert`](crate::Assert), as the following shows.

```rust, no_run
use type_identical::{Assert, Assertion};

impl<T: Trait<U>, U: ?Sized> Foo<T, U> {
    pub fn new<W: ?Sized>(t: T) -> Self
    where
        Assertion: Assert<Type<W> = U>,
    {
        Foo(t, PhantomData)
    }
}

fn foo<T: ?Sized>() {
    let _ = Foo::new(SingleImpl);
    let _ = Foo::new::<i32>(MultipleImpl);
    let _ = Foo::new::<dyn Send>(MultipleImpl);
    let _ = Foo::new::<T>(MultipleImpl);
}

use std::marker::PhantomData;

trait Trait<T: ?Sized> {}

struct Foo<T, U: ?Sized>(T, PhantomData<fn() -> *const U>);

struct SingleImpl;

impl Trait<i32> for SingleImpl {}

struct MultipleImpl;

impl<T: ?Sized> Trait<T> for MultipleImpl {}
```
