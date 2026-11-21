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

mod private_identical {
    //! The trait [`PrivateIdentical`].

    /// A trait that cannot be used as trait or supertrait bounds
    /// in any `dyn` types, at least outside this module.
    ///
    /// This trait is not for types but for other traits,
    /// forbidding its subtraits to be used in `dyn` types.
    trait NonDynBound {
        /// An associated type that forbids this trait
        /// to be used as trait or supertrait bounds in any `dyn` types,
        /// at least outside this module.
        ///
        /// # Approaches to achieving the forbidding
        ///
        /// This type achieves the forbidding through
        /// the following rules:
        ///
        /// 1. This type is **not** dyn compatible, so that this
        ///    trait is **not** dyn compatible.
        /// 2. This type is private, so that `dyn` types cannot specify
        ///    this associated type outside this module.
        ///    Because `dyn` types have to specify their associated types,
        ///    `dyn` types cannot use this trait outside this module.
        ///
        /// As long as any one of the above rules holds, the forbidding
        /// can be achieved. Otherwise, this trait should be fixed.
        ///
        /// # No equality constraints in trait bounds
        ///
        /// This type should not have an equality constraint (i.e., `= M`)
        /// when this trait is used as a trait bound, in order to ensure
        /// that this type is not specified by a subtrait of this trait,
        /// so that the rule (2) can work when it holds.
        type NonDynBound<U: ?Sized>: ?Sized;
    }

    impl<T: ?Sized> NonDynBound for T {
        type NonDynBound<U: ?Sized> = U;
    }

    /// A private version of [`Identical`].
    ///
    /// [`Identical`]: super::Identical
    ///
    /// This trait is used for:
    ///
    /// * Sealing [`Identical`], forbidding any extensions of
    ///   the implementations of [`Identical`] outside this [`crate`].
    /// * Forbidding [`Identical`] to be used in `dyn` types
    ///   at least outside this [`crate`].
    ///
    /// # Explainer of forbidding the uses in `dyn` types
    ///
    /// Some type like `dyn PrivateIdentical<()>` implements
    /// `PrivateIdentical<()>`, meaning that the type
    /// `dyn PrivateIdentical<()>`
    /// is identical to the type `()`, which is a conflict.
    /// Therefore, this trait should **not** be used in `dyn` types,
    /// and **neither** should [`Identical`].
    #[allow(private_bounds, reason = "NonDynBound")]
    pub trait PrivateIdentical<T: ?Sized>: NonDynBound {}

    impl<T: ?Sized> PrivateIdentical<T> for T {}
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
pub trait Identical<T: ?Sized>: private_identical::PrivateIdentical<T> {}

impl<T: ?Sized> Identical<T> for T {}

mod tests;
