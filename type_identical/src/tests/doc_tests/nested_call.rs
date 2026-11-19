//! Tests of the effect of trait bounds on nested function calls.

/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Identical<U>, U: ?Sized>() {
///     aaa::<T, _>();
///     aaa::<T, U>();
///     aaa::<T, T>();
///     aaa::<U, _>();
///     aaa::<U, U>();
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Identical<U> + Identical<W>, U: ?Sized, W: ?Sized>() {
///     aaa::<T, U, W>();
///     aaa::<T, W, U>();
///     aaa::<T, U, T>();
///     aaa::<T, W, T>();
///     aaa::<T, T, U>();
///     aaa::<T, T, W>();
///     aaa::<T, T, T>();
///     aaa::<U, _, _>();
///     aaa::<U, U, U>();
///     aaa::<W, _, _>();
///     aaa::<W, W, W>();
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Assert<Type<U> = T, Type<T> = T>, U: ?Sized>() {
///     aaa::<T, T>();
///     aaa::<T, U>();
///     aaa::<U, _>();
///     aaa::<U, U>();
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
/// {
///     aaa::<T, _>();
///     aaa::<T, U>();
///     aaa::<U, _>();
///     aaa::<U, U>();
/// }
/// ```
#[allow(non_camel_case_types)]
struct _test_nested_call;
