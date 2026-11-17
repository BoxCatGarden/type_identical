#![doc = include_str!("../README.md")]
#![no_std]

/// This trait is used for asserting that [`Type<T>`](Self::Type)
/// satisfies the constraints on it in an `Assert` trait bound.
///
/// The main use of `Assert` is to assert that a type `T` is identical
/// to another type `U`, i.e., `Assert<Type<T> = U>`.
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

/// The sealed version of [`Identical`].
///
/// This trait is the underlying trait of [`Identical`].
/// This trait is used for:
///
/// * Freezing the implementation of [`Identical`].
/// * Forbidding [`Identical`] to be used in `dyn` types.
trait SealedIdentical<T: ?Sized> {
    /// An associated type that forbidding this trait
    /// to be used as trait or supertrait bounds in any `dyn` types,
    /// at least outside this [`crate`].
    ///
    /// # Approaches to achieving the forbidding
    ///
    /// This type achieves the forbidding through
    /// the following rules:
    ///
    /// * This type is **not** dyn compatible, so that this
    ///   trait is **not** dyn compatible.
    /// * This type is sealed, so that `dyn` types cannot specify
    ///   this associated type outside this [`crate`].
    ///   Because `dyn` types have to specify their associated types,
    ///   `dyn` types cannot use this trait outside this [`crate`].
    ///
    /// As long as any one of the above rules hold, the forbidding
    /// can be achieved. Otherwise, this trait should be fixed.
    ///
    /// # Explainer of the forbidding
    ///
    /// Some type like `dyn SealedIdentical<()>` implements
    /// `SealedIdentical<()>`, meaning that the type `dyn SealedIdentical<()>`
    /// is identical to the type `()`, which is a conflict.
    /// Therefore, this trait should **not** be used in `dyn` types,
    /// and **neither** should [`Identical`].
    type NonDynBound<U: ?Sized>: ?Sized;
}

impl<T: ?Sized> SealedIdentical<T> for T {
    type NonDynBound<U: ?Sized> = U;
}

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
#[allow(private_bounds)]
pub trait Identical<T: ?Sized>:
    // According to `SealedIdentical::NonDynBound`,
    // any associated type of `SealedIdentical` should not be
    // specified here because of the approaches to achieving
    // the forbidding (see `SealedIdentical::NonDynBound`).
    SealedIdentical<T>
{
}

impl<T: ?Sized> Identical<T> for T {}

mod tests;
