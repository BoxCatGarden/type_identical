//! Tests.
#![cfg(any(test, doctest))]

/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized>(m: &T) -> &<T as TypeItself>::Itself {
///     m
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Identical<U>, U: ?Sized>(m: &U) -> &T::Itself {
///     m
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Identical<U>, U: ?Sized>() {}
///
/// bbb::<i32, _>();
/// bbb::<[i32], _>();
/// bbb::<dyn Send, _>();
/// ```
///
/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Identical<U>, U: ?Sized>() {}
///
/// bbb::<i32, ()>();
/// ```
///
/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + TypeItself<Itself = U>, U: ?Sized>() {}
///
/// bbb::<i32, ()>();
/// ```
#[cfg(doctest)]
#[allow(non_camel_case_types)]
struct _test_type_itself;
