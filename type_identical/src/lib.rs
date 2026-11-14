#![doc = include_str!("../README.md")]
#![no_std]

/// This trait is used for asserting that [`Type<T>`](Self::Type)
/// satisfies the constraints on it in an `Assert` trait bound.
///
/// The main use of `Assert` is to assert that a type `T` is identical
/// to another type `U`, i.e., `Assert<Type<T> = U>`.
/// The trait [`Identical`] is a special kind of `Assert` for that purpose.
///
/// See the [crate documentation](crate) and [`Assertion`] for examples.
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

/// An asserter.
///
/// This type is used for adding [`Assert`] trait bounds in where-clauses.
/// [`Assert`] trait bounds can be written on it.
///
/// # Examples
///
/// In the following example, an [`Assert`] trait bound is written on
/// `Assertion` in the where-clause, asserting that the type `T` is
/// identical to the type `U`.
///
/// ```no_run
/// use type_identical::*;
///
/// fn foo<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
/// {
/// }
///
/// foo::<i32, _>();
/// foo::<_, i32>();
/// ```
///
/// However, [`Assert`] trait bounds can also be written on other types,
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
///
/// foo::<i32, _>();
/// foo::<_, i32>();
/// ```
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

impl<T, U> Identical<U> for T
where
    T: ?Sized + Assert<Type<U> = T>,
    U: ?Sized,
{
}

mod tests;
