#![doc = include_str!("../README.md")]

/// This trait denotes a type that has an associated type
/// [`Itself`](Self::Itself).
///
/// 'TypeItself' stands for the associated type '`type Itself`'.
pub trait TypeItself {
    /// The implementing type itself, i.e., `Self`.
    ///
    /// The blanket implementation of this trait makes
    /// this type always identical to `Self`.
    type Itself: ?Sized + TypeItself<Itself = Self>;
}

impl<T: ?Sized> TypeItself for T {
    type Itself = T;
}

/// This trait denotes that `Self` is identical to `T`.
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
pub trait Identical<T: ?Sized>: TypeItself<Itself = T> {}

impl<T: ?Sized> Identical<T> for T {}

mod tests;
