#![doc = include_str!("../README.md")]

/// This trait is used to assert that [`Type<T>`](Self::Type)
/// satisfies the constraints on it in an `Assert` trait bound.
///
/// See the [crate documentation](crate) for examples.
pub trait Assert {
    /// The type to be evaluated, which is identical to the type `T`.
    ///
    /// This type is always defined as `T` by the blanket
    /// implementation of this trait for any type.
    type Type<T: ?Sized>: ?Sized;
}

impl<T: ?Sized> Assert for T {
    type Type<U: ?Sized> = U;
}

/// A type asserter.
///
/// This type is used for introducing `Assert` trait bounds.
/// `Assert` trait bounds can be written on it.
///
/// # Examples
///
/// ```no_run
/// use type_identical::*;
///
/// fn foo<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
/// {
/// }
/// ```
///
/// However, `Assert` trait bounds can also be written on other types,
/// as the following shows.
///
/// ```no_run
/// use type_identical::*;
///
/// fn foo<T: ?Sized, U: ?Sized>()
/// where
///     (): Assert<Type<T> = U>,
/// {
/// }
/// ```
#[derive(Debug)]
pub struct Assertion;

/// This trait denotes that `Self` is identical to `T`.
///
/// See the [crate documentation](crate) for examples.
///
/// # Explainer of the `Identical` trait bound
///
/// * `U: Identical<T>` cannot make `U` and `T` identical
///   in the type system. They are still two different
///   types in the type system. `Identical<T>` is a trait bound that
///   requires `U` to be identical to `T`.
/// * `U: Identical<T>` can help to infer `T` from `U`, but cannot
///   help to infer `U` from `T`. `T: Identical<U>` can be
///   added to help to infer `U` from `T`.
pub trait Identical<T: ?Sized>: Assert<Type<T> = Self> {}

impl<T: ?Sized> Identical<T> for T {}

mod tests;
